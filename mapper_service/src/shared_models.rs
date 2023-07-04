use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceConfig {
    pub active_profile: Option<Uuid>,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub key_remaps: Vec<KeyRemap>,
    pub shortcut_remaps: Vec<ShortcutRemap>,
}

impl Default for Profile {
    fn default() -> Self {
        let new_uuid = Uuid::new_v4();
        Self {
            id: new_uuid,
            name: format!("profile - {}", new_uuid),
            key_remaps: vec![KeyRemap::default()],
            shortcut_remaps: vec![ShortcutRemap::default()],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyRemap {
    pub from: u8,
    pub to: u8,
}

impl Default for KeyRemap {
    fn default() -> Self {
        Self {
            from: 0x41,
            to: 0x42,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutRemap {
    pub from_shortcut_holding_keys: [u8; 4],
    pub from_shortcut_execution_key: u8,
    pub to_shortcut_holding_keys: [u8; 4],
    pub to_shortcut_execution_key: u8,
}

impl Default for ShortcutRemap {
    fn default() -> Self {
        Self {
            from_shortcut_holding_keys: [0xA2, 0, 0, 0],
            from_shortcut_execution_key: 0x41,
            to_shortcut_holding_keys: [0xA2, 0, 0, 0],
            to_shortcut_execution_key: 0x42,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UIConfig {
    pub window_height: u32,
    pub window_width: u32,
    pub window_position_x: i32,
    pub window_position_y: i32,
}
