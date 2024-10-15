use actix_web::{get, HttpResponse, Error};
use crate::services::external_api_service::call_external_api;
use log::error; // Importa função de log de erro

#[get("/external")]
async fn call_external() -> Result<HttpResponse, Error> {
    match call_external_api().await {
        Ok(response) => Ok(HttpResponse::Ok().body(response)),
        Err(e) => {
            error!("Error calling external API: {}", e); // Imprime o erro no console
            Ok(HttpResponse::InternalServerError().body("Failed to call external API"))
        }
    }
}

