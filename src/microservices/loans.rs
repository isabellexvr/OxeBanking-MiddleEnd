use std::fmt::Result;

use reqwest::Client;
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::{dto::new_user_dto::UserDTO, errors::microservices_errors::ParseError};
use serde_json::from_str;
use crate::helpers::load_env;
use crate::dto::loans_dto::{LoanRequestDTO, LoanResponseDTO};

pub async fn request_loan(loan_info: web::Json<LoanRequestDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_LOANS_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_LOANS_URL");

    let url = format!("{}/loans/request", url);
    let response = client.post(&url)
        .json(&loan_info)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)

}