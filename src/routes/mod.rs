use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(crate::controllers::auth_controller::login)
       .service(crate::controllers::api_controller::call_external);
}
