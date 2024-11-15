use std::result::Result;
use reqwest::Client;
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::{dto::new_user_dto::UserDTO, errors::microservices_errors::ParseError};
use serde_json::from_str;
use crate::helpers::load_env;
use crate::dto::loans_dto::{LoansRequestDTO, LoansRequestResponseDTO};

pub async fn request_loan(loan_info: web::Json<LoansRequestDTO>) -> Result<String, ParseError> {
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


pub async fn get_loan_request_by_user_id(user_id: i32) -> Result<LoansRequestResponseDTO, ParseError> {

    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_LOANS_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_LOANS_URL");

    let url = format!("{}/loans/request?customerId={}", url, user_id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
        let loans: LoansRequestResponseDTO = from_str(&response)?;
        
    Ok(loans)
}

pub async fn get_loan_request_info_by_id(user_id: i32) -> Result<LoansRequestResponseDTO, ParseError> {

    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_LOANS_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_LOANS_URL");

    let url = format!("{}/loans/request/{}", url, user_id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
        let loans: LoansRequestResponseDTO = from_str(&response)?;
        
    Ok(loans)
}

pub async fn delete_loan_request_by_id(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_LOANS_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_LOANS_URL");

    let url = format!("{}/loans/request/{}", url, id);
    let response = client.delete(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_all_user_loans(user_id: i32) -> Result<Vec<LoansRequestResponseDTO>, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_LOANS_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_LOANS_URL");

    let url = format!("{}/loans/request?customerId={}", url, user_id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
        let loans: Vec<LoansRequestResponseDTO> = from_str(&response)?;
        
    Ok(loans)
}

pub async fn get_loan_info_by_id(loan_id: i32) -> Result<LoansRequestResponseDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_LOANS_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_LOANS_URL");

    let url = format!("{}/loans/request/{}", url, loan_id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
        let loans: LoansRequestResponseDTO = from_str(&response)?;
        
    Ok(loans)
}

pub async fn get_loan_payments(loan_id: i32) -> Result<Vec<LoansRequestResponseDTO>, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_LOANS_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_LOANS_URL");

    let url = format!("{}/loans/{}/payments", url, loan_id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
        let loans: Vec<LoansRequestResponseDTO> = from_str(&response)?;
        
    Ok(loans)
}

pub async fn pay_loan(loan_id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_LOANS_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_LOANS_URL");

    let url = format!("{}/loans/{}/payments", url, loan_id);
    let response = client.post(&url)
        .json(&loan_id)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}