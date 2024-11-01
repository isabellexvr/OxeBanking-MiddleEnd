use reqwest::Client;
use crate::{dto::new_user_dto::UserDTO, errors::microservices_errors::ParseError};
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use serde_json::from_str;
use crate::dto::payment_dto::PaymentDTO;

