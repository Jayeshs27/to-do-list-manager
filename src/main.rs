use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

const FILE_PATH: &str = "database/tasks.json";

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    fn load() -> Self {
        if Path::new(FILE_PATH).exists() {
            let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&data).unwrap_or_else(|_| TaskList { tasks: Vec::new() })
        } else {
            TaskList { tasks: Vec::new() }
        }
    }

    fn save(&self) {
        let data = serde_json::to_string_pretty(&self).expect("Failed to serialize");
        fs::write(FILE_PATH, data).expect("Failed to write file");
    }

    fn add_task(&mut self, description: String) {
        let id = self.tasks.len() + 1;
        self.tasks.push(Task { id, description });
        self.save();
        println!("Task added: {}", id);
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            for task in &self.tasks {
                println!("{}. {}", task.id, task.description);
            }
        }
    }

    fn remove_task(&mut self, id: usize) {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            self.save();
            println!("Task {} removed.", id);
        } else {
            println!("Task not found.");
        }
    }
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Simple CLI To-Do List", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Remove { id: usize },
}

fn main() {
    let cli = Cli::parse();
    let mut task_list = TaskList::load();

    match cli.command {
        Commands::Add { description } => task_list.add_task(description),
        Commands::List => task_list.list_tasks(),
        Commands::Remove { id } => task_list.remove_task(id),
    }
}
