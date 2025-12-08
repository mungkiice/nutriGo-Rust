use actix_web::web;

pub mod users;
// pub mod auth;


pub fn configure(_cfg: &mut web::ServiceConfig) {
    _cfg
        .service(web::scope("/users").configure(users::routes));
}