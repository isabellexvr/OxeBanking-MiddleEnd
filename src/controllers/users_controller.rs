use actix_web::{get, post, web, HttpResponse, Responder, Error};
use log::error; // Importa função de log de erro
use crate::dto::new_user_dto::UserDTO;

#[post("/sign-up")]
async fn sign_up(credentials: web::Json<UserDTO>) -> impl Responder {
    let new_user_info = credentials.into_inner();

    // Example: You can add some logic to save `new_user_info` to the database

    HttpResponse::Ok().json(new_user_info) // Return a JSON response
}
