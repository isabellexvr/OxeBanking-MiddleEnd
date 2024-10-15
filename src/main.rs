use actix_cors::Cors;
use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use actix_web::http::header;
use crate::controllers::auth_controller::login;
use crate::controllers::api_controller::call_external;
use crate::controllers::auth_controller::protected_route;
use env_logger::Env;
use log::info;

mod controllers;
mod services;
mod dto;
mod routes;

// Health check route
#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json("API is healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .service(health)  // Health check
            .service(login)   // Login route for signing JWT
            .service(protected_route)  // Protected route
            .service(call_external)  // Route to call external API
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
