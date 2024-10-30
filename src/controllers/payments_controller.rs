use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use crate::dto::payment_dto::PaymentDTO;
use crate::dto::jwt::Claims;
use crate::microservices::payments::{post_new_payment};
use actix_web::HttpMessage;

#[post("/new")]
async fn create_payment(payment: web::Json<PaymentDTO>, req: HttpRequest) -> impl Responder {
    // Verifica se as informações do token (Claims) foram armazenadas no request pelo middleware
    if let Some(claims) = req.extensions().get::<Claims>() {
        println!("User ID from token: {}", claims.user_id);
        println!("User name from token: {}", claims.sub);

    }

    // Chama o microserviço de pagamentos para criar um novo pagamento
    let response = post_new_payment(payment).await.unwrap();
    println!("Response from microservice: {}", response);

    HttpResponse::Ok().json("Payment created!")
}
