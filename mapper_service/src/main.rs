// Some unused assignments matter to C and WinApi
// Rust correctlly shows warnings, however, in this context, those warnings are redundant and annoying.
#![allow(unused_assignments)]

pub mod models;
pub mod utils;

use mapper_service::shared_constants::{self, log_error};
use models::{LastSentRemapInfo, RemappedShortcut};
use utils::is_sys_key;

use anyhow::anyhow;
use winapi::shared::minwindef::WPARAM;
use winapi::um::winuser::{
    GetAsyncKeyState, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_RCONTROL, VK_RMENU, VK_RSHIFT,
    VK_RWIN,
};
use winapi::{
    shared::minwindef::LPARAM,
    um::{
        winuser::{CallNextHookEx, GetMessageW, SetWindowsHookExW, MSG, WH_KEYBOARD_LL},
        winuser::{KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC},
    },
};
use winapi::{
    shared::{minwindef::LRESULT, windef::HHOOK__},
    um::{
        libloaderapi::GetModuleHandleW,
        winuser::{
            keybd_event, DispatchMessageW, MapVirtualKeyW, TranslateMessage, HC_ACTION,
            KBDLLHOOKSTRUCT, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
        },
    },
};

// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
// Basically, we have at max 255 keys. What we do with REMAPPED_KEYS and SYS_KEYS_TABLE is
// fill the arrays with None/false and then add Some/true to places where keys are supposed to be
// this allowes us to find REMAPPED_KEYS, SYS_KEYS and REMAPPED_SHORTCUTS_CONTAIN_KEY in constant time, search on stack instead of heap and most likely use array inside CPU cache (this depends on CPU tho).
static mut REMAPPED_KEYS: [Option<u8>; 256] = [None; 256];

static mut REMAPPED_SHORTCUTS: Vec<RemappedShortcut> = vec![];
static mut REMAPPED_SHORTCUTS_CONTAIN_KEY: [bool; 256] = [false; 256];

include!(concat!(env!("OUT_DIR"), "/GENERATED_SYS_KEYS.rs"));

static mut WINDOW_HHOOK: *mut HHOOK__ = std::ptr::null_mut();
static mut ENABLE_RECURSIVE_REMAPPING: bool = false;
static mut ENABLE_RECURSIVE_SHORTCUTS: bool = false;

fn main() -> anyhow::Result<()> {
    if let Err(err) = setup() {
        log_error(&err);
        return Err(err);
    }

    if let Err(err) = unsafe { program_loop() } {
        log_error(&err);
        return Err(err);
    }

    Ok(())
}

fn setup() -> anyhow::Result<()> {
    let Ok(service_config) = utils::get_service_config(shared_constants::service_config_file_path()) else {
        return Err(anyhow!("Unable to get/parse service config file."));
    };
    let Some(active_profile_id) = service_config.active_profile else {
        return Err(anyhow!("No profile active, shutting down."));
    };

    let Some(active_profile) = service_config.profiles.iter().find(|s| s.id == active_profile_id) else {
        return Err(
            anyhow!(
                format!("Profiles ({}) do not include active profile id ({})", 
                    service_config.profiles.iter().map(|s| s.id.to_string()).collect::<Vec<String>>().join(", "), 
                    active_profile_id
                )
            )
        );
    };

    for key_remap in active_profile.key_remaps.iter() {
        unsafe { REMAPPED_KEYS[key_remap.from as usize] = Some(key_remap.to) }
    }

    for shortcut_remap in active_profile.shortcut_remaps.iter() {
        unsafe {
            REMAPPED_SHORTCUTS_CONTAIN_KEY[shortcut_remap.from_shortcut_execution_key as usize] =
                true;
            REMAPPED_SHORTCUTS.push(RemappedShortcut::new(
                shortcut_remap.from_shortcut_holding_keys,
                shortcut_remap.from_shortcut_execution_key,
                shortcut_remap.to_shortcut_holding_keys,
                shortcut_remap.to_shortcut_execution_key,
            ));
        }
    }

    unsafe {
        ENABLE_RECURSIVE_REMAPPING = false;
    }

    Ok(())
}

unsafe fn program_loop() -> anyhow::Result<()> {
    let h_mod = GetModuleHandleW(std::ptr::null());

    WINDOW_HHOOK = SetWindowsHookExW(WH_KEYBOARD_LL, Some(remap_keys_callback), h_mod, 0);
    if WINDOW_HHOOK.is_null() {
        return Err(anyhow!("Failed to set WINDOWS_HOOK."));
    }

    let mut msg: MSG = std::mem::zeroed();
    while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) != 0 {
        // TODO: Are these calls usefull? They have been in the original example,
        // however I am sceptical that they do anything or even get called, since
        // the program behaves exactly the same without them. --- help needed from somebody who knows more about it.
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }

    std::process::exit(msg.wParam as i32);
}

// I could use kbd_struct.dwExtraInfo instead of static muts,
// however I currently like this approach more.
static mut LAST_SENT_REMAP_INFO: Option<LastSentRemapInfo> = None;
static mut STOP_RECURSIVE_SHORTCUT: bool = false;

unsafe extern "system" fn remap_keys_callback(
    n_code: i32,
    mut w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    // Guard clause, don't do anything unless n_code is telling us to.
    if n_code != HC_ACTION {
        call_next_hook!(n_code, w_param, l_param);
    }

    // Remap shorcuts
    let kbd_struct = &*(l_param as *const KBDLLHOOKSTRUCT);
    let w_param_u32 = w_param as u32;
    let trigger_key = kbd_struct.vkCode as u8;

    if !ENABLE_RECURSIVE_SHORTCUTS && STOP_RECURSIVE_SHORTCUT {
        call_next_hook!(n_code, w_param, l_param);
    }

    if (w_param_u32 == WM_KEYDOWN || w_param_u32 == WM_SYSKEYDOWN)
        && REMAPPED_SHORTCUTS_CONTAIN_KEY[trigger_key as usize]
    {
        let lwin_state = GetAsyncKeyState(VK_LWIN) < 0;
        let lctrl_state = GetAsyncKeyState(VK_LCONTROL) < 0;
        let lalt_state = GetAsyncKeyState(VK_LMENU) < 0;
        let lshift_state = GetAsyncKeyState(VK_LSHIFT) < 0;

        let rwin_state = GetAsyncKeyState(VK_RWIN) < 0;
        let rctrl_state = GetAsyncKeyState(VK_RCONTROL) < 0;
        let ralt_state = GetAsyncKeyState(VK_RMENU) < 0;
        let rshift_state = GetAsyncKeyState(VK_RSHIFT) < 0;

        if let Some(shortcut_info) = REMAPPED_SHORTCUTS.iter().find(|s| {
            s.get_from_execution_char() == trigger_key
                && s.mask_matches(
                    lwin_state,
                    lctrl_state,
                    lalt_state,
                    lshift_state,
                    rwin_state,
                    rctrl_state,
                    ralt_state,
                    rshift_state,
                )
        }) {
            STOP_RECURSIVE_SHORTCUT = true;

            let mut keys_to_leave_pressed = shortcut_info.get_from_shortcut();
            for possible_key in keys_to_leave_pressed.iter().rev() {
                let Some(key) = *possible_key else {
                    continue;
                };
                keybd_trigger_key_up!(key, map_virtual_key!(key));
            }

            let keys_to_press = shortcut_info.get_to_shortcut();
            for possible_key in keys_to_press {
                let Some(key) = possible_key else {
                    continue;
                };
                keybd_trigger_key_down!(key, map_virtual_key!(key));
            }
            for possible_key in keys_to_press.iter().rev() {
                let Some(key) = *possible_key else {
                    continue;
                };
                keybd_trigger_key_up!(key, map_virtual_key!(key));
            }

            keys_to_leave_pressed[4] = None;
            for possible_key in keys_to_leave_pressed {
                let Some(key) = possible_key else {
                    continue;
                };
                keybd_trigger_key_down!(key, map_virtual_key!(key));
            }

            STOP_RECURSIVE_SHORTCUT = false;
            event_handled!();
        }
    }

    // Prevent recursive character remaping
    if !ENABLE_RECURSIVE_REMAPPING && LAST_SENT_REMAP_INFO.is_some() {
        let last_sent_remap = LAST_SENT_REMAP_INFO.as_ref().unwrap();

        // Neccessary for syskey to stay down
        if last_sent_remap.sender_key == trigger_key && w_param_u32 == WM_SYSKEYDOWN {
            event_handled!();
        }

        if last_sent_remap.sender_key == trigger_key
            && (w_param_u32 == WM_SYSKEYUP || w_param_u32 == WM_KEYUP)
        {
            log_debug!("keyup recursive mapping event fired");

            keybd_trigger_key_up!(
                last_sent_remap.remap_key,
                map_virtual_key!(last_sent_remap.remap_key)
            );
            LAST_SENT_REMAP_INFO = None;

            event_handled!();
        }

        if last_sent_remap.remap_key == trigger_key {
            call_next_hook!(n_code, w_param, l_param);
        }
    }

    let remapped_key: u8 = match REMAPPED_KEYS.get(trigger_key as usize) {
        Some(Some(s)) => *s,
        _ => call_next_hook!(n_code, w_param, l_param),
    };
    let scan_code = map_virtual_key!(remapped_key);

    // holding SYSKEY || normal buttons down || pressed syskey
    if w_param_u32 == WM_SYSKEYDOWN
        || (w_param_u32 == WM_KEYDOWN && !is_sys_key(trigger_key) && !is_sys_key(remapped_key))
        || (w_param_u32 == WM_KEYDOWN && is_sys_key(remapped_key) && LAST_SENT_REMAP_INFO.is_none())
    {
        log_debug!("keydown event fired");

        LAST_SENT_REMAP_INFO = Some(LastSentRemapInfo {
            sender_key: trigger_key,
            remap_key: remapped_key,
        });
        keybd_trigger_key_down!(remapped_key, scan_code);

        event_handled!();
    }

    // This will get triggered only when recursive remap is ON
    if w_param_u32 == WM_KEYUP {
        log_debug!("keyup event fired");

        LAST_SENT_REMAP_INFO = None;
        keybd_trigger_key_up!(remapped_key, scan_code);

        event_handled!();
    }

    // Neccessary for syskey to stay down
    w_param = WM_SYSKEYDOWN as usize;

    event_handled!();
}
