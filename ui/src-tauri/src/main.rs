// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod constants;
mod models;
mod utils;

use tauri::{WindowBuilder, WindowUrl};

fn main() {
    // TODO: Do not spawn mulitple windows on startup, just add limitations to the default one
    tauri::Builder::default()
        .setup(|app| {
            WindowBuilder::new(app, "core", WindowUrl::App("index.html".into()))
                .min_inner_size(750.0, 650.0)
                .max_inner_size(1250.0, 750.0)
                .title("KeyXpert")
                .build()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_service_config,
            commands::create_profile,
            commands::delete_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
