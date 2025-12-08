use actix_web::web;

use crate::handlers::users::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_users))
       .route("", web::post().to(create_user));
}