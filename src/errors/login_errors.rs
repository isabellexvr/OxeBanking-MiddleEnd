use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("HTTP request failed")]
    Reqwest(#[from] ReqwestError),
    #[error("Failed to parse JSON")]
    Serde(#[from] SerdeError),
}
