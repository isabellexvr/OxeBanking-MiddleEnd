use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] ReqwestError), // Converts reqwest error

    #[error("Failed to parse JSON: {0}")]
    Serde(#[from] SerdeError), // Converts serde error
}
