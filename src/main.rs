use actix_cors::Cors;
use actix_web::{App, HttpServer, HttpResponse, Responder, web};
use actix_web::http::header;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::sqlite::SqlitePool;

use crate::controllers::auth_controller::sign_in;
use crate::controllers::api_controller::call_external;
use crate::controllers::users_controller::{sign_up, get_user_info, get_account_info};
use crate::controllers::payments_controller::create_payment;
use crate::controllers::insurances_controller::{get_all_insurances_controller, get_all_mocked_insurances_controller};
use crate::middleware::auth_middleware::Auth;

mod controllers;
mod services;
mod dto;
mod routes;
mod middleware;
mod microservices;
mod models;
mod errors;
mod helpers;
mod schema;
mod repositories;

// Health check route
async fn health() -> impl Responder {
    HttpResponse::Ok().json("API is healthy!")
}

fn configure_app(cfg: &mut web::ServiceConfig) {
    // Apenas registra as rotas
    cfg.service(sign_in);
    cfg.service(sign_up);
    cfg.service(call_external);
    cfg.service(
        web::scope("/payments")
            .wrap(Auth)
            .service(create_payment),
    );
    cfg.service(
        web::scope("/auth")
            .wrap(Auth)
            .service(get_user_info)
            .service(get_account_info)
    );
    cfg.service(
        web::scope("/insurances")
            .wrap(Auth)
            .service(get_all_insurances_controller)
            .service(get_all_mocked_insurances_controller)
    );
    cfg.route("/", web::get().to(health));
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let pool = SqlitePool::connect("sqlite://middle-mocked.db").await.unwrap();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .configure(configure_app)
    })
    .bind(("0.0.0.0", 4200))?
    .run()
    .await

}
