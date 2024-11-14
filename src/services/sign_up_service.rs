use crate::repositories::bank_accounts::create_new_account;
use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest};
use log::error; // Importa função de log de erro
use crate::dto::new_user_dto::UserDTO;
use crate::microservices::admin::{insert_user, get_authenticated_user};
use bcrypt::{hash, DEFAULT_COST};
use crate::services::auth_service::create_jwt_token;
use crate::models::auth::AuthResponse;

async fn sign_up(credentials: web::Json<UserDTO>) -> impl Responder {
    let hashed_password = hash(&credentials.user_password, DEFAULT_COST).expect("Failed to hash password");

    let user_info = UserDTO {
        user_password: hashed_password, 
        ..credentials.into_inner() 
    };

    match insert_user(user_info).await {
        Ok(response) => {
            // Microservice call was successful, return the response
            let user_id = response.id.expect("User ID is missing");

            let account_creation_result = create_new_account(user_id).await;

            match account_creation_result {
                Ok(_) => {
                    let token = create_jwt_token(&response.full_name, user_id, &response.profile_pic).expect("Failed to create JWT token");
                    let res = AuthResponse {
                        auth_token: token,
                    };
                    HttpResponse::Ok().json(res)
                },
                Err(err) => {
                    let error_message = format!("Erro ao criar conta: {}", err);
                    error!("{}", error_message);
                    HttpResponse::InternalServerError().json(error_message)
                }
            }

        },
        Err(err) => {
            let error_message = format!("Erro do Microsserviço de Admin: {}", err);
            error!("{}", error_message);
            HttpResponse::InternalServerError().json(error_message)
        }
    }
}