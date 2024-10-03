use axum::{
    routing::get,
    response::Json,
    Router,
};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::Request;
use axum::Extension;
use std::future::{IntoFuture, Future};
use reqwest::StatusCode;

pub mod controllers;
mod routes;
mod services;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    data: String,
}

pub async fn make_request_handler(Extension(client): Extension<Client>) -> Result<Json<ApiResponse>, (StatusCode, String)> {
    // Make a request to another API
    let res = client
        .get("http://127.0.0.1:4000/hello")
        .send()
        .await
        .map_err(|e| {
            eprintln!("Error fetching data: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    // Parse the response as JSON
    let api_response: ApiResponse = res.json().await.map_err(|e| {
        eprintln!("Error parsing JSON: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Json(api_response)) // Return the Json wrapped in Ok
}

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to conne to connect to the server");

    println!("Listening on {}", listener.local_addr().unwrap() );

    let app = routes::app().await;

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
