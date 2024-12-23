use chrono::NaiveDate;
use clap::{Args, Command, Subcommand};
use crate::models::Priority;

#[derive(Debug)]
pub struct Cli {
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Adds a new task
    Add(AddTask),

    /// Deletes a task by ID
    Delete(DeleteTask),

    /// Lists tasks (optionally filtered)
    List(ListTasks),

    /// Edits an existing task
    Edit(EditTask),

    /// Marks a task as completed
    Complete(CompleteTask),
}

#[derive(Debug, Args)]
pub struct AddTask {
    /// Title of the task
    #[arg(long)]
    pub title: String,

    /// Description of the task
    #[arg(long)]
    pub description: Option<String>,

    /// Priority of the task (low, medium, high)
    #[arg(long)]
    pub priority: Option<Priority>,

    /// Due date of the task (format: YYYY-MM-DD)
    #[arg(long)]
    pub due: Option<NaiveDate>,
}

#[derive(Debug, Args)]
pub struct DeleteTask {
    /// ID of the task to delete
    #[arg(long)]
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct ListTasks {
    /// Filter tasks by criteria (e.g., priority=high)
    #[arg(long)]
    pub filter: Option<String>,
}

#[derive(Debug, Args)]
pub struct EditTask {
    /// ID of the task to edit
    #[arg(long)]
    pub id: u32,

    /// New title for the task
    #[arg(long)]
    pub title: Option<String>,

    /// New description for the task
    #[arg(long)]
    pub description: Option<String>,

    /// New priority for the task
    #[arg(long)]
    pub priority: Option<Priority>,

    /// New due date for the task (format: YYYY-MM-DD)
    #[arg(long)]
    pub due: Option<NaiveDate>,
}

#[derive(Debug, Args)]
pub struct CompleteTask {
    /// ID of the task to mark as complete
    #[arg(long)]
    pub id: u32,
}

pub fn setup_cli() -> Command {
    Command::new("todo")
        .about("A CLI-based TODO list manager on Rust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(AddTask::augment_args(Command::new("add").about("Add a new task")))
        .subcommand(DeleteTask::augment_args(Command::new("delete").about("Delete a task by ID")))
        .subcommand(ListTasks::augment_args(Command::new("list").about("List tasks with optional filtering")))
        .subcommand(EditTask::augment_args(Command::new("edit").about("Edit an existing task")))
        .subcommand(CompleteTask::augment_args(Command::new("complete").about("Mark a task as completed")))
}
