use sqlx::PgPool;
use std::sync::Arc;
use crate::entities::user::UserModel;

pub struct AuthRepository {
    pool: Arc<PgPool>,
}

#[derive(Debug)]
pub enum AuthRepositoryError {
    UserAlreadyExists,
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
                Err(AuthRepositoryError::DatabaseError(e))
            }
        }
    }
}
