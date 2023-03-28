use uuid::Uuid;

use crate::{
    constants,
    models::{Profile, ServiceConfig},
    utils,
};

#[tauri::command]
pub fn get_service_config() -> ServiceConfig {
    utils::get_config::<ServiceConfig>(constants::SERVICE_CONFIG_FILE_PATH)
        .expect("Unable to get service config file")
}

// #[tauri::command]
// fn get_ui_config() -> ServiceConfig {
//     get_config::<ServiceConfig>(constants::UI_CONFIG_FILE_PATH)
//         .expect("Unable to get ui config file")
// }

#[tauri::command]
pub fn create_profile() -> Uuid {
    let mut config = utils::get_config::<ServiceConfig>(constants::SERVICE_CONFIG_FILE_PATH)
        .expect("Unable to get service config file");

    let new_profile = Profile::default();
    let new_uuid = new_profile.id;
    config.profiles.push(new_profile);
    utils::save_config(constants::SERVICE_CONFIG_FILE_PATH, &config)
        .expect("Unable to add profile to service config file");

    new_uuid
}
