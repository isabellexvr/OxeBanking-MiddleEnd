use actix_web::{post, get, web, HttpRequest, HttpResponse, Responder};
use crate::dto::payment_dto::PaymentDTO;
use crate::dto::jwt::Claims;
use crate::models::payment::Payment;
use crate::microservices::payments::{post_new_payment, get_payment_by_id};
use actix_web::HttpMessage;

#[post("/new")]
async fn create_payment(payment: web::Json<PaymentDTO>, req: HttpRequest) -> impl Responder {
    // Verifica se as informações do token (Claims) foram armazenadas no request pelo middleware
/*     if let Some(claims) = req.extensions().get::<Claims>() {
        println!("User ID from token: {}", claims.user_id);
        println!("User name from token: {}", claims.sub);

    } */

    // Chama o microserviço de pagamentos para criar um novo pagamento
    let response = post_new_payment(payment).await.unwrap();
    println!("Response from microservice: {}", response);

    HttpResponse::Ok().json("Payment created!")
}

#[get("/all/{user_id}")]
async fn get_payments_by_user_id(user_id: web::Path<i32>) -> impl Responder {
    let mocked_payments = vec![
        Payment {
            id: 1,
            descricao: "Pagamento de aluguel".to_string(),
            id_usuario: *user_id,
            metodo_pagamento: "credito".to_string(),
            valor: 100000,
            saldo: 500000,
            fatura_pendente: None,
            destinatario: 2,
            inserted_at: "2021-08-01T00:00:00Z".to_string(),
            updated_at: "2021-08-01T00:00:00Z".to_string(),
        },
        Payment {
            id: 2,
            descricao: "Pagamento de condomínio".to_string(),
            id_usuario: *user_id,
            metodo_pagamento: "credito".to_string(),
            valor: 50000,
            saldo: 450000,
            fatura_pendente: None,
            destinatario: 3,
            inserted_at: "2021-08-01T00:00:00Z".to_string(),
            updated_at: "2021-08-01T00:00:00Z".to_string(),
        },
        Payment {
            id: 3,
            descricao: "Pagamento de luz".to_string(),
            id_usuario: *user_id,
            metodo_pagamento: "pix".to_string(),
            valor: 20000,
            saldo: 430000,
            fatura_pendente: None,
            destinatario: 4,
            inserted_at: "2021-08-01T00:00:00Z".to_string(),
            updated_at: "2021-08-01T00:00:00Z".to_string(),
        },
        Payment {
            id: 4,
            descricao: "Pagamento de internet".to_string(),
            id_usuario: *user_id,
            metodo_pagamento: "credito".to_string(),
            valor: 15000,
            saldo: 415000,
            fatura_pendente: None,
            destinatario: 5,
            inserted_at: "2021-08-01T00:00:00Z".to_string(),
            updated_at: "2021-08-01T00:00:00Z".to_string(),
        },
        Payment {
            id: 5,
            descricao: "Pagamento de academia".to_string(),
            id_usuario: *user_id,
            metodo_pagamento: "credito".to_string(),
            valor: 30000,
            saldo: 385000,
            fatura_pendente: Some(2500),
            destinatario: 6,
            inserted_at: "2021-08-01T00:00:00Z".to_string(),
            updated_at: "2021-08-01T00:00:00Z".to_string(),
        }
    ];

    HttpResponse::Ok().json(mocked_payments)
}

#[get("/latest/{user_id}")]	
async fn get_latest_payments_by_user_id(user_id: web::Path<i32>) -> impl Responder {
    let mocked_payments = vec![
        Payment {
            id: 1,
            descricao: "Pagamento de aluguel".to_string(),
            id_usuario: *user_id,
            metodo_pagamento: "credito".to_string(),
            valor: 100000,
            saldo: 500000,
            fatura_pendente: None,
            destinatario: 2,
            inserted_at: "2021-08-01T00:00:00Z".to_string(),
            updated_at: "2021-08-01T00:00:00Z".to_string(),
        },
        Payment {
            id: 2,
            descricao: "Pagamento de condomínio".to_string(),
            id_usuario: *user_id,
            metodo_pagamento: "credito".to_string(),
            valor: 50000,
            saldo: 450000,
            fatura_pendente: None,
            destinatario: 3,
            inserted_at: "2021-08-01T00:00:00Z".to_string(),
            updated_at: "2021-08-01T00:00:00Z".to_string(),
        },
        Payment {
            id: 3,
            descricao: "Pagamento de luz".to_string(),
            id_usuario: *user_id,
            metodo_pagamento: "pix".to_string(),
            valor: 20000,
            saldo: 430000,
            fatura_pendente: None,
            destinatario: 4,
            inserted_at: "2021-08-01T00:00:00Z".to_string(),
            updated_at: "2021-08-01T00:00:00Z".to_string(),
        },
    ];

    HttpResponse::Ok().json(mocked_payments)
}

#[get("/details/{payment_id}")]
async fn get_payment_info(payment_id: web::Path<i32>) -> impl Responder {
    let response = get_payment_by_id(*payment_id).await.unwrap();

    HttpResponse::Ok().json(response)
}