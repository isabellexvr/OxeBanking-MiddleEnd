use reqwest::Client;
use crate::{dto::new_user_dto::UserDTO, errors::microservices_errors::ParseError};
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use serde_json::from_str;
use crate::dto::payment_dto::PaymentDTO;

pub async fn post_new_payment(payment: web::Json<PaymentDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let response = client.post("http://localhost:4000/api/pagamentos")
        .json(&payment)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

pub async fn get_all_payments() -> Result<Vec<PaymentDTO>, ParseError> {
    let client = Client::new();
    let response = client.get("http://localhost:4000/api/pagamentos")
        .send()
        .await?
        .text()
        .await?;
    let payments: Vec<PaymentDTO> = from_str(&response).map_err(ParseError::Serde)?;
    Ok(payments)
}

pub async fn get_payment_by_id(id: i32) -> Result<Option<PaymentDTO>, ParseError> {
    let client = Client::new();
    let url = format!("http://localhost:4000/api/pagamentos/{}", id);
    let response = client.get(&url).send().await?;
    if response.status().is_success() {
        let payment: PaymentDTO = response.json().await?;
        Ok(Some(payment))
    } else {
        Ok(None)
    }
}