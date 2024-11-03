use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpServer, HttpResponse, Responder, web};
use actix_web::dev::ServiceRequest;
use actix_web::http::header;
use dotenv::dotenv;
use env_logger::Env;
use std::future::{ready, Ready};
use rusqlite::{params, Connection, Result};
use serde::Serialize;
use sqlx::sqlite::SqlitePool;


use crate::controllers::auth_controller::sign_in;
use crate::controllers::api_controller::call_external;
use crate::controllers::users_controller::sign_up;
use crate::controllers::payments_controller::create_payment;
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

struct RouteLogger;

impl<S, B> actix_service::Transform<S, ServiceRequest> for RouteLogger
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error> + 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Transform = RouteLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RouteLoggerMiddleware { service }))
    }
}

struct RouteLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RouteLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error> + 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Rota disponível: {} {}", req.method(), req.path());
        self.service.call(req)
    }
}

// Health check route
async fn health() -> impl Responder {
    HttpResponse::Ok().json("API is healthy!")
}

fn print_routes() {
    println!("Rota disponível: POST /sign_in");
    println!("Rota disponível: POST /sign_up");
    println!("Rota disponível: GET /call_external");
    println!("Rota disponível: POST /payments/create_payment");
    println!("Rota disponível: GET /");
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
    cfg.route("/", web::get().to(health));
}


/* #[derive(Serialize)]
struct Test {
    id: i64,
    name: String,
    email: String,
}

async fn get_users(pool: web::Data<SqlitePool>) -> impl Responder {
    let users = sqlx::query_as!(Test, "SELECT id, name, email FROM test")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(users)
} */


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let pool = SqlitePool::connect("sqlite://middle-mocked.db").await.unwrap();
    print_routes();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(RouteLogger)
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
    .bind(("0.0.0.0", 3000))?
    .run()
    .await

    

}
