use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest};
use log::error; // Importa função de log de erro
use crate::dto::new_user_dto::UserDTO;
use crate::microservices::admin::{insert_user, get_authenticated_user};
use bcrypt::{hash, DEFAULT_COST};
use crate::services::auth_service::create_jwt_token;
use crate::services::users_services;
use crate::models::auth::AuthResponse;

#[post("/sign-up")]
async fn sign_up(credentials: web::Json<UserDTO>) -> impl Responder {

    users_services::sign_up(credentials).await

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
#[get("/account-info/{user_id}")]
async fn get_account_info(user_id: web::Path<i32>) -> impl Responder {
    let account = users_services::get_bank_account_info(user_id.into_inner()).await;

    match account {
        Ok(account_info) => HttpResponse::Ok().json(account_info),
        Err(err) => {
            let error_message = format!("Erro ao obter informações da conta: {}", err);
            error!("{}", error_message);
            HttpResponse::InternalServerError().json(error_message)
        }
    }
}
