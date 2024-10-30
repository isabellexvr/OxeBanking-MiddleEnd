use actix_cors::Cors;
use actix_web::{ App, HttpServer, HttpResponse, Responder};
use actix_web::http::header;
use actix_web::web;
use dotenv::dotenv;
use env_logger::Env;

use crate::controllers::auth_controller::sign_in;
use crate::controllers::api_controller::call_external;
use crate::controllers::users_controller::sign_up;
use crate::{controllers::payments_controller::create_payment};
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


// Health check route
async fn health() -> impl Responder {
    HttpResponse::Ok().json("API is healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .service(sign_in)  // Rota de login para geração de JWT
            .service(sign_up)
            .service(call_external)  // Rota para chamada de API externa
            .service(
                web::scope("/payments")  
                    .wrap(Auth)
                    .service(create_payment)
            )
            .route("/", web::get().to(health))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
