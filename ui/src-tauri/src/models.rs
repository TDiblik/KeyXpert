use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct ServiceConfig {
    pub active_profile: Option<Uuid>,
    pub profiles: Vec<Profile>,
}

#[derive(Serialize, Deserialize)]
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
            key_remaps: vec![],
            shortcut_remaps: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyRemap {
    pub from: u8,
    pub to: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ShortcutRemap {
    pub from_shortcut_holding_keys: [u8; 4],
    pub from_shorcut_execution_key: u8,
    pub to_shortcut_holding_keys: [u8; 4],
    pub to_shorcut_execution_key: u8,
}
