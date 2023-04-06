// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_service_config,
            commands::create_profile,
            commands::delete_profile,
            commands::save_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
