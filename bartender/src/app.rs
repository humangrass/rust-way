use std::sync::Arc;
use sqlx::PgPool;

pub struct AppState {
}

impl AppState {
    pub fn new(database_pool: PgPool) -> Self {

        let _database_pool = Arc::new(database_pool);

        Self {}
    }
}
