use mapper_service::{
    shared_constants,
    shared_models::{Profile, ServiceConfig},
};
use uuid::Uuid;

use crate::{models::ProfileSaveObj, utils};

#[tauri::command]
pub fn get_service_config() -> ServiceConfig {
    utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path())
        .expect("Unable to get service config file")
}

// #[tauri::command]
// fn get_ui_config() -> ServiceConfig {
//     get_config::<ServiceConfig>(constants::UI_CONFIG_FILE_PATH)
//         .expect("Unable to get ui config file")
// }

#[tauri::command]
pub fn create_profile() -> Uuid {
    let mut config =
        utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path())
            .expect("Unable to get service config file");

    let new_profile = Profile::default();
    let new_uuid = new_profile.id;
    config.profiles.push(new_profile);
    utils::save_config(&shared_constants::service_config_file_path(), &config)
        .expect("Unable to add profile to service config file");

    new_uuid
}

#[tauri::command]
pub fn delete_profile(id_to_delete: Uuid) {
    let mut config =
        utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path())
            .expect("Unable to get service config file");

    if let Some(position_to_delete) = config.profiles.iter().position(|s| s.id == id_to_delete) {
        let deleted_profile = config.profiles.remove(position_to_delete);

        // TODO: Rewrite this nested monstrosity as soon as if let chains are stable. (https://github.com/rust-lang/rust/issues/53667)
        if let Some(active_profile) = config.active_profile {
            if deleted_profile.id == active_profile {
                config.active_profile = None;
            }
        }
    }

    utils::save_config(&shared_constants::service_config_file_path(), &config)
        .expect("Unable to remove profile from service config file");
}

#[tauri::command]
pub fn save_profile(profile: ProfileSaveObj) {
    let mut config =
        utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path())
            .expect("Unable to get service config file");

    if profile.use_this_profile {
        config.active_profile = Some(profile.id);
    }

    if let Some(profile_to_change) = config.profiles.iter_mut().find(|s| s.id == profile.id) {
        *profile_to_change = profile.into();
    }

    utils::save_config(&shared_constants::service_config_file_path(), &config)
        .expect("Unable to save new service config file");
}
