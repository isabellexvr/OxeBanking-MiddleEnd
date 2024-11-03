use actix_web::{get, post, web, HttpResponse, Responder, Error};
use log::error; // Importa função de log de erro
use crate::dto::new_user_dto::UserDTO;
use crate::microservices::admin::create_a_new_mocked_user;
use bcrypt::{hash, DEFAULT_COST};

#[post("/sign-up")]
async fn sign_up(credentials: web::Json<UserDTO>) -> impl Responder {

    let hashed_password = hash(&credentials.user_password, DEFAULT_COST).expect("Failed to hash password");

    println!("Hashed password: {}", hashed_password);

    let user_info = UserDTO {
        user_password: hashed_password, // Use the hashed password
        ..credentials.into_inner() // Copy other fields from credentials
    };

    match create_a_new_mocked_user(web::Json(user_info)).await {
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

