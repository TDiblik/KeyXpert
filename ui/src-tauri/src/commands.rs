use mapper_service::{
    shared_constants,
    shared_models::{Profile, ServiceConfig},
};
use uuid::Uuid;

use crate::{
    models::{CommandResult, ProfileSaveObj},
    utils,
};

#[tauri::command]
pub fn get_service_config() -> CommandResult<ServiceConfig> {
    let Ok(config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to get service config file");
    };

    CommandResult::new_success_with_value(Some(config), None)
}

// #[tauri::command]
// fn get_ui_config() -> ServiceConfig {
//     get_config::<ServiceConfig>(constants::UI_CONFIG_FILE_PATH)
//         .expect("Unable to get ui config file")
// }

#[tauri::command]
pub fn create_profile() -> CommandResult<Uuid> {
    let Ok(mut config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to read or parse service config file.");
    };

    let new_profile = Profile::default();
    let new_uuid = new_profile.id;
    config.profiles.push(new_profile);

    if utils::save_config(&shared_constants::service_config_file_path(), &config).is_err() {
        return CommandResult::new_err("Unable to add profile to service config file");
    }

    CommandResult::new_success_with_value(Some(new_uuid), None)
}

#[tauri::command]
pub fn delete_profile(id_to_delete: Uuid) -> CommandResult<()> {
    let Ok(mut config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to read or parse service config file.");
    };

    if let Some(position_to_delete) = config.profiles.iter().position(|s| s.id == id_to_delete) {
        let deleted_profile = config.profiles.remove(position_to_delete);

        // TODO: Rewrite this nested monstrosity as soon as if let chains are stable. (https://github.com/rust-lang/rust/issues/53667)
        if let Some(active_profile) = config.active_profile {
            if deleted_profile.id == active_profile {
                config.active_profile = None;
            }
        }
    } else {
        return CommandResult::new_err(
            "Unable to determine position where in config to delete based on id.",
        );
    }

    if utils::save_config(&shared_constants::service_config_file_path(), &config).is_err() {
        return CommandResult::new_err("Unable to remove profile from service config file.");
    }

    CommandResult::new_success(None)
}

#[tauri::command]
pub fn save_profile(profile: ProfileSaveObj) -> CommandResult<()> {
    let Ok(mut config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to read or parse service config file.");
    };

    if let Some(active_profile) = config.active_profile {
        if active_profile == profile.id {
            config.active_profile = None;
        }
    }

    if profile.use_this_profile {
        config.active_profile = Some(profile.id);
    }

    if let Some(profile_to_change) = config.profiles.iter_mut().find(|s| s.id == profile.id) {
        *profile_to_change = profile.into();
    } else {
        return CommandResult::new_err(
            "Unable to find profile in saved profiles -- unable to save.",
        );
    }

    if utils::save_config(&shared_constants::service_config_file_path(), &config).is_err() {
        return CommandResult::new_err("Unable to save new service config file");
    }

    CommandResult::new_success(None)
}
