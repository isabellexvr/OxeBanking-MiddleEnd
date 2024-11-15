use reqwest::Client;
use crate::dto::credit_card_dto::CreditCardDTO;
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::{dto::new_user_dto::UserDTO, errors::microservices_errors::ParseError};
use serde_json::from_str;
use crate::helpers::load_env;

pub async fn create_new_credit_card(credit_card_info: web::Json<CreditCardDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_CREDIT_CARD_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_CREDIT_CARD_URL");

    let url = format!("{}/credit-card", url);
    let response = client.post(&url)
        .json(&credit_card_info)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_credit_card_by_id(id: i32) -> Result<CreditCardDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_CREDIT_CARD_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_CREDIT_CARD_URL");

    let url = format!("{}/credit-card/{}", url, id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
        let credit_card: CreditCardDTO = from_str(&response)?;
        
    Ok(credit_card)
}

pub async fn delete_credit_card_by_id(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_CREDIT_CARD_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_CREDIT_CARD_URL");

    let url = format!("{}/credit-card/{}", url, id);
    let response = client.delete(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn update_credit_card_limit(id: i32, new_limit: f64) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_CREDIT_CARD_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_CREDIT_CARD_URL");

    let url = format!("{}/credit-card/{}/limit?limiteTotal={}", url, id, new_limit);
    let response = client.patch(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}