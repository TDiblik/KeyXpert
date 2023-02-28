// Some unused assignments matter to C and WinApi
// Rust correctlly shows warnings, however, in this context, those warnings are redundant and annoying.
#![allow(unused_assignments)]

pub mod utils;
use utils::{is_sys_key, LastSentRemapInfo};

use anyhow::{anyhow, Result};
use winapi::{
    shared::{
        minwindef::{LPARAM, LRESULT, WPARAM},
        windef::HHOOK__,
    },
    um::{
        libloaderapi::GetModuleHandleW,
        winuser::{
            keybd_event, CallNextHookEx, DispatchMessageW, GetMessageW, MapVirtualKeyW,
            SetWindowsHookExW, TranslateMessage, HC_ACTION, KBDLLHOOKSTRUCT, KEYEVENTF_EXTENDEDKEY,
            KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
            WM_SYSKEYDOWN, WM_SYSKEYUP,
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
        REMAPPED_KEYS[66] = Some(65);
        // alt => ctrl
        REMAPPED_KEYS[164] = Some(162);
    }

    // Setup settings, currentlly only recursive remapping
    unsafe {
        ENABLE_RECURSIVE_REMAPPING = false;
    }

    // Start listening to keyboard
    unsafe {
        // Get the handle to the current module
        let h_mod = GetModuleHandleW(std::ptr::null());

        WINDOW_HHOOK = SetWindowsHookExW(WH_KEYBOARD_LL, Some(remap_keys_callback), h_mod, 0);
        if WINDOW_HHOOK.is_null() {
            return Err(anyhow!("Failed to set WINDOWS_HOOK."));
        }

        let messages: *mut MSG = std::ptr::null_mut();
        while GetMessageW(messages, std::ptr::null_mut(), 0, 0) != 0 {
            TranslateMessage(messages);
            DispatchMessageW(messages);
        }

        std::process::exit((*messages).wParam as i32);
    }
}

static mut LAST_SENT_REMAP_INFO: Option<LastSentRemapInfo> = None;
unsafe extern "system" fn remap_keys_callback(
    n_code: i32,
    mut w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code != HC_ACTION {
        call_next_hook!(n_code, w_param, l_param);
    }

    let kbd_struct = &*(l_param as *const KBDLLHOOKSTRUCT);

    let w_param_u32 = w_param as u32;
    let trigger_key = kbd_struct.vkCode as usize;
    let trigger_key_u8 = trigger_key as u8;

    // Prevent recursive remaping
    if !ENABLE_RECURSIVE_REMAPPING && LAST_SENT_REMAP_INFO.is_some() {
        let last_sent_remap = LAST_SENT_REMAP_INFO.as_ref().unwrap();

        // Neccessary for syskey to stay down
        if last_sent_remap.sender_key == trigger_key_u8 && w_param_u32 == WM_SYSKEYDOWN {
            event_handled!();
        }

        // Send up commands to remapped keys
        if last_sent_remap.sender_key == trigger_key_u8
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

        if last_sent_remap.remap_key == trigger_key_u8 {
            call_next_hook!(n_code, w_param, l_param);
        }
    }

    // Remap chars
    let possibly_remapped_key_in_range: Option<&Option<u8>> = REMAPPED_KEYS.get(trigger_key);
    if possibly_remapped_key_in_range.is_none() {
        call_next_hook!(n_code, w_param, l_param);
    }

    let possibly_remapped_key: &Option<u8> = possibly_remapped_key_in_range.unwrap();
    if possibly_remapped_key.is_none() {
        call_next_hook!(n_code, w_param, l_param);
    }

    let remmaped_key: u8 = possibly_remapped_key.unwrap();
    let scan_code = map_virtual_key!(remmaped_key);

    // hodling SYSKEY || normal buttons down || pressed syskey
    if w_param_u32 == WM_SYSKEYDOWN
        || (w_param_u32 == WM_KEYDOWN && !is_sys_key(trigger_key_u8) && !is_sys_key(remmaped_key))
        || (w_param_u32 == WM_KEYDOWN && is_sys_key(remmaped_key) && LAST_SENT_REMAP_INFO.is_none())
    {
        log_debug!("keydown event fired");

        LAST_SENT_REMAP_INFO = Some(LastSentRemapInfo {
            sender_key: trigger_key_u8,
            remap_key: remmaped_key,
        });
        keybd_trigger_key_down!(remmaped_key, scan_code);

        event_handled!();
    }

    // This will get triggered only when recursive remap is OFF
    if w_param_u32 == WM_KEYUP {
        log_debug!("keyup event fired");

        LAST_SENT_REMAP_INFO = None;
        keybd_trigger_key_up!(remmaped_key, scan_code);

        event_handled!();
    }

    // Neccessary for syskey to stay down
    w_param = WM_SYSKEYDOWN as usize;

    event_handled!();
}
