use chrono::{NaiveDate, NaiveTime};
use clap::ValueEnum;

pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub due_date: Option<NaiveDate>,
    pub status: Status,
}

pub struct TaskSlot {
    pub task_id: u32,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}

pub enum Status {
    Pending,
    Completed,
}
