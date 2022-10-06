#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use std::sync::Mutex;
use tauri::*;
use std::env;
use commands::*;

#[derive(Debug)]
pub struct TaskPath(Mutex<String>);


fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        create_task_command,
        move_task_command,
        initial_setting_command,
        delete_task_command,
        check_path_command,
        get_task_command,
    ])
    .setup(|app| {
        app.manage(TaskPath(Mutex::new(String::from(""))));

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
