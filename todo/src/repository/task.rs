use crate::models::task::Task;
use anyhow::Result;
use sqlx::PgPool;

pub struct TaskRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TaskRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
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
        .fetch_one(self.pool)
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
}
