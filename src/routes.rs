use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use crate::controllers::auth_controller;
use crate::services;
use crate::make_request_handler; // Import the request handler
use reqwest::Client;
use crate::Extension;
use tower_http::{
    cors::{Any, CorsLayer},
};
use tower::ServiceBuilder;

pub async fn app() -> Router {
    let client = Client::new(); // Create a new reqwest client

    // Define the CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allows any origin, you can restrict it to specific domains
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST]) // Allowed HTTP methods
        .allow_headers(Any); // Allows any headers, or specify required ones like [http::header::CONTENT_TYPE]

    Router::new()
        .route("/signin", post(auth_controller::sign_in))
        .route(
            "/protected",
            get(services::hello).layer(middleware::from_fn(auth_controller::authorize)),
        )
        .route("/fetch-data", get(make_request_handler)) // New route for fetching data
        .layer(Extension(client)) // Add the client as an extension
        .layer(cors) // Add the CORS layer
}
