use winapi::um::winuser::{
    VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_RWIN,
};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct LastSentRemapInfo {
    pub sender_key: u8,
    pub remap_key: u8,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct RemappedShortcut {
    pub from_shortcut: u16,
    pub to_shortcut: u16,
}

//   0         0         0       0          0                   0                   0               0
//   ^         ^         ^       ^          ^                   ^                   ^               ^
// windows  control     alt     shift   is_windows_right    is_control_right    is_alt_right    is_shift_right
const LWIN_MASK: u8 = 128;
const LCTRL_MASK: u8 = 64;
const LALT_MASK: u8 = 32;
const LSHIFT_MASK: u8 = 16;

const RWIN_MASK: u8 = 8;
const RCTRL_MASK: u8 = 4;
const RALT_MASK: u8 = 2;
const RSHIFT_MASK: u8 = 1;

impl RemappedShortcut {
    pub fn new(
        from_shortcut_keys: [u8; 4],
        from_shorcut_execution_key: u8,
        to_shortcut_keys: [u8; 4],
        to_shorcut_execution_key: u8,
    ) -> Self {
        RemappedShortcut {
            from_shortcut: Self::combine_mask_and_execution_key(
                Self::keys_to_mask(from_shortcut_keys),
                from_shorcut_execution_key,
            ),
            to_shortcut: Self::combine_mask_and_execution_key(
                Self::keys_to_mask(to_shortcut_keys),
                to_shorcut_execution_key,
            ),
        }
    }

    pub fn keys_to_mask(keys: [u8; 4]) -> u8 {
        let mut translated_shortcut = 0_u8;

        for key in keys {
            translated_shortcut |= match key as i32 {
                VK_LWIN => LWIN_MASK,
                VK_LCONTROL => LCTRL_MASK,
                VK_LMENU => LALT_MASK,
                VK_LSHIFT => LSHIFT_MASK,

                VK_RWIN => RWIN_MASK,
                VK_RCONTROL => RCTRL_MASK,
                VK_RMENU => RALT_MASK,
                VK_RSHIFT => RSHIFT_MASK,

                _ => 0_u8,
            };
        }

        translated_shortcut
    }

    pub fn combine_mask_and_execution_key(key_mask: u8, execution_key: u8) -> u16 {
        (key_mask as u16) << 8 | execution_key as u16
    }

    #[allow(clippy::too_many_arguments)]
    pub fn mask_matches(
        &self,
        lwin_state: bool,
        mut lctrl_state: bool,
        lalt_state: bool,
        lshift_state: bool,
        rwin_state: bool,
        rctrl_state: bool,
        ralt_state: bool,
        rshift_state: bool,
    ) -> bool {
        let shortcut = (self.from_shortcut >> 8) as u8;

        if ralt_state && lctrl_state {
            lctrl_state = false;
        }

        if ((lwin_state && shortcut & LWIN_MASK == 0) || (!lwin_state && shortcut & LWIN_MASK > 0))
            || ((lctrl_state && shortcut & LCTRL_MASK == 0)
                || (!lctrl_state && shortcut & LCTRL_MASK > 0))
            || ((lalt_state && shortcut & LALT_MASK == 0)
                || (!lalt_state && shortcut & LALT_MASK > 0))
            || ((lshift_state && shortcut & LSHIFT_MASK == 0)
                || (!lshift_state && shortcut & LSHIFT_MASK > 0))
            || ((rwin_state && shortcut & RWIN_MASK == 0)
                || (!rwin_state && shortcut & RWIN_MASK > 0))
            || ((rctrl_state && shortcut & RCTRL_MASK == 0)
                || (!rctrl_state && shortcut & RCTRL_MASK > 0))
            || ((ralt_state && shortcut & RALT_MASK == 0)
                || (!ralt_state && shortcut & RALT_MASK > 0))
            || ((rshift_state && shortcut & RSHIFT_MASK == 0)
                || (!rshift_state && shortcut & RSHIFT_MASK > 0))
        {
            return false;
        }

        true
    }

    pub fn get_from_execution_char(&self) -> u8 {
        (self.from_shortcut & 255) as u8
    }

    pub fn get_to_shortcut(&self) -> [Option<u8>; 5] {
        Self::get_shortcut(self.to_shortcut)
    }

    pub fn get_from_shortcut(&self) -> [Option<u8>; 5] {
        Self::get_shortcut(self.from_shortcut)
    }

    fn get_shortcut(base_shortcut: u16) -> [Option<u8>; 5] {
        let shortcut = (base_shortcut >> 8) as u8;

        let win_place = if shortcut & LWIN_MASK > 0 {
            Some(VK_LWIN as u8)
        } else if shortcut & RWIN_MASK > 0 {
            Some(VK_RWIN as u8)
        } else {
            None
        };

        let ctrl_place = if shortcut & LCTRL_MASK > 0 {
            Some(VK_LCONTROL as u8)
        } else if shortcut & RCTRL_MASK > 0 {
            Some(VK_RCONTROL as u8)
        } else {
            None
        };

        let alt_place = if shortcut & LALT_MASK > 0 {
            Some(VK_LMENU as u8)
        } else if shortcut & RALT_MASK > 0 {
            Some(VK_RMENU as u8)
        } else {
            None
        };

        let shift_place = if shortcut & LSHIFT_MASK > 0 {
            Some(VK_LSHIFT as u8)
        } else if shortcut & RSHIFT_MASK > 0 {
            Some(VK_RSHIFT as u8)
        } else {
            None
        };

        [
            win_place,
            ctrl_place,
            alt_place,
            shift_place,
            Some((base_shortcut & 255) as u8),
        ]
    }
}
