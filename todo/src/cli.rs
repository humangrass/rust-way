use chrono::NaiveDate;
use clap::{Args, Command, Subcommand};

pub const COMMAND_ADD: &str = "add";
pub const COMMAND_START: &str = "start";
pub const COMMAND_DELETE: &str = "delete";
pub const COMMAND_LIST: &str = "list";
pub const COMMAND_EDIT: &str = "edit";
pub const COMMAND_COMPLETE: &str = "complete";

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Adds a new task
    Add(AddTask),

    /// Start the task
    Start(TaskId),

    /// Deletes a task by ID
    Delete(TaskId),

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

    /// Date of the task (format: YYYY-MM-DD)
    #[arg(long)]
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Args)]
pub struct TaskId {
    /// ID of the task
    #[arg(long)]
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct ListTasks {
    /// Filter tasks by criteria
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

    /// New date for the task (format: YYYY-MM-DD)
    #[arg(long)]
    pub date: Option<NaiveDate>,
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
        .subcommand(AddTask::augment_args(
            Command::new(COMMAND_ADD).about("Add a new task"),
        ))
        .subcommand(TaskId::augment_args(
            Command::new(COMMAND_START).about("Start a task"),
        ))
        .subcommand(TaskId::augment_args(
            Command::new(COMMAND_DELETE).about("Delete a task by ID"),
        ))
        .subcommand(ListTasks::augment_args(
            Command::new(COMMAND_LIST).about("List tasks with optional filtering"),
        ))
        .subcommand(EditTask::augment_args(
            Command::new(COMMAND_EDIT).about("Edit an existing task"),
        ))
        .subcommand(CompleteTask::augment_args(
            Command::new(COMMAND_COMPLETE).about("Mark a task as completed"),
        ))
}
