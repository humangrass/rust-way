use sqlx::PgPool;
use multitool_hg::rediska::client::Rediska;
use std::sync::Arc;
use crate::models::task::Task;
use crate::repository::task::TaskRepository;

pub struct AppState {
    pub database_pool: Arc<PgPool>,
    pub redis_pool: Arc<Rediska>,
}

impl AppState {
    pub fn new(database_pool: PgPool, redis_pool: Rediska) -> Self {
        Self {
            database_pool: Arc::new(database_pool),
            redis_pool: Arc::new(redis_pool),
        }
    }

    pub async fn create_task(&self, task: Task) -> anyhow::Result<Task> {
        let repo = TaskRepository::new(&self.database_pool);
        // let repo = TaskRepository::new(&self.database_pool, &self.redis_pool);
        repo.create(&task).await
    }
}
