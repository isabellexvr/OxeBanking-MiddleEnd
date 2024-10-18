use actix_cors::Cors;
use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next}, Error,
};
use actix_web::http::header;
use actix_web::web;
use dotenv::dotenv;
use env_logger::Env;

use crate::controllers::auth_controller::login;
use crate::controllers::api_controller::call_external;
use crate::controllers::auth_controller::protected_route;
use crate::controllers::users_controller::sign_up;
use crate::middleware::auth_middleware::SayHi;

mod controllers;
mod services;
mod dto;
mod routes;
mod middleware;
mod microservices;

// Health check route
async fn health() -> impl Responder {
    HttpResponse::Ok().json("API is healthy!")
}

async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    next.call(req).await
    // post-processing
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            // CORS setup to allow requests from any origin
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .service(login)   // Login route for signing JWT
            .service(sign_up)
            .service(call_external)  // Route to call external API
            .service(protected_route)  // Protected route
            .route("/", web::get().to(health).wrap(SayHi))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
