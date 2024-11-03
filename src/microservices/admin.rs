use reqwest::Client;
use crate::{dto::new_user_dto::UserDTO, errors::microservices_errors::ParseError};
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::models::user::{User, Address};
use serde_json::from_str;
use sqlx::sqlite::SqlitePool;

pub async fn create_a_new_mocked_user(credentials: web::Json<UserDTO>) -> Result<String, ParseError> {
    let pool = SqlitePool::connect("sqlite://middle-mocked.db").await.unwrap();

    let user = User {
        id: 0, // Assuming id is auto-incremented
        full_name: credentials.full_name.clone(),
        profile_pic: credentials.profile_pic.clone(),
        cpf: credentials.cpf.clone(),
        birthdate: credentials.birthdate.clone(),
        marital_status: credentials.marital_status.clone(),
        gross_mensal_income: credentials.gross_mensal_income,
        email: credentials.email.clone(),
        phone_number: credentials.phone_number.clone(),
        is_admin: credentials.is_admin,
        is_blocked: credentials.is_blocked,
        user_password: credentials.user_password.clone(),
        created_at: chrono::Utc::now().to_string(),
        updated_at: chrono::Utc::now().to_string(),
        address: Address {
            id: 0, // Assuming id is auto-incremented
            zip_code: credentials.address.zip_code.clone(),
            city: credentials.address.city.clone(),
            state: credentials.address.state.clone(),
            uf: credentials.address.uf.clone(),
            street: credentials.address.street.clone(),
            number: credentials.address.number.clone(),
            complement: credentials.address.complement.clone(),
            is_main: credentials.address.is_main,
        },
    };

    let address_id = sqlx::query!(
        r#"
        INSERT INTO addresses (zip_code, city, state, uf, street, number, complement, is_main)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        "#,
        user.address.zip_code,
        user.address.city,
        user.address.state,
        user.address.uf,
        user.address.street,
        user.address.number,
        user.address.complement,
        user.address.is_main,
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .id;

    sqlx::query!(
        r#"
        INSERT INTO users (full_name, profile_pic, cpf, birthdate, marital_status, gross_mensal_income, email, phone_number, is_admin, is_blocked, user_password, created_at, updated_at, address_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        user.full_name,
        user.profile_pic,
        user.cpf,
        user.birthdate,
        user.marital_status,
        user.gross_mensal_income,
        user.email,
        user.phone_number,
        user.is_admin,
        user.is_blocked,
        user.user_password,
        user.created_at,
        user.updated_at,
        address_id
    )
    .execute(&pool)
    .await
    .unwrap();

    Ok("User created successfully".to_string())
}

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