use anyhow::anyhow;

use crate::{
    models::{LastSentRemapInfo, RemappedShortcut},
    utils::{get_active_profile, is_sys_key},
    ENABLE_RECURSIVE_REMAPPING, ENABLE_RECURSIVE_SHORTCUTS, LAST_SENT_REMAP_INFO, REMAPPED_KEYS,
    REMAPPED_SHORTCUTS, REMAPPED_SHORTCUTS_CONTAIN_KEY, STOP_RECURSIVE_SHORTCUT,
};

pub struct App;
pub trait AppCore {
    fn setup() -> anyhow::Result<()>;
    unsafe fn program_loop() -> anyhow::Result<()>;
}

// ------------------------------------
// ------ Windows implementation ------
// ------------------------------------
#[cfg(target_os = "windows")]
use crate::{
    call_next_hook, event_handled, keybd_trigger_key_down, keybd_trigger_key_up, log_debug,
    map_virtual_key,
};

#[cfg(target_os = "windows")]
use winapi::{
    shared::minwindef::{LPARAM, LRESULT, WPARAM},
    shared::windef::HHOOK__,
    um::{
        libloaderapi::GetModuleHandleW,
        winuser::{
            keybd_event, CallNextHookEx, DispatchMessageW, GetAsyncKeyState, GetMessageW,
            MapVirtualKeyW, SetWindowsHookExW, TranslateMessage, HC_ACTION, KBDLLHOOKSTRUCT,
            KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC, MSG, VK_LCONTROL, VK_LMENU,
            VK_LSHIFT, VK_LWIN, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_RWIN, WH_KEYBOARD_LL,
            WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
        },
    },
};

#[cfg(target_os = "windows")]
pub static mut WINDOW_HHOOK: *mut HHOOK__ = std::ptr::null_mut();

#[cfg(target_os = "windows")]
impl AppCore for App {
    fn setup() -> anyhow::Result<()> {
        let active_profile = get_active_profile()?;

        for key_remap in active_profile.key_remaps.iter() {
            unsafe { REMAPPED_KEYS[key_remap.from as usize] = Some(key_remap.to) }
        }

        for shortcut_remap in active_profile.shortcut_remaps.iter() {
            unsafe {
                REMAPPED_SHORTCUTS_CONTAIN_KEY
                    [shortcut_remap.from_shortcut_execution_key as usize] = true;
                REMAPPED_SHORTCUTS.push(RemappedShortcut::new(
                    shortcut_remap.from_shortcut_holding_keys,
                    shortcut_remap.from_shortcut_execution_key,
                    shortcut_remap.to_shortcut_holding_keys,
                    shortcut_remap.to_shortcut_execution_key,
                ));
            }
        }

        Ok(())
    }

    unsafe fn program_loop() -> anyhow::Result<()> {
        let h_mod = GetModuleHandleW(std::ptr::null());

        WINDOW_HHOOK = SetWindowsHookExW(WH_KEYBOARD_LL, Some(Self::remap_keys_callback), h_mod, 0);
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
}

#[cfg(target_os = "windows")]
impl App {
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

                let keys_to_release = shortcut_info.get_from_shortcut();
                for possible_key in keys_to_release.iter().rev() {
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
                    if key == VK_RMENU as u8 {
                        continue;
                    }
                    keybd_trigger_key_up!(key, map_virtual_key!(key));
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
            || (w_param_u32 == WM_KEYDOWN
                && is_sys_key(remapped_key)
                && LAST_SENT_REMAP_INFO.is_none())
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
}

// ------------------------------------
// ------- Linux implementation -------
// ------------------------------------
//
// Help needed
