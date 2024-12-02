use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    content: String,
    completed: bool,
}

#[derive(Parser)]
#[command(name = "ToDo CLI")]
#[command(author = "Himanshu Rawat")]
#[command(version = "1.0")]
#[command(about = "Manage your tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a new task
    Add { task: String },
    /// Lists all tasks
    List,
    /// Marks a task as complete
    Complete { id: u32 },
}

fn main() {
    let cli = Cli::parse();

    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());

    match cli.command {
        Commands::Add { task } => {
            add_task(&mut tasks, task);
            save_tasks(&tasks).expect("Failed to save tasks");
            println!("Task added!");
        }
        Commands::List => list_tasks(&tasks),
        Commands::Complete { id } => {
            mark_task_complete(&mut tasks, id);
            save_tasks(&tasks).expect("Failed to save tasks");
            println!("Task marked as completed!");
        }
    }
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let data = serde_json::to_string(tasks)?;
    fs::write("tasks.json", data)
}

fn load_tasks() -> io::Result<Vec<Task>> {
    if let Ok(data) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Ok(Vec::new())
    }
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

fn mark_task_complete(tasks: &mut Vec<Task>, task_id: u32) {
    if let Some(task) = tasks.iter_mut().find(|task| task.id == task_id) {
        task.completed = true;
    } else {
        eprintln!("Task with ID {} not found", task_id);
    }
}
