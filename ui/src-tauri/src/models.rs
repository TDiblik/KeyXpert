use mapper_service::shared_models::{KeyRemap, Profile, ShortcutRemap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl std::convert::From<ProfileSaveObj> for Profile {
    fn from(val: ProfileSaveObj) -> Self {
        Profile {
            id: val.id,
            name: val.name,
            key_remaps: val.key_remaps,
            shortcut_remaps: val.shortcut_remaps,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProfileSaveObj {
    pub id: Uuid,
    pub name: String,
    pub key_remaps: Vec<KeyRemap>,
    pub shortcut_remaps: Vec<ShortcutRemap>,
    pub use_this_profile: bool,
}
