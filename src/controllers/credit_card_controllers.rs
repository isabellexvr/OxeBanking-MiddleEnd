use actix_web::{get, post, put, web, HttpResponse, Responder, Error, HttpMessage, HttpRequest};
use crate::dto::credit_card_dto::{CreditCardDTO, CreditCardNoUserId};
use crate::dto::jwt::Claims;
use crate::microservices::credit_card::{self, create_new_credit_card, delete_credit_card_by_id, get_credit_card_by_id, update_credit_card_limit};

#[post("/new")]
async fn request_new_credit_card(req:HttpRequest, credit_card_info: web::Json<CreditCardNoUserId>) -> impl Responder {

    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let credit_card = CreditCardDTO {
            id: None,
            numeroCartao: credit_card_info.numeroCartao.clone(),
            cvv: credit_card_info.cvv.clone(),
            dataValidade: credit_card_info.dataValidade.clone(),
            limiteTotal: credit_card_info.limiteTotal,
            status: credit_card_info.status.clone(),
            limiteDisponivel: credit_card_info.limiteDisponivel,
            idUsuario: user_id,
            closingDay: credit_card_info.closingDay,
        };

        let microserviceRes = create_new_credit_card(web::Json(credit_card)).await;

            let response = match microserviceRes {
                Ok(response) => HttpResponse::Ok().body(response),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            };
            return response;
        } else {
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
    }

#[get("/info/{id}")]
async fn get_credit_card_info(req:HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
        if user_id == id {
            let microserviceRes = get_credit_card_by_id(id).await;
            let response = match microserviceRes {
                Ok(response) => HttpResponse::Ok().body(serde_json::to_string(&response).unwrap()),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            };
            return response;
        } else {
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
}

#[put("/update/{id}")]
async fn update_credit_card_limit_route(req:HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
        if user_id == id {
            let new_limit = req.query_string().parse::<f64>().unwrap();
            let microserviceRes = update_credit_card_limit(id, new_limit).await;
            let response = match microserviceRes {
                Ok(response) => HttpResponse::Ok().body(response),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            };
            return response;
        } else {
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
}