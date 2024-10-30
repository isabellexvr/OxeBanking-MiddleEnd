use actix_web::{get, post, web, HttpResponse, Responder, Error};
use thiserror::Error;
use crate::services::sign_in_service;
use crate::dto::signin_dto::SignInDTO;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct AuthResponse {
    auth_token: String,
}

#[post("/sign-in")]
async fn sign_in(credentials: web::Json<SignInDTO>) -> impl Responder {
    let user_result = sign_in_service::sign_in_service(actix_web::web::Json(credentials.clone())).await;

    match user_result {
        Ok(user) => {
            // If user is found and password is verified, return user as JSON
            let response = AuthResponse {
                auth_token: user,
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            // Handle the error appropriately
            HttpResponse::InternalServerError().body(format!("Error: {:?}", err))
        }
    }
}