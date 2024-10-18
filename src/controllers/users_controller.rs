use actix_web::{get, post, web, HttpResponse, Responder, Error};
use log::error; // Importa função de log de erro
use crate::dto::new_user_dto::UserDTO;
use crate::microservices::admin::call_admin_microservice;

#[post("/sign-up")]
async fn sign_up(credentials: web::Json<UserDTO>) -> impl Responder {


    match call_admin_microservice(credentials).await {
        Ok(response) => {
            // Microservice call was successful, return the response
            HttpResponse::Ok().json(response)
        },
        Err(err) => {
            // Microservice call failed, log the error and return an internal server error
            error!("Error calling microservice: {:?}", err);
            HttpResponse::InternalServerError().json("Error communicating with admin microservice")
        }
    }


    // Example: You can add some logic to save `new_user_info` to the database

    //HttpResponse::Ok() // Return a JSON response
}
