// fn main() {
//     let abc: windows::Win32::UI::WindowsAndMessaging::HHOOK = 0;
//     println!("Hello, world!");
// }

use anyhow::{anyhow, Result};

use windows_sys::Win32::{
    System::LibraryLoader::GetModuleHandleA,
    UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA, TranslateMessage, HHOOK,
        MSG, WH_KEYBOARD_LL,
    },
};

static mut WINDOW_HOOK: HHOOK = 0;
fn main() -> Result<()> {
    unsafe {
        WINDOW_HOOK = SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(LowLevelKeyboardProc),
            GetModuleHandleA(std::ptr::null()),
            0,
        );

        if WINDOW_HOOK == 0 {
            return Err(anyhow!("Failed to set WINDOWS_HOOK."));
        }

        let messages: *mut MSG = std::ptr::null_mut();
        while GetMessageA(messages, 0, 0, 0) != 0 {
            TranslateMessage(messages);
            DispatchMessageA(messages);
        }
    }

    println!("Everything executed correctlly.");
    Ok(())
}

unsafe extern "system" fn LowLevelKeyboardProc(
    n_code: i32,
    w_param: usize,
    l_param: isize,
) -> isize {
    print!("{}", n_code);
    CallNextHookEx(WINDOW_HOOK, n_code, w_param, l_param)
}
