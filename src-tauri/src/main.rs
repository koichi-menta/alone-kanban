#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use std::{fs::File, error};
use std::io::{prelude::*};
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use std::io::BufReader;
use ts_rs::TS;
use std::env;

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        create_memo_command,
        move_memo_command,
        initial_setting_command,
        delete_memo_command,
        check_path,
        get_memo_command,
    ])
    .setup(|app| {
        app.manage(TaskPath(Mutex::new(String::from(""))));

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
struct Todo {
    id: String,
    name: String,
    is_complete: bool,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
struct Memo {
    todo: Vec<Todo>,
    in_progress: Vec<Todo>,
    done: Vec<Todo>,
}

#[derive(Debug)]
struct TaskPath(Mutex<String>);

fn read_file(path: String) ->  Result<Memo, Box<dyn error::Error>> {
    let memos_file = File::open(path)?;
    let reader = BufReader::new(memos_file);
    let memo= serde_json::from_reader(reader)?;
    
    Ok(memo)
}

fn create_todo(task_path: String,arg_memo: Todo) -> Result<bool, Box<dyn error::Error>> {
    let todo = Todo {
        id: String::from(arg_memo.id),
        name: String::from(arg_memo.name),
        is_complete: arg_memo.is_complete,
    };
    let read_memo_result = read_file(task_path.clone());

    match read_memo_result {
        Ok(mut memo) => {
            memo.todo.push(todo);
            let json_dadta = serde_json::to_string_pretty(&memo).unwrap();
            let mut json_file = File::create(task_path).unwrap();
            writeln!(json_file, "{}", json_dadta);
            Ok(true)
        },
        Err(err) =>  {
            Err(err)
        }
    }
}

fn move_todo(task_path: String,arg_memo: Todo, from: String, to: String) -> Result<bool, Box<dyn error::Error>> {
    let todo = Todo {
        id: String::from(arg_memo.id),
        name: String::from(arg_memo.name),
        is_complete: arg_memo.is_complete,
    };
    let mut read_memo = read_file(task_path.clone())?;
    let from_to: (&str, &str) = (&from, &to);

    match &*from {
        "todo" => {
            let target_id = read_memo.todo.iter().position(|x| *x.id == todo.id);
            match target_id {
                Some(x) => {
                    match &from_to {
                        ("todo", "in_progress") => {
                            read_memo.todo.remove(x);
                            read_memo.in_progress.push(todo);
                        },
                        ("todo", "done") => {
                            read_memo.todo.remove(x);
                            read_memo.done.push(todo);
                        },
                        _ => {println!("todoではありません")},
                    }
                },
                None => println!("移動するデータはありません"),
            }
        },
        "in_progress" => {
            let target_id = read_memo.in_progress.iter().position(|x| *x.id == todo.id);
            match target_id {
                Some(x) => {
                    match &from_to {
                        ("in_progress", "todo") => {
                            read_memo.in_progress.remove(x);
                            read_memo.todo.push(todo);
                        },
                        ("in_progress", "done") => {
                            read_memo.in_progress.remove(x);
                            read_memo.done.push(todo);
                        },
                        _ => {println!("in_progressではありません")},
                    }
                },
                None => println!("移動するデータはありません"),
            }
        },
        "done" => {
            let target_id = read_memo.done.iter().position(|x| *x.id == todo.id);
            match target_id {
                Some(x) => {
                    match &from_to {
                        ("done", "in_progress") => {
                            read_memo.done.remove(x);
                            read_memo.in_progress.push(todo);
                        },
                        ("done", "todo") => {
                            read_memo.done.remove(x);
                            read_memo.todo.push(todo);
                        },
                        _ => {println!("in_progressではありません")},
                    }
                },
                None => println!("移動するデータはありません"),
            }
        },
        _ => {println!("todoではありません")},
    };

    let json_data = serde_json::to_string_pretty(&read_memo).unwrap();
    println!("更新内容 {}", json_data);
    let mut json_file = File::create(task_path).unwrap();
    writeln!(json_file, "{}", json_data);

    Ok(true)
}

fn delete_memo(path: String, target: String, id: String) -> Result<bool, Box<dyn error::Error>> {
    let mut read_memo = read_file(path.clone())?;
    
    match &*target {
        "todo" => {
            let target_id = read_memo.todo.iter().position(|x| *x.id == id);
            println!("target_id {:?}",target_id);
            match target_id {
                Some(fix_target_id) => {
                    read_memo.todo.remove(fix_target_id);
                },
                None => {println!("todoではありません")},
            }
        },
        "in_progress" => {
            let target_id = read_memo.in_progress.iter().position(|x| *x.id == id);
            match target_id {
                Some(fix_target_id) => {
                    read_memo.in_progress.remove(fix_target_id);
                },
                None => {println!("todoではありません")},
            }
        },
        "done" => {
            let target_id = read_memo.done.iter().position(|x| *x.id == id);
            match target_id {
                Some(fix_target_id) => {
                    read_memo.done.remove(fix_target_id);
                },
                None => {println!("todoではありません")},
            }
        },
        _ => {println!("todoではありません")},
    }
    let json_data = serde_json::to_string_pretty(&read_memo).unwrap();
    let mut json_file = File::create(path).unwrap();
    writeln!(json_file, "{}", json_data);
    Ok(true)

}

#[tauri::command]
fn create_memo_command(arg_memo: Todo, task_path: State<TaskPath>) {
    let m_task_path = task_path.0.lock().unwrap();
    // エラーハンドリングは必要
    create_todo(m_task_path.to_string(), arg_memo);
}


#[tauri::command]
fn move_memo_command(arg_memo: Todo, from: String, to: String, task_path: State<TaskPath>) {
    let m_task_path = task_path.0.lock().unwrap();
    // エラーハンドリングは必要
    move_todo(m_task_path.to_string(),arg_memo,from,to);
}

#[tauri::command]
fn initial_setting_command(path:String, task_path: State<TaskPath>) -> Result<Memo, String> {
    let mut m_task_path = task_path.0.lock().unwrap();
    *m_task_path = path;

    let result = read_file(m_task_path.to_string());

    match result {
        Ok(memo) => Ok(memo),
        Err(_) => Err(String::from("ファイル読み込み時にエラーが発生しました"))
    }
}

#[tauri::command]
fn delete_memo_command(target: String, id: String,task_path: State<TaskPath>) -> Result<bool, String> {
    let m_task_path = task_path.0.lock().unwrap();
    let result = delete_memo(m_task_path.to_string(), target,id);

    match result {
        Ok(memo) => Ok(memo),
        Err(_) => Err(String::from("ファイル読み込み時にエラーが発生しました"))
    }
}

#[tauri::command]
fn check_path(task_path: State<TaskPath>) -> Result<bool,String> {
    let m_task_path = task_path.0.lock().unwrap();

    if m_task_path.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}


#[tauri::command]
fn get_memo_command(task_path: State<TaskPath>) -> Result<Memo,String> {
    let mut m_task_path = task_path.0.lock().unwrap();
    let result = read_file(m_task_path.to_string());

    match result {
        Ok(memo) => Ok(memo),
        Err(_) => Err(String::from("ファイル読み込み時にエラーが発生しました"))
    }
}

