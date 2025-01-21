use sqlx::PgPool;
use std::sync::Arc;
use crate::repository::auth::AuthRepository;

pub struct AppState {
    pub auth_repository: Arc<AuthRepository>,
    pub jwt_state: JWTState,
}

pub struct JWTState {
    pub jwt_secret: String,
    pub access_token_expiration: u64,
    pub refresh_token_expiration: u64,
}

impl AppState {
    pub fn new(database_pool: PgPool, jwt_state: JWTState) -> Self {
        let database_pool = Arc::new(database_pool);
        let auth_repository = Arc::new(AuthRepository::new(database_pool));

        Self { auth_repository, jwt_state }
    }
}
