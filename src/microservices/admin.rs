use reqwest::Client;
use crate::dto::new_user_dto::UserDTO;
use actix_web::{get, post, web, HttpResponse, Responder, Error};

pub async fn call_admin_microservice(credentials: web::Json<UserDTO>) -> Result<String, reqwest::Error> {
    // Create an HTTP client
    let client = Client::new();

    let response = client
        .post("http://localhost:8080/customers") // API endpoint
        .json(&credentials) // Send credentials as JSON in the body
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