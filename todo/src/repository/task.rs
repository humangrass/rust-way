use crate::models::task::Task;
use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;

pub struct TaskRepository {
    pool: Arc<PgPool>,
}

impl TaskRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, task: &Task) -> Result<Task> {
        let row = sqlx::query!(
            r#"
            INSERT INTO tasks (title, description, status, starts_at, ends_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                title,
                description,
                status,
                starts_at,
                ends_at,
                created_at,
                updated_at
            "#,
            task.title,
            task.description,
            task.status,
            task.starts_at,
            task.ends_at,
        )
        .fetch_one(&*self.pool)
        .await?;

        let created_at = row.created_at.unwrap();
        let updated_at = row.updated_at.unwrap();

        let new_task = Task {
            id: row.id as i64,
            title: row.title.to_string(),
            description: row.description.to_string(),
            status: row.status,
            starts_at: task.starts_at,
            ends_at: Default::default(),
            created_at,
            updated_at,
        };

        Ok(new_task)
    }

    pub async fn list(&self) -> Result<Vec<Task>> {
        // TODO: IDK... The trait `From<std::option::Option<chrono::DateTime<Utc>>>` is not implemented for `chrono::DateTime<Utc>`, which is required by `std::option::Option<chrono::DateTime<Utc>>: Into<_>`
        // use chrono::{DateTime, TimeZone, Utc};
        // let tasks = sqlx::query_as!(
        //     Task,
        //     r#"
        //     SELECT
        //         id,
        //         title,
        //         description,
        //         status,
        //         starts_at,
        //         ends_at AS "ends_at: Option<DateTime<Utc>>",
        //         created_at,
        //         updated_at
        //     FROM tasks
        //     "#
        // )
        // .fetch_all(&*self.pool)
        // .await?;

        let tasks = sqlx::query_as_unchecked!(
            Task,
            r#"
            SELECT
                id,
                title,
                description,
                status,
                starts_at,
                ends_at,
                created_at,
                updated_at
            FROM tasks
            "#
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(tasks)
    }

    pub async fn by_id(&self, task_id: i32) -> Result<Task> {
        todo!()
    }

    pub async fn delete(&self, task_id: i32) -> Result<Task> {
        todo!()
    }

    pub async fn edit(&self, task_id: i32, task: &Task) -> Result<Task> {
        todo!()
    }
}
