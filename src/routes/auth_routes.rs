use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(crate::controllers::auth_controller::login);
}
