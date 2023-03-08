// Some unused assignments matter to C and WinApi
// Rust correctlly shows warnings, however, in this context, those warnings are redundant and annoying.
#![allow(unused_assignments)]

pub mod models;
pub mod utils;

use models::{LastSentRemapInfo, RemappedShortcut};
use utils::is_sys_key;

use anyhow::{anyhow, Result};
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

fn main() -> Result<()> {
    // TODO: Read settings

    // Setup remappings
    unsafe {
        // + => é
        REMAPPED_KEYS[49] = Some(48);
        // é => ě
        REMAPPED_KEYS[48] = Some(50);
        // š => alt
        REMAPPED_KEYS[51] = Some(164);
        // a => ctrl
        REMAPPED_KEYS[65] = Some(162);
        // o => a
        REMAPPED_KEYS[0x4F] = Some(0x41);
        // alt => ctrl
        REMAPPED_KEYS[164] = Some(162);
        // CAPS_LOCK => BACKSPACE
        REMAPPED_KEYS[20] = Some(8);

        // Win + P => Win + O
        REMAPPED_SHORTCUTS_CONTAIN_KEY[0x50] = true;
        REMAPPED_SHORTCUTS.push(RemappedShortcut::new(
            [VK_LWIN as u8, 0, 0, 0],
            0x50,
            [VK_LWIN as u8, 0, 0, 0],
            0x4F,
        ));

        // ALTGr + 8 => ALTGr + 7
        REMAPPED_SHORTCUTS_CONTAIN_KEY[0x38] = true;
        REMAPPED_SHORTCUTS.push(RemappedShortcut::new(
            [VK_RMENU as u8, 0, 0, 0],
            0x38,
            [VK_RMENU as u8, 0, 0, 0],
            0x37,
        ));

        // Win + Ctrl + P => Win + I
        REMAPPED_SHORTCUTS_CONTAIN_KEY[0x50] = true;
        REMAPPED_SHORTCUTS.push(RemappedShortcut::new(
            [VK_LWIN as u8, VK_LCONTROL as u8, 0, 0],
            0x50,
            [VK_LWIN as u8, 0, 0, 0],
            0x49,
        ));

        // Win + B => Win + I
        REMAPPED_SHORTCUTS_CONTAIN_KEY[0x42] = true;
        REMAPPED_SHORTCUTS.push(RemappedShortcut::new(
            [VK_LWIN as u8, 0, 0, 0],
            0x42,
            [VK_LWIN as u8, 0, 0, 0],
            0x49,
        ));

        log_debug!("{:?}", REMAPPED_SHORTCUTS);
    }

    // Setup settings, currentlly only recursive remapping
    unsafe {
        ENABLE_RECURSIVE_REMAPPING = false;
    }

    // Start listening to keyboard
    unsafe {
        let h_mod = GetModuleHandleW(std::ptr::null());

        WINDOW_HHOOK = SetWindowsHookExW(WH_KEYBOARD_LL, Some(remap_keys_callback), h_mod, 0);
        if WINDOW_HHOOK.is_null() {
            return Err(anyhow!("Failed to set WINDOWS_HOOK."));
        }

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) != 0 {
            // TODO: Are these calls usefull? They have been in the original example,
            // however I am sceptical that they do anything or even get called, since
            // the program behaves exactly the same without them.
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        std::process::exit(msg.wParam as i32);
    }
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
