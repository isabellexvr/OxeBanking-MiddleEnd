use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::services::sign_in_service;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use crate::dto::user::Claims;
use crate::models::user::User;

async fn verify_jwt(req: actix_web::HttpRequest) -> Result<Claims, Error> {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(token) = header_value.to_str() {
            let token = token.trim_start_matches("Bearer ");
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(SECRET),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(token_data) => Ok(token_data.claims),
                Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid Token")),
            }
        } else {
            Err(actix_web::error::ErrorUnauthorized("Invalid Authorization Header"))
        }
    } else {
        Err(actix_web::error::ErrorUnauthorized("Missing Authorization Header"))
    }
}



// Route to sign a JWT if the credentials are valid
fn create_jwt_token(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize + 60 * 60; // Token valid for 1 hour

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}