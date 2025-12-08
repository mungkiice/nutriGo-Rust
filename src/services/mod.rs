use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::services::user_service::UserService;

pub mod user_service;

#[derive(Clone)]
pub struct AppServices {
    pub user: Arc<UserService>,
}

impl AppServices {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            user: Arc::new(UserService::new(pool)),
        }
    }
}