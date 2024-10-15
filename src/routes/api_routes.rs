use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(crate::controllers::api_controller::call_external);
}
