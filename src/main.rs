use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, fs, io, process};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    content: String,
    completed: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 && args.len() != 3 {
        eprintln!("Not the correct usage!");
        process::exit(0);
    }

    let command = &args[1];
    let mut task = "";

    if command != "list" {
        task = &args[2];
    }

    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());
    match command.as_str() {
        "add" => {
            add_task(&mut tasks, task.to_string());
            save_tasks(&tasks).expect("Unable to save");
            println!("Task {} has been added!", task);
        }
        "list" => {
            list_tasks(&tasks);
        }
        "complete" => {
            complete_task(&mut tasks, task.parse::<u32>().unwrap());
            save_tasks(&tasks).expect("Unable to save");
            println!("Task {} has been set to complete!", task);
        }
        _ => {
            println!("Wrong command entered!");
            process::exit(0);
        }
    }
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let serialized = serde_json::to_string(&tasks)?;
    fs::write("tasks.json", serialized)
}

fn load_tasks() -> Result<Vec<Task>, io::Error> {
    let data = fs::read_to_string("tasks.json")?;
    let tasks = serde_json::from_str(&data)?;
    Ok(tasks)
}

fn add_task(tasks: &mut Vec<Task>, content: String) {
    let new_task = Task {
        id: tasks.len() as u32 + 1,
        content,
        completed: false,
    };
    tasks.push(new_task);
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    } else {
        for task in tasks {
            println!(
                "[{}] {} - {}",
                task.id,
                if task.completed { "x" } else { " " },
                task.content
            );
        }
    }
}

fn complete_task(tasks: &mut Vec<Task>, task_id: u32) {
    if let Some(task) = tasks.iter_mut().find(|task| task.id == task_id) {
        if task.completed {
            println!("Task already completed!");
            process::exit(0);
        }
        task.completed = true;
    } else {
        eprintln!("Task with ID {} not found", task_id);
        process::exit(0);
    }
}
