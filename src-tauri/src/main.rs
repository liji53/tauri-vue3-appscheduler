// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app_manager;
mod sys_resource;
mod utils;
use app_manager::view::{
    get_app_categories, get_apps, install_app, readme_app, ungrade_app, uninstall_app,
};
use sys_resource::get_sys_info;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_sys_info,
            install_app,
            uninstall_app,
            readme_app,
            get_app_categories,
            get_apps,
            ungrade_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
