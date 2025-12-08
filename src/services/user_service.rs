use sqlx::PgPool;
use crate::repositories::user_repositories::UserRepository;
use crate::models::user::{CreateUserRequest, UserResponse};
use crate::errors::ApiError;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: UserRepository::new(pool),
        }
    }

    pub async fn create_user(
        &self,
        data: CreateUserRequest,
    ) -> Result<UserResponse, ApiError> {
        log::debug!("UserService: Creating user with email: {}", data.email);
        
        if data.email.is_empty() {
            log::warn!("UserService: Email validation failed - email is empty");
            return Err(ApiError::BadRequest("Email required".into()));
        }

        match self.repo.insert(&data).await {
            Ok(user) => {
                log::info!("UserService: User created successfully: {}", user.id);
                Ok(user.into())
            }
            Err(e) => {
                log::error!("UserService: Database error while creating user: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn get_all(&self) -> Result<Vec<UserResponse>, ApiError> {
        log::debug!("UserService: Fetching all users");
        
        match self.repo.get_all().await {
            Ok(users) => {
                log::info!("UserService: Retrieved {} users from database", users.len());
                Ok(users.into_iter().map(|u| u.into()).collect())
            }
            Err(e) => {
                log::error!("UserService: Database error while fetching users: {:?}", e);
                Err(e)
            }
        }
    }
}