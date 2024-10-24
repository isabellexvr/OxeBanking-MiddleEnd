use reqwest::Client;
use crate::{dto::new_user_dto::UserDTO, errors::login_errors::ParseError};
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::models::user::User;
use serde_json::from_str;

pub async fn create_a_new_user(credentials: web::Json<UserDTO>) -> Result<String, ParseError> {
    // Create an HTTP client
    let client = Client::new();

    // Make the POST request to create a new user
    let response = client
        .post("http://localhost:8081/users") // API endpoint
        .json(&credentials) // Send credentials as JSON in the body
        .send()
        .await
        .map_err(ParseError::Reqwest)?; // Convert reqwest error to ParseError

    // Get the response body as text
    let text = response.text().await.map_err(ParseError::Reqwest)?; // Convert reqwest error to ParseError

    Ok(text) // Return the response text
}


pub async fn get_all_users() -> Result<Vec<User>, ParseError> {
    // Create an HTTP client
    let client = Client::new();

    // Make the GET request to fetch users
    let response = client
        .get("http://localhost:8081/users") // API endpoint
        .send()
        .await
        .map_err(ParseError::Reqwest)?; // Convert reqwest error to ParseError

    let body = response.text().await.map_err(ParseError::Reqwest)?; // Convert reqwest error to ParseError

    let users: Vec<User> = from_str(&body).map_err(ParseError::Serde)?; // Convert serde error to ParseError

    Ok(users)
}

pub async fn get_user_by_id(id: i32) -> Result<User, ParseError> {
    // Create an HTTP client
    let client = Client::new();

    // Build the URL using the format! macro
    let url = format!("http://localhost:8081/users/{}", id);

    // Make the GET request to fetch the user
    let response = client.get(&url).send().await?;

    // Deserialize the response body into User
    let user: User = response.json().await?;

    Ok(user)
}
