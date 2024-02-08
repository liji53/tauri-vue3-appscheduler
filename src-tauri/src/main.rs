// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app_manager;
mod sys_resource;
mod task_manager;
mod utils;

use app_manager::view::{
    get_app_categories, get_app_tree, get_apps, install_app, readme_app, ungrade_app, uninstall_app,
};
use rusqlite::Connection;
use sys_resource::get_sys_info;
use task_manager::view::{create_job, get_jobs};

fn main() {
    tauri::Builder::default()
        .setup(|_| {
            // 建表
            let db_path = utils::program_db_path();
            let conn = Connection::open(db_path)?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS Task (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    remark TEXT,
                    status BOOLEAN DEFAULT true,
                    create_at DATETIME DEFAULT CURRENT_TIMESTAMP,                    
                    cron TEXT,
                    app_name TEXT NOT NULL,
                    category TEXT,
                    url TEXT NOT NULL
                )",
                (),
            )?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_sys_info,
            install_app,
            uninstall_app,
            readme_app,
            get_app_categories,
            get_apps,
            get_app_tree,
            ungrade_app,
            create_job,
            get_jobs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
