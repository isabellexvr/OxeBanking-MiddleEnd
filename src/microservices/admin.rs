use reqwest::Client;
use crate::{dto::new_user_dto::UserDTO, errors::microservices_errors::ParseError};
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::models::user::{User, Address};
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

pub async fn get_user_by_id(id: i32) -> Result<Option<User>, ParseError> {
    // Create an HTTP client
    let client = Client::new();

    // Build the URL using the format! macro
    let url = format!("http://localhost:8081/users/{}", id);

    // Make the GET request to fetch the user
    let response = client.get(&url).send().await.map_err(ParseError::Reqwest)?;

    // Check if the response status indicates success
    if response.status().is_success() {
        // Deserialize the response body into User
        let user: User = response.json().await.map_err(ParseError::Reqwest)?; // Correctly mapping the serde error
        Ok(Some(user)) // User found
    } else {
        Ok(None) // User not found
    }
}


pub async fn get_user_by_cpf(cpf: &str) -> Result<Option<User>, ParseError> {
    let mocked_user = User {
        id: 1,
        full_name: "John Doe".to_string(),
        profile_pic: "https://example.com/profile.jpg".to_string(),
        cpf: "123.456.789-00".to_string(),
        birthdate: "1990-01-01".to_string(),
        marital_status: "Single".to_string(),
        gross_mensal_income: 5000,
        email: "example@gmail.com".to_string(),
        phone_number: "+55 11 99999-9999".to_string(),
        is_admin: false,
        is_blocked: false,
        user_password: "$2b$12$QgGfGwKNepVKIxAaglKLVOqv4CvGKpYxVMERNRltfvLuCvXUVOeRW".to_string(), //senha123
        created_at: "2021-01-01".to_string(),
        updated_at: "2021-01-01".to_string(),
        address: Address {
            id: 1,
            zip_code: "12345-678".to_string(),
            city: "City".to_string(),
            state: "State".to_string(),
            uf: "ST".to_string(),
            street: "Street".to_string(),
            number: "123".to_string(),
            complement: Some("Complement".to_string()),
            is_main: true,
        },
    };

    Ok(Some(mocked_user))
}

pub async fn get_user_addresses(user_id: i32) -> Result<Vec<Address>, ParseError> {
    // Create an HTTP client
    let client = Client::new();

    // Build the URL using the format! macro
    let url = format!("http://localhost:8081/users/{}/addresses", user_id);

    // Make the GET request to fetch the user's addresses
    let response = client.get(&url).send().await.map_err(ParseError::Reqwest)?;

    // Check if the response status indicates success
    if response.status().is_success() {
        // Deserialize the response body into Vec<Address>
        let addresses: Vec<Address> = response.json().await.map_err(ParseError::Reqwest)?;
        Ok(addresses) // Addresses found
    } else {
        Ok(vec![]) // No addresses found
    }
}