use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use crate::dto::payment_dto::PaymentDTO;
use crate::dto::user::Claims;
use actix_web::HttpMessage;

#[post("/new")]
async fn create_payment(payment: web::Json<PaymentDTO>, req: HttpRequest) -> impl Responder {
    // Verifica se as informações do token (Claims) foram armazenadas no request pelo middleware
    if let Some(claims) = req.extensions().get::<Claims>() {
        println!("User ID from token: {}", claims.user_id);
        println!("User name from token: {}", claims.sub);

    }

    HttpResponse::Ok().json("Payment created!")
}
