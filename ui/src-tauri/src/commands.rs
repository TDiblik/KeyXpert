use std::{fs, path::Path};

use crate::constants;

#[tauri::command]
fn get_config() -> String {
    let config_path = Path::new(constants::CONFIG_DIR_PATH);
    fs::create_dir_all(config_path);
}
