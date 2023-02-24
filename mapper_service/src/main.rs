use anyhow::{anyhow, Result};
use winapi::{
    shared::minwindef::{LPARAM, LRESULT, WPARAM},
    um::{
        libloaderapi::GetModuleHandleW,
        winuser::{
            keybd_event, CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW,
            TranslateMessage, HC_ACTION, KBDLLHOOKSTRUCT, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP,
            MSG, VK_LCONTROL, VK_LMENU, WH_KEYBOARD_LL, WM_KEYUP, WM_SYSKEYDOWN,
        },
    },
};

fn main() -> Result<()> {
    unsafe {
        // Get the handle to the current module
        let h_mod = GetModuleHandleW(std::ptr::null());

        // Set the keyboard hook
        let window_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_callback), h_mod, 0);

        if window_hook.is_null() {
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
        return CallNextHookEx(std::ptr::null_mut(), $n_code, $w_param, $l_param)
    };
}

unsafe extern "system" fn keyboard_callback(
    n_code: i32,
    mut w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code != HC_ACTION {
        call_next_hook!(n_code, w_param, l_param);
    }

    let kbd_struct = &*(l_param as *const KBDLLHOOKSTRUCT);
    println!("Key code: {}", kbd_struct.vkCode);

    if kbd_struct.vkCode == VK_LMENU.try_into().unwrap() {
        match w_param.try_into().unwrap() {
            WM_SYSKEYDOWN => {
                keybd_event(
                    VK_LCONTROL.try_into().unwrap(),
                    0x1D,
                    KEYEVENTF_EXTENDEDKEY,
                    0,
                );
            }
            WM_KEYUP => {
                keybd_event(
                    VK_LCONTROL.try_into().unwrap(),
                    0x1D,
                    KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
                    0,
                );
            }
            _ => {
                w_param = WM_SYSKEYDOWN.try_into().unwrap(); // if you do not specify it changes back to alt
            }
        }
    }

    call_next_hook!(n_code, w_param, l_param);
}
