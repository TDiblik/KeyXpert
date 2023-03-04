// Some unused assignments matter to C and WinApi
// Rust correctlly shows warnings, however, in this context, those warnings are redundant and annoying.
#![allow(unused_assignments)]

pub mod models;
pub mod utils;

use models::LastSentRemapInfo;
use utils::is_sys_key;

use anyhow::{anyhow, Result};
use winapi::shared::minwindef::WPARAM;
use winapi::um::winuser::{GetAsyncKeyState, VK_RMENU};
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
// this allowes us to find REMAPPED_KEYS and SYS_KEYS in constant time (and search on stack instead of heap).
static mut REMAPPED_KEYS: [Option<u8>; 256] = [None; 256];
include!(concat!(env!("OUT_DIR"), "/GENERATED_SYS_KEYS.rs"));

static mut WINDOW_HHOOK: *mut HHOOK__ = std::ptr::null_mut();
static mut ENABLE_RECURSIVE_REMAPPING: bool = false;

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

        // b => a
        // REMAPPED_KEYS[66] = Some(65);

        // alt => ctrl
        REMAPPED_KEYS[164] = Some(162);

        // CAPS_LOCK => BACKSPACE
        REMAPPED_KEYS[20] = Some(8);
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

static mut LAST_SENT_REMAP_INFO: Option<LastSentRemapInfo> = None;
unsafe extern "system" fn remap_keys_callback(
    n_code: i32,
    mut w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    // Guard clause, don't do anything unless n_code is telling us to.
    if n_code != HC_ACTION {
        call_next_hook!(n_code, w_param, l_param);
    }

    // It's faster to call next hook right away, because we don't work with any other events
    let w_param_u32 = w_param as u32;
    if w_param_u32 != WM_SYSKEYUP
        && w_param_u32 != WM_KEYUP
        && w_param_u32 != WM_SYSKEYDOWN
        && w_param_u32 != WM_KEYDOWN
    {
        call_next_hook!(n_code, w_param, l_param);
    }

    // Remap shorcuts
    let kbd_struct = &*(l_param as *const KBDLLHOOKSTRUCT);
    let trigger_key = kbd_struct.vkCode as u8;

    if (w_param_u32 == WM_KEYDOWN || w_param_u32 == WM_SYSKEYDOWN)
        && trigger_key == b'8'
        && GetAsyncKeyState(VK_RMENU) < 0
    {
        keybd_trigger_key_down!(VK_RMENU, map_virtual_key!(VK_RMENU));
        keybd_trigger_key_down!(0x37, map_virtual_key!(0x37));
        keybd_trigger_key_up!(0x37, map_virtual_key!(0x37));
        keybd_trigger_key_up!(VK_RMENU, map_virtual_key!(VK_RMENU));

        keybd_trigger_key_down!(VK_RMENU, map_virtual_key!(VK_RMENU));

        event_handled!();
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
