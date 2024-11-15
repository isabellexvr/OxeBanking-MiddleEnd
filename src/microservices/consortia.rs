use reqwest::Client;
use crate::{errors::microservices_errors::ParseError};
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use serde_json::from_str;
use crate::helpers::load_env;

use crate::dto::consortia_dto::ConsortiumDTO;

pub async fn post_new_consortia(payment: web::Json<ConsortiumDTO>) -> Result<String, ParseError> {
    let client = Client::new();
    let url = "http://localhost/consorcios".to_string();

    let response = client.post(&url)
        .json(&payment)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_all_consortia() -> Result<ConsortiumDTO, ParseError> {
    let client = Client::new();
    let url = "http://localhost/consorcios".to_string();

    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
    let consortia: ConsortiumDTO = from_str(&response)?;
        
    Ok(consortia)
}

pub async fn add_participant_to_consortium(id: i32, participant_id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = format!("http://localhost/consorcios/{}/participantes/{}", id, participant_id);

    let response = client.post(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_consortium_participants(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = format!("http://localhost/consorcios/{}/participantes", id);

    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_consortium_by_user_id(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = format!("http://localhost/consorcios/usuario/{}", id);

    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}


pub async fn contemplate_consortium(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = format!("http://localhost/consorcios/{}/contemplar", id);

    let response = client.post(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn delete_consortium(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = format!("http://localhost/consorcios/{}", id);

    let response = client.delete(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

pub async fn get_consortium_by_id(id: i32) -> Result<String, ParseError> {
    let client = Client::new();
    let url = format!("http://localhost/consorcios/{}", id);

    let response = client.get(&url)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}


