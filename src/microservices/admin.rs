use reqwest::Client;
use crate::dto::new_user_dto::UserDTO;
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use serde::Serialize;

// Your existing UserDTO and Address structs here...


pub async fn call_admin_microservice(credentials: web::Json<UserDTO>) -> Result<String, reqwest::Error> {
    // Create an HTTP client
    let client = Client::new();

    // Create the UserRequest struct from UserDTO

    let response = client
        .post("http://localhost:8081/users") // API endpoint
        .json(&credentials) // Send user_request as JSON in the body
        .send()
        .await;

    // Log details about the response or error
    match response {
        Ok(resp) => {
            let text = resp.text().await?;
            Ok(text)
        },
        Err(err) => {
            println!("Erro ao fazer requisição: {:?}", err);
            Err(err)
        }
    }
}
