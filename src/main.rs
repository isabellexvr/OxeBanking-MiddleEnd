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
use crate::controllers::credit_card_controllers::{request_new_credit_card, get_credit_card_info, update_credit_card_limit_route};
use crate::controllers::consortia_controller::{get_all_consortia_controller, post_new_consortium_controller, add_participant_controller, get_consortium_participants_controller, get_consortium_by_user_id_controller, delete_consortium_controller, contemplate_consortium_controller};

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
    cfg.service(
        web::scope("/credit-card")
            .wrap(Auth)
            .service(request_new_credit_card)
            .service(get_credit_card_info)
            .service(update_credit_card_limit_route)
    );
    cfg.service(
        web::scope("/consortia")
            .wrap(Auth)
            .service(get_all_consortia_controller)
            .service(post_new_consortium_controller)
            .service(add_participant_controller)
            .service(get_consortium_participants_controller)
            .service(get_consortium_by_user_id_controller)
            .service(delete_consortium_controller)
            .service(contemplate_consortium_controller)

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
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
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
