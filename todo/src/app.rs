use crate::repository::task::TaskRepository;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub task_repository: Arc<TaskRepository>,
}

impl AppState {
    pub fn new(database_pool: PgPool) -> Self {
        let database_pool = Arc::new(database_pool);
        let task_repository = Arc::new(TaskRepository::new(database_pool));

        Self {
            task_repository,
        }
    }
}
