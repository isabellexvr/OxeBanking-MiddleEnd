use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder, Error};
use actix_web::http::header;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::Client;

// Secret key for signing and verifying JWT tokens
const SECRET: &[u8] = b"my_secret_key"; // goes to .env later

// Mock user data with encrypted password
#[derive(Serialize, Deserialize, Clone)]
struct User {
    username: String,
    email: String,
    password: String, // This will hold the hashed password
}

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Mock database of users (for demonstration purposes)
fn mock_users() -> Vec<User> {
    let password = hash("password123", DEFAULT_COST).expect("Failed to hash password");
    vec![User {
        username: "mockuser".to_string(),
        email: "mockuser@example.com".to_string(),
        password,
    }]
}

async fn call_external_api() -> Result<String, reqwest::Error> {
    // Create an HTTP client
    let client = Client::new();

    // Make a GET request to the external API
    let response = client.get("http://localhost:3000/hello")
        .send()
        .await?;

    // Get the response text
    let text = response.text().await?;
    Ok(text)
}

// Route to call the external API
#[get("/external")]
async fn call_external() -> Result<HttpResponse, Error> {
    match call_external_api().await {
        Ok(response) => Ok(HttpResponse::Ok().body(response)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Failed to call external API")),
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

// Protected route that requires a valid JWT token
#[get("/protected")]
async fn protected_route(req: actix_web::HttpRequest) -> impl Responder {
    match verify_jwt(req).await {
        Ok(claims) => HttpResponse::Ok().json(format!("Welcome, {}!", claims.sub)),
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}

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
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .service(health)  // Health check
            .service(login)   // Login route for signing JWT
            .service(protected_route)  // Protected route
            .service(call_external)  // Route to call external API
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
