// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth_commands;

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![auth_commands::create_user, auth_commands::authenticate_user]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
