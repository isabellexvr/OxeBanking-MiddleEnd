use actix_web::{get, post, web, HttpResponse, Responder, Error};
use crate::microservices::insurances::{get_insurance_details, get_all_insurances};

#[get("/all")]
async fn get_all_insurances_controller() -> impl Responder {
    match get_all_insurances() {
        Ok(insurances) => HttpResponse::Ok().json(insurances),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}