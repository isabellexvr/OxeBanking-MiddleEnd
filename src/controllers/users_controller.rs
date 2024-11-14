use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest};
use log::error; // Importa função de log de erro
use crate::dto::new_user_dto::UserDTO;
use crate::microservices::admin::{insert_user, get_authenticated_user};
use bcrypt::{hash, DEFAULT_COST};
use crate::services::auth_service::create_jwt_token;
use crate::models::auth::AuthResponse;

#[post("/sign-up")]
async fn sign_up(credentials: web::Json<UserDTO>) -> impl Responder {

    let hashed_password = hash(&credentials.user_password, DEFAULT_COST).expect("Failed to hash password");

    //println!("Hashed password: {}", hashed_password);

    let user_info = UserDTO {
        user_password: hashed_password, // Use the hashed password
        ..credentials.into_inner() // Copy other fields from credentials
    };

    match insert_user(user_info).await {
        Ok(response) => {
            // Microservice call was successful, return the response
            let user_id = response.id.expect("User ID is missing");
            let token = create_jwt_token(&response.full_name, user_id, &response.profile_pic).expect("Failed to create JWT token");
            let res = AuthResponse {
                auth_token: token,
            };
            HttpResponse::Ok().json(res)
        },
        Err(err) => {
            // Microservice call failed, log the error and return an internal server error
            let error_message = format!("Erro do Microsserviço de Admin: {}", err);
            error!("{}", error_message);
            HttpResponse::InternalServerError().json(error_message)
        }
    }


    // Example: You can add some logic to save `new_user_info` to the database

    //HttpResponse::Ok() // Return a JSON response
}

#[get("/user-info")]
async fn get_user_info(req: HttpRequest) -> impl Responder {
    let user = get_authenticated_user(req).await;
    match user {
        Ok(user_info) => HttpResponse::Ok().json(user_info),
        Err(err) => {
            let error_message = format!("Erro ao obter informações do usuário: {}", err);
            error!("{}", error_message);
            HttpResponse::InternalServerError().json(error_message)
        }
    }
}