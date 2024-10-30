use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::services::sign_in_service;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use crate::dto::user::Claims;
use crate::models::user::User;
use crate::dto::signin_dto::SignInDTO;


#[post("/sign-in")]
async fn sign_in(credentials: web::Json<SignInDTO>) -> impl Responder {
    let user_result = sign_in_service::sign_in_service(actix_web::web::Json(credentials.clone())).await;

    match user_result {
        Ok(user) => {
            // If user is found and password is verified, return user as JSON
            HttpResponse::Ok().json(user)
        }
        Err(err) => {
            // Handle the error appropriately
            HttpResponse::InternalServerError().body(format!("Error: {:?}", err))
        }
    }
}