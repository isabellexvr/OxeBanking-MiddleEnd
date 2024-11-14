use reqwest::Client;
use crate::{dto::new_user_dto::UserDTO, errors::microservices_errors::ParseError};
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use serde_json::from_str;
use crate::helpers::load_env;

use crate::dto::insurance_dto::{
    InsuranceGetDTO, 
    InsuranceReceiveDTO, 
    ClaimGetDTO, 
    ClaimReceiveDTO, 
    LogGetDTO,
    LogReceiveDTO
};

pub async fn post_new_insurance(payment: web::Json<InsuranceReceiveDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/insurance", url);
    let response = client.post(&url)
        .json(&payment)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_all_insurances() -> Result<InsuranceGetDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/insurance", url);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
        let insurance: InsuranceGetDTO = from_str(&response)?;
        
    Ok(insurance)
}

pub async fn get_insurances_by_id(id: i32) -> Result<InsuranceGetDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/insurance/{}", url, id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
        let insurance: InsuranceGetDTO = from_str(&response)?;
        
    Ok(insurance)
}

pub async fn get_insurances_by_user_id(id: i32) -> Result<InsuranceGetDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/insurance/user/{}", url, id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
        let insurance: InsuranceGetDTO = from_str(&response)?;
        
    Ok(insurance)
}

pub async fn update_insurance(id: i32, insurance: web::Json<InsuranceReceiveDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/insurance/{}", url, id);
    let response = client.put(&url)
        .json(&insurance)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn delete_insurance(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
    .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/insurance/{}", url, id);
    let response = client.delete(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn post_new_claim(claim: web::Json<ClaimReceiveDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/claim", url);
    let response = client.post(&url)
        .json(&claim)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_all_claims() -> Result<ClaimGetDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/claim", url);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
    let claims: ClaimGetDTO = from_str(&response)?;

    Ok(claims)
}

pub async fn get_claim_by_id(id: i32) -> Result<ClaimGetDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/claim/{}", url, id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
    let claim: ClaimGetDTO = from_str(&response)?;

    Ok(claim)
}

pub async fn update_claim(id: i32, claim: web::Json<ClaimReceiveDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/claim/{}", url, id);
    let response = client.put(&url)
        .json(&claim)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn delete_claim(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/claim/{}", url, id);
    let response = client.delete(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_claims_by_insurance_id(insurance_id: i32) -> Result<ClaimGetDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/claim/insurance/{}", url, insurance_id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
    let claims: ClaimGetDTO = from_str(&response)?;

    Ok(claims)
}

pub async fn post_new_log(log: web::Json<LogReceiveDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/log", url);
    let response = client.post(&url)
        .json(&log)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_all_logs() -> Result<LogGetDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/log", url);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
    let logs: LogGetDTO = from_str(&response)?;

    Ok(logs)
}

pub async fn get_log_by_id(id: i32) -> Result<LogGetDTO, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/log/{}", url, id);
    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
    let log: LogGetDTO = from_str(&response)?;

    Ok(log)
}

pub async fn update_log(id: i32, log: web::Json<LogReceiveDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/log/{}", url, id);
    let response = client.put(&url)
        .json(&log)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn delete_log(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = String::from_utf8(load_env("MICROSERVICE_INSURANCES_URL".to_string()))
        .expect("Invalid UTF-8 in MICROSERVICE_INSURANCES_URL");

    let url = format!("{}/log/{}", url, id);
    let response = client.delete(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}