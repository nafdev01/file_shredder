// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use notify_rust::Notification as DesktopNotification;
use notify_rust::Timeout;

mod auth_commands;
mod initialize_app;
mod shred_commands;
mod shredder_functions;
mod user_commands;

use crate::initialize_app::initialize_database;

fn main() {
    match initialize_database() {
        Ok(_) => {}
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

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            auth_commands::get_departments,
            auth_commands::create_employee,
            auth_commands::authenticate_employee,
            auth_commands::authenticate_admin,
            user_commands::get_employee,
            user_commands::get_admin,
            user_commands::update_employee,
            user_commands::update_admin,
            user_commands::change_employee_password,
            user_commands::change_admin_password,
            shred_commands::find_files,
            shred_commands::get_search_history,
            shred_commands::create_shred_request,
            shred_commands::get_pending_shred_requests,
            shred_commands::get_approved_shred_requests,
            shred_commands::get_denied_shred_requests,
            shred_commands::update_shred_request,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
