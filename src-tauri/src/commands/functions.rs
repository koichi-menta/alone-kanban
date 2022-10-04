use std::io::BufReader;
use std::{fs::File, error};
use std::io::{prelude::*};

use super::{Memo, Todo};

pub fn read_file(path: String) -> Result<Memo, Box<dyn error::Error>> {
    let memos_file = File::open(path)?;
    let reader = BufReader::new(memos_file);
    let memo= serde_json::from_reader(reader)?;
    
    Ok(memo)
}

pub fn create_todo(task_path: String,arg_memo: Todo) -> Result<bool, Box<dyn error::Error>> {
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

pub fn move_todo(task_path: String,arg_memo: Todo, from: String, to: String) -> Result<bool, Box<dyn error::Error>> {
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

pub fn delete_memo(path: String, target: String, id: String) -> Result<bool, Box<dyn error::Error>> {
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
