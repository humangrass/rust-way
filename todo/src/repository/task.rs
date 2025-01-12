use crate::models::task::TaskModel;
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

    pub async fn create(&self, model: &TaskModel) -> Result<TaskModel> {
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
            model.title,
            model.description,
            model.status,
            model.starts_at,
            model.ends_at,
        )
        .fetch_one(&*self.pool)
        .await?;

        let created_at = row.created_at.unwrap();
        let updated_at = row.updated_at.unwrap();

        Ok(TaskModel {
            id: row.id,
            title: row.title.to_string(),
            description: row.description.to_string(),
            status: row.status,
            starts_at: model.starts_at,
            ends_at: Default::default(),
            created_at,
            updated_at,
        })
    }

    pub async fn list(&self) -> Result<Vec<TaskModel>> {
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
        let models = sqlx::query_as_unchecked!(
            TaskModel,
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

        Ok(models)
    }

    pub async fn by_id(&self, id: i32) -> Result<Option<TaskModel>, sqlx::Error> {
        // TODO: IDK... The trait `From<std::option::Option<chrono::DateTime<Utc>>>` is not implemented for `chrono::DateTime<Utc>`, which is required by `std::option::Option<chrono::DateTime<Utc>>: Into<_>`
        sqlx::query_as_unchecked!(
            TaskModel,
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
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn update(
        &self,
        id: i32,
        model: &TaskModel,
    ) -> Result<Option<TaskModel>, sqlx::Error> {
        // TODO: query_as_unchecked: same error
        let updated_task = sqlx::query_as_unchecked!(
            TaskModel,
            r#"
            UPDATE tasks
            SET
                title = $1,
                description = $2,
                status = $3,
                starts_at = $4,
                ends_at = $5,
                updated_at = NOW()
            WHERE id = $6
            RETURNING id, title, description, status, starts_at, ends_at, created_at, updated_at
            "#,
            model.title,
            model.description,
            model.status,
            model.starts_at,
            model.ends_at,
            id
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(updated_task)
    }
}
