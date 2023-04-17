use mapper_service::shared_models::{KeyRemap, Profile, ShortcutRemap};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ProfileSaveObj {
    pub id: Uuid,
    pub name: String,
    pub key_remaps: Vec<KeyRemap>,
    pub shortcut_remaps: Vec<ShortcutRemap>,
    pub use_this_profile: bool,
}

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
pub struct CommandResult<T> {
    pub is_success: bool,
    pub message: String,
    pub result: Option<T>,
}

impl<T> CommandResult<T>
where
    T: Default + DeserializeOwned + Serialize,
{
    pub fn new_err(msg: &str) -> CommandResult<T> {
        CommandResult {
            is_success: false,
            message: String::from(msg),
            result: None,
        }
    }

    pub fn new_success(msg: Option<String>) -> CommandResult<T> {
        CommandResult {
            is_success: true,
            message: msg.unwrap_or(String::from("")),
            result: None,
        }
    }

    pub fn new_success_with_value(value: Option<T>, msg: Option<String>) -> CommandResult<T> {
        CommandResult {
            is_success: true,
            message: msg.unwrap_or(String::from("")),
            result: value,
        }
    }
}

#[cfg(target_os = "windows")]
pub trait SilentCmd {
    fn new_silent_cmd() -> std::process::Command;
}

#[cfg(target_os = "windows")]
impl SilentCmd for std::process::Command {
    fn new_silent_cmd() -> std::process::Command {
        use std::os::windows::process::CommandExt;
        use std::process::Command;

        let mut new_cmd = Command::new("cmd");
        new_cmd.creation_flags(0x08000000); // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
        new_cmd
    }
}
