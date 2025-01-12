use crate::models::status::Status;
use crate::storage::FILE_NAME;
use chrono::{Local, NaiveDate};
use clap::ArgMatches;
use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDate,
    pub status: Status,
}

impl Task {
    pub fn new(args: &ArgMatches) {
        let mut tasks = Task::load_tasks();

        let id = tasks.last().map_or(1, |t| t.id + 1);

        let title = args
            .get_one::<String>("title")
            .expect("Title is required")
            .to_string();

        let description = args.get_one::<String>("description").map(|s| s.to_string());

        let date = args
            .get_one::<NaiveDate>("date")
            .copied()
            .unwrap_or_else(|| Local::now().date_naive());

        let task = Task {
            id,
            title,
            description,
            date,
            status: Status::Pending,
        };

        let mut table = Self::get_table_template();
        task.show_in_table(&mut table);
        table.printstd();

        tasks.push(task);
        Task::save_tasks(&tasks);
    }

    pub fn start(args: &ArgMatches) {
        // it's OK
        let id = args.get_one::<u32>("id").expect("id is required");
        let mut tasks = Self::load_tasks();
        // TODO: не слишком оптимально. Лучше через HashMap и переписать формат данных в файле
        if let Some(task) = tasks.iter_mut().find(|task| task.id == *id) {
            task.status = Status::InProgress;
        } else {
            println!("Task with ID {} not found.", id);
        }

        Self::save_tasks(&tasks);
    }

    fn load_tasks() -> Vec<Task> {
        if let Ok(mut file) = File::open(FILE_NAME) {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_ok() {
                // it's OK
                serde_json::from_str(&content).unwrap_or_default()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    fn save_tasks(tasks: &Vec<Task>) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(FILE_NAME)
        {
            // it's OK
            let content = serde_json::to_string_pretty(tasks).expect("could not serialize tasks");
            file.write_all(content.as_bytes())
                .expect("could not write tasks into file");
        }
    }

    fn get_table_template() -> Table {
        let mut table = Table::new();

        table.add_row(Row::new(vec![
            Cell::new("ID"),
            Cell::new("Title"),
            Cell::new("Description"),
            Cell::new("Date"),
            Cell::new("Status"),
        ]));

        table
    }

    fn show_in_table(&self, table: &mut Table) {
        table.add_row(Row::new(vec![
            Cell::new(&self.id.to_string()),
            Cell::new(&self.title),
            Cell::new(self.description.as_deref().unwrap_or_default()),
            Cell::new(&self.date.to_string()),
            Cell::new(&format!("{:?}", &self.status)),
        ]));
    }

    pub fn show_list() {
        let tasks = Task::load_tasks();

        if tasks.is_empty() {
            println!("Tasks not found.");
            return;
        }

        let mut table = Self::get_table_template();

        for task in tasks {
            task.show_in_table(&mut table);
        }

        table.printstd();
    }
}
