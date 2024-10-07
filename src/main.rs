use actix_cors::Cors;
use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use actix_web::http::header;

// Health check route
#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json("API is healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // CORS setup to allow requests from any origin
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .service(health)  // Register the health route
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
