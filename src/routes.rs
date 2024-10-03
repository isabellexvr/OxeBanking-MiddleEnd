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

pub async fn app() -> Router {
    let client = Client::new(); // Create a new reqwest client

    Router::new()
        .route("/signin", post(auth_controller::sign_in))
        .route(
            "/protected",
            get(services::hello).layer(middleware::from_fn(auth_controller::authorize)),
        )
        .route("/fetch-data", get(make_request_handler)) // New route for fetching data
        .layer(Extension(client)) // Add the client as an extension
}
