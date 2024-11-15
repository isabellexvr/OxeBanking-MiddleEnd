use actix_web::{get, post, put, web, HttpResponse, Responder, Error, HttpMessage, HttpRequest};
use crate::dto::credit_card_dto::{CreditCardDTO, CreditCardNoUserId};
use crate::dto::jwt::Claims;
use crate::microservices::loans::{request_loan, get_loan_request_by_user_id, get_loan_request_info_by_id, delete_loan_request_by_id, get_all_user_loans, get_loan_info_by_id, get_loan_payments, pay_loan};
use crate::dto::loans_dto::{LoansRequestDTO, LoansRequestResponseDTO};

#[post("/new-request")]
async fn request_new_loan(req:HttpRequest, loan_info: web::Json<LoansRequestDTO>) -> impl Responder {

    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let loan = LoansRequestDTO {
            customerId: loan_info.customerId,
            requestedValue: loan_info.requestedValue,
            termInMonths: loan_info.termInMonths,
        };

        let microserviceRes = request_loan(web::Json(loan)).await;

            let response = match microserviceRes {
                Ok(response) => HttpResponse::Ok().body(response),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            };
            return Ok(response);
        } else {
            return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
        }
}


#[get("/request/info")]
async fn get_loan_request_info(req:HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let microserviceRes = get_loan_request_by_user_id(user_id).await;
        let response = match microserviceRes {
            Ok(response) => HttpResponse::Ok().body(serde_json::to_string(&response).unwrap()),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        };
        return response;
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
}

#[get("/request/info/{id}")]
async fn get_loan_request_info_by_id_route(req:HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
        if user_id == id {
            let microserviceRes = get_loan_request_info_by_id(id).await;
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

#[get("/request/delete/{id}")]
async fn delete_loan_request_by_id_route(req:HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
        if user_id == id {
            let microserviceRes = delete_loan_request_by_id(id).await;
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

#[get("/request/all")]
async fn get_all_user_loans_route(req:HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let microserviceRes = get_all_user_loans(user_id).await;
        let response = match microserviceRes {
            Ok(response) => HttpResponse::Ok().body(serde_json::to_string(&response).unwrap()),
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        };
        return response;
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
}

#[get("/info/{id}")]
async fn get_loan_info_by_id_route(req:HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
        if user_id == id {
            let microserviceRes = get_loan_info_by_id(id).await;
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

#[get("/payments/{id}")]
async fn get_loan_payments_route(req:HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
        if user_id == id {
            let microserviceRes = get_loan_payments(id).await;
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

#[post("/payments/{id}")]
async fn pay_loan_route(req:HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<Claims>() {
        let user_id = claims.user_id;
        let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
        if user_id == id {
            let microserviceRes = pay_loan(id).await;
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