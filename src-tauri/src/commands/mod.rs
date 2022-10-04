mod functions;

use tauri::{State};
use ts_rs::TS;
use serde::{Deserialize, Serialize};
use crate::TaskPath;


#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct Todo {
    id: String,
    name: String,
    is_complete: bool,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct Memo {
    todo: Vec<Todo>,
    in_progress: Vec<Todo>,
    done: Vec<Todo>,
}

#[tauri::command]
pub fn create_memo_command(arg_memo: Todo, task_path: State<TaskPath>) {
    let m_task_path = task_path.0.lock().unwrap();
    // エラーハンドリングは必要
    functions::create_todo(m_task_path.to_string(), arg_memo);
}

#[tauri::command]
pub fn move_memo_command(arg_memo: Todo, from: String, to: String, task_path: State<TaskPath>) {
    let m_task_path = task_path.0.lock().unwrap();
    // エラーハンドリングは必要
    functions::move_todo(m_task_path.to_string(),arg_memo,from,to);
}

#[tauri::command]
pub fn initial_setting_command(path:String, task_path: State<TaskPath>) -> Result<Memo, String> {
    let mut m_task_path = task_path.0.lock().unwrap();
    *m_task_path = path;

    let result = functions::read_file(m_task_path.to_string());

    match result {
        Ok(memo) => Ok(memo),
        Err(_) => Err(String::from("ファイル読み込み時にエラーが発生しました"))
    }
}

#[tauri::command]
pub fn delete_memo_command(target: String, id: String,task_path: State<TaskPath>) -> Result<bool, String> {
    let m_task_path = task_path.0.lock().unwrap();
    let result = functions::delete_memo(m_task_path.to_string(), target,id);

    match result {
        Ok(memo) => Ok(memo),
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
pub fn get_memo_command(task_path: State<TaskPath>) -> Result<Memo,String> {
    let mut m_task_path = task_path.0.lock().unwrap();
    let result = functions::read_file(m_task_path.to_string());

    match result {
        Ok(memo) => Ok(memo),
        Err(_) => Err(String::from("ファイル読み込み時にエラーが発生しました"))
    }
}
