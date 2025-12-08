use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}

#[derive(sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub name: String,
}

impl From<UserModel> for UserResponse {
    fn from(m: UserModel) -> Self {
        Self {
            id: m.id,
            email: m.email,
            name: m.name,
        }
    }
}