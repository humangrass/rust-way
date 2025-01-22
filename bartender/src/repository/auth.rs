use crate::entities::user::UserModel;
use log::error;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthRepository {
    pool: Arc<PgPool>,
}

#[derive(Debug)]
pub enum AuthRepositoryError {
    UserAlreadyExists,
    UserNotFound,
    #[allow(dead_code)] // Warning field `0` is never read: isn't true.
    DatabaseError(sqlx::Error),
}

impl AuthRepositoryError {
    pub fn is_user_already_exists(&self) -> bool {
        matches!(self, AuthRepositoryError::UserAlreadyExists)
    }
}

impl AuthRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        AuthRepository { pool }
    }

    pub async fn create(&self, model: &UserModel) -> Result<(), AuthRepositoryError> {
        let query = sqlx::query!(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
            model.id,
            model.username,
            model.email,
            model.password_hash,
        );

        match query.execute(&*self.pool).await {
            Ok(_) => Ok(()),
            Err(e) => {
                if let Some(db_error) = e.as_database_error() {
                    if let Some(constraint) = db_error.constraint() {
                        if constraint == "users_email_key" || constraint == "users_username_key" {
                            return Err(AuthRepositoryError::UserAlreadyExists);
                        }
                    }
                }
                error!("Database error: {}", e);
                Err(AuthRepositoryError::DatabaseError(e))
            }
        }
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<UserModel, AuthRepositoryError> {
        let query = sqlx::query_as!(
            UserModel,
            "SELECT id, username, email, password_hash FROM users WHERE username = $1",
            username
        );
        // TODO: Refactoring is needed, but I don't understand how to take out the common part.
        match query.fetch_optional(&*self.pool).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(AuthRepositoryError::UserNotFound),
            Err(e) => {
                error!("Database error: {}", e);
                Err(AuthRepositoryError::DatabaseError(e))
            }
        }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<UserModel, AuthRepositoryError> {
        let query = sqlx::query_as!(
            UserModel,
            "SELECT id, username, email, password_hash FROM users WHERE id = $1",
            id
        );
        // TODO: Refactoring is needed, but I don't understand how to take out the common part.
        match query.fetch_optional(&*self.pool).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(AuthRepositoryError::UserNotFound),
            Err(e) => {
                error!("Database error: {}", e);
                Err(AuthRepositoryError::DatabaseError(e))
            }
        }
    }
}
