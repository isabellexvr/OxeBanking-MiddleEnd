use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::repositories::insurances::get_all_insurances_repository;
use crate::microservices::insurances::{get_all_insurances, post_new_insurance};
use crate::dto::insurance_dto::{
    InsuranceGetDTO, 
    InsuranceReceiveDTO, 
    ClaimGetDTO, 
    ClaimReceiveDTO, 
    LogGetDTO,
    LogReceiveDTO
};

#[get("/mocked/all")]
async fn get_all_mocked_insurances_controller() -> impl Responder {
    match get_all_insurances_repository() {
        Ok(insurances) => HttpResponse::Ok().json(insurances),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

#[get("/all")]
async fn get_all_insurances_controller() -> impl Responder {
    let microserviceRes = get_all_insurances().await;

    match microserviceRes {
        Ok(insurances) => HttpResponse::Ok().json(insurances),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

#[post("/new/insurance")]
async fn post_new_insurance_controller(insurance: web::Json<InsuranceReceiveDTO>) -> impl Responder {
    let microserviceRes = post_new_insurance(insurance).await;

    match microserviceRes {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}