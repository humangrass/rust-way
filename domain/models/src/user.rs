use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            password_hash: user.password_hash,
        }
    }
}

impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            password_hash: model.password_hash,
        }
    }
}
