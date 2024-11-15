use actix_web::{get, post, web, HttpResponse, Responder, Error, HttpMessage, HttpRequest};
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
