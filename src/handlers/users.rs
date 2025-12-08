use actix_web::{web, HttpResponse};
use crate::services::AppServices;
use crate::models::user::{CreateUserRequest};

pub async fn create_user(
    services: web::Data<AppServices>,
    payload: web::Json<CreateUserRequest>,
) -> actix_web::Result<HttpResponse> {
    log::debug!("Received request to create user: {}", payload.email);
    
    match services.user.create_user(payload.into_inner()).await {
        Ok(user) => {
            log::info!("User created successfully: {}", user.id);
            Ok(HttpResponse::Ok().json(user))
        }
        Err(e) => {
            log::error!("Failed to create user: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn get_users(
    services: web::Data<AppServices>,
) -> actix_web::Result<HttpResponse> {
    log::debug!("Received request to get all users");
    
    match services.user.get_all().await {
        Ok(result) => {
            log::info!("Retrieved {} users", result.len());
            Ok(HttpResponse::Ok().json(result))
        }
        Err(e) => {
            log::error!("Failed to retrieve users: {:?}", e);
            Err(e.into())
        }
    }
}