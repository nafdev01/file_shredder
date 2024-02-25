// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use notify_rust::Notification as DesktopNotification;
use notify_rust::Timeout;

mod auth_commands;
mod initialize_app;

use crate::initialize_app::initialize_database;

fn main() {
    match initialize_database() {
        Ok(_) => {
            DesktopNotification::new()
                .summary("File Shredder Database Initialized!")
                .body(
                    "The tables and data for the File Shredder application have been initialized successfully."
                )
                .icon("32x32")
                .timeout(Timeout::Milliseconds(6000)) //milliseconds
                .show()
                .unwrap();
        }
        Err(_e) => {
            DesktopNotification::new()
                .summary("File Shredder Database Initialization Error!")
                .body("An error occurred while initializing the File Shredder database.")
                .icon("32x32")
                .timeout(Timeout::Milliseconds(6000)) //milliseconds
                .show()
                .unwrap();
        }
    }

    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                auth_commands::get_departments,
                auth_commands::create_employee,
                auth_commands::authenticate_employee,
                auth_commands::authenticate_admin
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
