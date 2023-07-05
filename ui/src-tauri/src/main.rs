// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mapper_service::{shared_constants, shared_models::UIConfig};
use tauri::Manager;

mod commands;
mod models;
mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::initial_check,
            commands::get_ui_config,
            commands::current_mapper_state,
            commands::change_mapper_state,
            commands::download_and_install_update,
            commands::get_service_config,
            commands::create_profile,
            commands::delete_profile,
            commands::save_profile,
            commands::save_advanced_settings
        ])
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
                let window = event.window().app_handle().get_window("main").unwrap();
                let position = window.inner_position().unwrap();
                let size = window.inner_size().unwrap();
                let new_ui_config = UIConfig {
                    window_height: size.height,
                    window_width: size.width,
                    window_position_x: position.x,
                    window_position_y: position.y,
                };

                _ = utils::save_config(&shared_constants::ui_config_file_path(), &new_ui_config);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
