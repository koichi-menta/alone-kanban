use std::io::BufReader;
use std::{fs::File, error};
use std::io::{prelude::*};

use super::{Kanban, Task};

pub fn read_file(path: String) -> Result<Kanban, Box<dyn error::Error>> {
    let tasks_file = File::open(path)?;
    let reader = BufReader::new(tasks_file);
    let kanban= serde_json::from_reader(reader)?;
    
    Ok(kanban)
}

pub fn create_task(task_path: String, task: Task, target: String) -> Result<bool, Box<dyn error::Error>> {
    let task = Task {
        id: String::from(task.id),
        name: String::from(task.name),
        is_complete: task.is_complete,
    };
    let read_kanban_result = read_file(task_path.clone());

    match read_kanban_result {
        Ok(mut kanban) => {
            match &*target {
                "todo" => {
                    kanban.todo.push(task);
                }
                "in_progress" => {
                    kanban.in_progress.push(task);
                }
                "done" => {
                    kanban.done.push(task);
                }
                _ => {println!("対象がありません")}
            }
            let json_data = serde_json::to_string_pretty(&kanban).unwrap();
            let mut json_file = File::create(task_path).unwrap();
            writeln!(json_file, "{}", json_data);
            Ok(true)
        },
        Err(err) =>  {
            Err(err)
        }
    }
}

pub fn move_task(task_path: String,task: Task, from: String, to: String) -> Result<bool, Box<dyn error::Error>> {
    let task = Task {
        id: String::from(task.id),
        name: String::from(task.name),
        is_complete: task.is_complete,
    };
    let mut read_kanban = read_file(task_path.clone())?;
    let from_to: (&str, &str) = (&from, &to);

    match &*from {
        "todo" => {
            let target_id = read_kanban.todo.iter().position(|x| *x.id == task.id);
            match target_id {
                Some(x) => {
                    match &from_to {
                        ("todo", "in_progress") => {
                            read_kanban.todo.remove(x);
                            read_kanban.in_progress.push(task);
                        },
                        ("todo", "done") => {
                            read_kanban.todo.remove(x);
                            read_kanban.done.push(task);
                        },
                        _ => {println!("taskではありません")},
                    }
                },
                None => println!("移動するデータはありません"),
            }
        },
        "in_progress" => {
            let target_id = read_kanban.in_progress.iter().position(|x| *x.id == task.id);
            match target_id {
                Some(x) => {
                    match &from_to {
                        ("in_progress", "todo") => {
                            read_kanban.in_progress.remove(x);
                            read_kanban.todo.push(task);
                        },
                        ("in_progress", "done") => {
                            read_kanban.in_progress.remove(x);
                            read_kanban.done.push(task);
                        },
                        _ => {println!("in_progressではありません")},
                    }
                },
                None => println!("移動するデータはありません"),
            }
        },
        "done" => {
            let target_id = read_kanban.done.iter().position(|x| *x.id == task.id);
            match target_id {
                Some(x) => {
                    match &from_to {
                        ("done", "in_progress") => {
                            read_kanban.done.remove(x);
                            read_kanban.in_progress.push(task);
                        },
                        ("done", "todo") => {
                            read_kanban.done.remove(x);
                            read_kanban.todo.push(task);
                        },
                        _ => {println!("in_progressではありません")},
                    }
                },
                None => println!("移動するデータはありません"),
            }
        },
        _ => {println!("taskではありません")},
    };

    let json_data = serde_json::to_string_pretty(&read_kanban).unwrap();
    println!("更新内容 {}", json_data);
    let mut json_file = File::create(task_path).unwrap();
    writeln!(json_file, "{}", json_data);

    Ok(true)
}

pub fn delete_task(path: String, target: String, id: String) -> Result<bool, Box<dyn error::Error>> {
    let mut read_kanban = read_file(path.clone())?;
    
    match &*target {
        "todo" => {
            let target_id = read_kanban.todo.iter().position(|x| *x.id == id);
            println!("target_id {:?}",target_id);
            match target_id {
                Some(fix_target_id) => {
                    read_kanban.todo.remove(fix_target_id);
                },
                None => {println!("taskではありません")},
            }
        },
        "in_progress" => {
            let target_id = read_kanban.in_progress.iter().position(|x| *x.id == id);
            match target_id {
                Some(fix_target_id) => {
                    read_kanban.in_progress.remove(fix_target_id);
                },
                None => {println!("taskではありません")},
            }
        },
        "done" => {
            let target_id = read_kanban.done.iter().position(|x| *x.id == id);
            match target_id {
                Some(fix_target_id) => {
                    read_kanban.done.remove(fix_target_id);
                },
                None => {println!("taskではありません")},
            }
        },
        _ => {println!("taskではありません")},
    }
    let json_data = serde_json::to_string_pretty(&read_kanban).unwrap();
    let mut json_file = File::create(path).unwrap();
    writeln!(json_file, "{}", json_data);
    Ok(true)

}    
