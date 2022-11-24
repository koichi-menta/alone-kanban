mod functions;

use tauri::{State};
use ts_rs::TS;
use serde::{Deserialize, Serialize};
use crate::TaskPath;


#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct Task {
    id: String,
    name: String,
    is_complete: bool,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct Kanban {
    todo: Vec<Task>,
    in_progress: Vec<Task>,
    done: Vec<Task>,
}

#[tauri::command]
pub fn create_task_command(task: Task, target: String, task_path: State<TaskPath>) {
    let m_task_path = task_path.0.lock().unwrap();
    // エラーハンドリングは必要
    functions::create_task(m_task_path.to_string(), task, target);
}

#[tauri::command]
pub fn move_task_command(task: Task, from: String, to: String, task_path: State<TaskPath>) {
    let m_task_path = task_path.0.lock().unwrap();
    // エラーハンドリングは必要
    functions::move_task(m_task_path.to_string(),task,from,to);
}

#[tauri::command]
pub fn initial_setting_command(path:String, task_path: State<TaskPath>) -> Result<Kanban, String> {
    let mut m_task_path = task_path.0.lock().unwrap();
    *m_task_path = path;

    let result = functions::read_file(m_task_path.to_string());

    match result {
        Ok(kanban) => Ok(kanban),
        Err(_) => Err(String::from("ファイル読み込み時にエラーが発生しました"))
    }
}

#[tauri::command]
pub fn delete_task_command(target: String, id: String,task_path: State<TaskPath>) -> Result<bool, String> {
    let m_task_path = task_path.0.lock().unwrap();
    let result = functions::delete_task(m_task_path.to_string(), target,id);

    match result {
        Ok(kanban) => Ok(kanban),
        Err(_) => Err(String::from("ファイル読み込み時にエラーが発生しました"))
    }
}

#[tauri::command]
pub fn check_path_command(task_path: State<TaskPath>) -> Result<bool,String> {
    let m_task_path = task_path.0.lock().unwrap();

    if m_task_path.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}


#[tauri::command]
pub fn get_task_command(task_path: State<TaskPath>) -> Result<Kanban,String> {
    let m_task_path = task_path.0.lock().unwrap();
    let result = functions::read_file(m_task_path.to_string());

    match result {
        Ok(kanban) => Ok(kanban),
        Err(_) => Err(String::from("ファイル読み込み時にエラーが発生しました"))
    }
}
