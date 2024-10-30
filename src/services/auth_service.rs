use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::services::sign_in_service;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use crate::dto::user::Claims;
use crate::helpers::load_env;

pub async fn verify_jwt(req: actix_web::HttpRequest) -> Result<Claims, Error> {
    let auth_header = req.headers().get("Authorization");
    let SECRET = &load_env("SECRET".to_string());

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
pub fn create_jwt_token(username: &str, id: i32, profile_pic: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let SECRET: &Vec<u8> = &load_env("SECRET".to_string());
    
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize + 60 * 60; // Token valid for 1 hour

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration,
        user_id: id,
        profile_pic: profile_pic.to_string()
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}