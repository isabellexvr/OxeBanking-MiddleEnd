use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::dto::user::User;
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
#[post("/login")]
async fn login(credentials: web::Json<User>) -> impl Responder {
    let users = mock_users();
    let user = users.into_iter().find(|u| u.email == credentials.email);

    if let Some(user) = user {
        // Verify the password using bcrypt
        if verify(&credentials.password, &user.password).unwrap_or(false) {
            // Create JWT claims
            let expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as usize + 60 * 60; // Token valid for 1 hour

            let claims = Claims {
                sub: user.username.clone(),
                exp: expiration,
            };

            // Sign the token
            match encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)) {
                Ok(token) => HttpResponse::Ok().json(token),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        } else {
            HttpResponse::Unauthorized().body("Invalid password")
        }
    } else {
        HttpResponse::Unauthorized().body("User not found")
    }
}

#[get("/protected")]
async fn protected_route(req: actix_web::HttpRequest) -> impl Responder {
    match verify_jwt(req).await {
        Ok(claims) => HttpResponse::Ok().json(format!("Welcome, {}!", claims.sub)),
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}
