use crate::repository::task::TaskRepository;
use auth::tokens::TokenManager;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub task_repository: Arc<TaskRepository>,
    pub token_manager: Arc<TokenManager>,
}

impl AppState {
    pub fn new(database_pool: PgPool, token_manager: TokenManager) -> Self {
        // TODO: _redis_pool

        let database_pool = Arc::new(database_pool);
        let task_repository = Arc::new(TaskRepository::new(database_pool));
        let token_manager = Arc::new(token_manager);

        Self {
            task_repository,
            token_manager,
        }
    }
}
