use sqlx::PgPool;
use std::sync::Arc;
use auth::tokens::TokenManager;
use crate::repository::auth::AuthRepository;

pub struct AppState {
    pub auth_repository: Arc<AuthRepository>,
    pub token_manager: Arc<TokenManager>,
}

impl AppState {
    pub fn new(database_pool: PgPool, token_manager: TokenManager) -> Self {
        let database_pool = Arc::new(database_pool);
        let auth_repository = Arc::new(AuthRepository::new(database_pool));
        let token_manager = Arc::new(token_manager);

        Self { auth_repository, token_manager }
    }
}
