#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::File, error};
use std::io::{prelude::*};
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use ts_rs::TS;
use std::env;

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

#[tauri::command]
fn create_memo_command(task_path:String,arg_memo: Todo) {
    println!("送られてきたデータ： {:?}",arg_memo);
    // エラーハンドリングは必要
    create_todo(task_path, arg_memo);
}


#[tauri::command]
fn move_memo_command(task_path: String,arg_memo: Todo, from: String, to: String) {
    println!("送られてきたデータ task_path {:?}",task_path);
    println!("送られてきたデータ arg_memo {:?}",arg_memo);
    println!("送られてきたデータ from {:?}",from);
    println!("送られてきたデータ to {:?}",to);

    // エラーハンドリングは必要
    move_todo(task_path,arg_memo,from,to);
}

#[tauri::command]
fn initial_file_command(path: String) {
    println!("パス {:?}",path);
    println!("パス {:?}",env::vars());
}

#[tauri::command]
fn initial_setting_command(path: String) -> Result<Memo, String> {
    let result = read_file(path);
    match result {
        Ok(memo) => Ok(memo),
        Err(_) => Err(String::from("ファイル読み込み時にエラーが発生しました"))
    }
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        create_memo_command,
        move_memo_command,
        initial_file_command,
        initial_setting_command,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


