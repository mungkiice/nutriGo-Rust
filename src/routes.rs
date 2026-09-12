use actix_web::web;

pub mod users_routes {
    use actix_web::web;
    use crate::handlers::users::*;

    pub fn routes(cfg: &mut web::ServiceConfig) {
        cfg.route("", web::get().to(get_users))
           .route("", web::post().to(create_user));
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").configure(users_routes::routes));
}
