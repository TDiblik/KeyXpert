// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winapi::{shared::minwindef::BYTE, um::winuser::GetKeyboardState};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let mut keyboard_state: [BYTE; 256] = [0; 256];
    unsafe {
        GetKeyboardState(keyboard_state.as_mut_ptr());
    }
    println!("{:?}", keyboard_state.map(|s| (s & 0x80) != 0));

    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
