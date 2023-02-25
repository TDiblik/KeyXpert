// Some unused assignments matter to C and WinApi
// Rust correctlly shows warnings, however, in this context, those warnings are redundant and annoying.
#![allow(unused_assignments)]

use anyhow::{anyhow, Result};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{LPARAM, LRESULT, WPARAM},
        windef::HHOOK__,
    },
    um::{
        libloaderapi::GetModuleHandleW,
        winuser::{
            keybd_event, CallNextHookEx, DispatchMessageW, GetAsyncKeyState, GetMessageW,
            MapVirtualKeyW, SetWindowsHookExW, TranslateMessage, HC_ACTION, KBDLLHOOKSTRUCT,
            KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC, MSG, VK_APPS, VK_ATTN,
            VK_BROWSER_BACK, VK_BROWSER_FAVORITES, VK_BROWSER_FORWARD, VK_BROWSER_HOME,
            VK_BROWSER_REFRESH, VK_BROWSER_SEARCH, VK_BROWSER_STOP, VK_CRSEL, VK_EREOF, VK_EXSEL,
            VK_ICO_00, VK_ICO_CLEAR, VK_ICO_HELP, VK_LAUNCH_APP1, VK_LAUNCH_APP2, VK_LAUNCH_MAIL,
            VK_LAUNCH_MEDIA_SELECT, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_MEDIA_NEXT_TRACK,
            VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK, VK_MEDIA_STOP, VK_NONAME, VK_NUMLOCK,
            VK_OEM_1, VK_OEM_102, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7,
            VK_OEM_8, VK_OEM_AX, VK_OEM_CLEAR, VK_OEM_COMMA, VK_OEM_FJ_JISHO, VK_OEM_FJ_LOYA,
            VK_OEM_FJ_MASSHOU, VK_OEM_FJ_ROYA, VK_OEM_FJ_TOUROKU, VK_OEM_MINUS, VK_OEM_PERIOD,
            VK_OEM_PLUS, VK_PA1, VK_PACKET, VK_PLAY, VK_PROCESSKEY, VK_RCONTROL, VK_RMENU,
            VK_RSHIFT, VK_RWIN, VK_SCROLL, VK_SLEEP, VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP,
            VK_ZOOM, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN,
        },
    },
};

// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
// Basically, we have at max 255 keys. What we do with REMAPPED_KEYS and SYS_KEYS_TABLE is
// fill the arrays with None/false and then add Some/true to places where keys are supposed to be
// this allowes us to find REMAPPED_KEYS and SYS_KEYS in constant time (and search on stack instead of heap).
static mut REMAPPED_KEYS: [Option<u8>; 255] = [None; 255];
static mut WINDOW_HHOOK: *mut HHOOK__ = std::ptr::null_mut();
static mut SYS_KEYS_TABLE: [bool; 255] = [false; 255];
const SYS_KEYS: [c_int; 63] = [
    VK_LSHIFT,
    VK_RSHIFT,
    VK_LCONTROL,
    VK_RCONTROL,
    VK_LMENU,
    VK_RMENU,
    VK_LWIN,
    VK_RWIN,
    VK_APPS,
    VK_SLEEP,
    VK_NUMLOCK,
    VK_SCROLL,
    VK_OEM_FJ_JISHO,
    VK_OEM_FJ_MASSHOU,
    VK_OEM_FJ_TOUROKU,
    VK_OEM_FJ_LOYA,
    VK_OEM_FJ_ROYA,
    VK_BROWSER_BACK,
    VK_BROWSER_FORWARD,
    VK_BROWSER_REFRESH,
    VK_BROWSER_STOP,
    VK_BROWSER_SEARCH,
    VK_BROWSER_FAVORITES,
    VK_BROWSER_HOME,
    VK_VOLUME_MUTE,
    VK_VOLUME_DOWN,
    VK_VOLUME_UP,
    VK_MEDIA_NEXT_TRACK,
    VK_MEDIA_PREV_TRACK,
    VK_MEDIA_STOP,
    VK_MEDIA_PLAY_PAUSE,
    VK_LAUNCH_MAIL,
    VK_LAUNCH_MEDIA_SELECT,
    VK_LAUNCH_APP1,
    VK_LAUNCH_APP2,
    VK_OEM_1,
    VK_OEM_PLUS,
    VK_OEM_COMMA,
    VK_OEM_MINUS,
    VK_OEM_PERIOD,
    VK_OEM_2,
    VK_OEM_3,
    VK_OEM_4,
    VK_OEM_5,
    VK_OEM_6,
    VK_OEM_7,
    VK_OEM_8,
    VK_OEM_AX,
    VK_OEM_102,
    VK_ICO_HELP,
    VK_ICO_00,
    VK_PROCESSKEY,
    VK_ICO_CLEAR,
    VK_PACKET,
    VK_ATTN,
    VK_CRSEL,
    VK_EXSEL,
    VK_EREOF,
    VK_PLAY,
    VK_ZOOM,
    VK_NONAME,
    VK_PA1,
    VK_OEM_CLEAR,
];

fn main() -> Result<()> {
    // Setup remappings
    unsafe {
        // + => é
        REMAPPED_KEYS[49] = Some(48);
        // é => +
        REMAPPED_KEYS[48] = Some(49);

        // š => alt
        REMAPPED_KEYS[51] = Some(164);
        // a => ctrl
        REMAPPED_KEYS[65] = Some(162);
        // alt => ctrl
        REMAPPED_KEYS[164] = Some(162);
    }

    // Fill syskey table
    for &syskey in SYS_KEYS.iter() {
        unsafe {
            SYS_KEYS_TABLE[syskey as usize] = true;
        }
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

macro_rules! call_next_hook {
    ($n_code:expr, $w_param:expr, $l_param:expr) => {
        return CallNextHookEx(WINDOW_HHOOK, $n_code, $w_param, $l_param);
    };
}

macro_rules! event_handled {
    () => {
        return 1;
    };
}

macro_rules! log_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*)
    };
}

unsafe extern "system" fn remap_keys_callback(
    n_code: i32,
    mut w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code != HC_ACTION {
        call_next_hook!(n_code, w_param, l_param);
    }

    let kbd_struct = &*(l_param as *const KBDLLHOOKSTRUCT);

    let trigger_key = kbd_struct.vkCode as usize;
    let possibly_remapped_key_in_range: Option<&Option<u8>> = REMAPPED_KEYS.get(trigger_key);
    if possibly_remapped_key_in_range.is_none() {
        call_next_hook!(n_code, w_param, l_param);
    }

    let possibly_remapped_key: &Option<u8> = possibly_remapped_key_in_range.unwrap();
    if possibly_remapped_key.is_none() {
        call_next_hook!(n_code, w_param, l_param);
    }

    let w_param_u32 = w_param as u32;
    let remmaped_key: u8 = possibly_remapped_key.unwrap();
    let scan_code = MapVirtualKeyW(remmaped_key as u32, MAPVK_VK_TO_VSC) as u8;
    if w_param_u32 == WM_SYSKEYDOWN
        || (w_param_u32 == WM_KEYDOWN
            && !is_sys_key(trigger_key as u8)
            && !is_sys_key(remmaped_key))
        || (w_param_u32 == WM_KEYDOWN
            && is_sys_key(remmaped_key)
            && GetAsyncKeyState(remmaped_key as i32) == 0)
    {
        log_debug!("keydown event fired");

        keybd_event(remmaped_key, scan_code, KEYEVENTF_EXTENDEDKEY, 0);

        event_handled!();
    }

    // WM_KEYUP, fires only once, so we can share it.
    if w_param_u32 == WM_KEYUP {
        log_debug!("keyup event fired");

        keybd_event(
            remmaped_key,
            scan_code,
            KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
            0,
        );

        event_handled!();
    }

    // Neccessary for syskey to stay down
    w_param = WM_SYSKEYDOWN as usize;

    event_handled!();
}

unsafe fn is_sys_key(key: u8) -> bool {
    *SYS_KEYS_TABLE.get(key as usize).unwrap_or(&false)
}
