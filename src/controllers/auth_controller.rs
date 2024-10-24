use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::dto::user::User;
use crate::dto::signin_dto::SignInDTO;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use crate::dto::user::Claims;

const SECRET: &[u8] = b"my_secret_key"; // Carregar do .env mais tarde

pub fn mock_users() -> Vec<User> {
    let password = hash("password123", DEFAULT_COST).expect("Failed to hash password");
    vec![User {
        username: "mockuser".to_string(),
        email: "mockuser@example.com".to_string(),
        password,
    }]
}


// Middleware to verify the JWT token
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

#[post("/login")]
async fn login(credentials: web::Json<User>) -> impl Responder {
    let users = mock_users();
    let user = users.into_iter().find(|u| u.email == credentials.email);

    if let Some(user) = user {
        // Verify the password using bcrypt
        if verify(&credentials.password, &user.password).unwrap_or(false) {
            // Create JWT token
            match create_jwt_token(&user.username) {
                Ok(token) => {
                    // Respond with the token in a JSON object
                    let response_body = serde_json::json!({ "auth_token": token });
                    HttpResponse::Ok().json(response_body)
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        } else {
            HttpResponse::Unauthorized().body("Incorrect password")
        }
    } else {
        HttpResponse::Unauthorized().body("User not found")
    }
}

#[post("/sign-in")]
async fn sign_in(credentials: web::Json<User>) -> impl Responder {
    let users = mock_users();
    let user = users.into_iter().find(|u| u.email == credentials.email);

    if let Some(user) = user {
        // Verify the password using bcrypt
        if verify(&credentials.password, &user.password).unwrap_or(false) {
            // Create JWT token
            match create_jwt_token(&user.username) {
                Ok(token) => {
                    // Respond with the token in a JSON object
                    let response_body = serde_json::json!({ "auth_token": token });
                    HttpResponse::Ok().json(response_body)
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        } else {
            HttpResponse::Unauthorized().body("Incorrect password")
        }
    } else {
        HttpResponse::Unauthorized().body("User not found")
    }
}