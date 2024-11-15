use actix_web::{get, post, delete, put, web, HttpResponse, HttpRequest, Responder, Error};
use crate::microservices::consortia::{get_all_consortia, post_new_consortia, add_participant_to_consortium, get_consortium_participants, get_consortium_by_user_id, contemplate_consortium, delete_consortium};
use crate::dto::consortia_dto::ConsortiumDTO;


#[get("/all")]
async fn get_all_consortia_controller() -> impl Responder {
    let microserviceRes = get_all_consortia().await;

    match microserviceRes {
        Ok(consortia) => HttpResponse::Ok().json(consortia),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

#[post("/new/consortium")]
async fn post_new_consortium_controller(consortium: web::Json<ConsortiumDTO>) -> impl Responder {
    let microserviceRes = post_new_consortia(consortium).await;

    match microserviceRes {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

#[get("/participants/{id}")]
async fn get_consortium_by_user_id_controller(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let microserviceRes = get_consortium_by_user_id(id).await;

    match microserviceRes {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

#[get("/consortium/{id}/participants")]
async fn get_consortium_participants_controller(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let microserviceRes = get_consortium_participants(id).await;

    match microserviceRes {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

#[delete("/consortium/{id}")]
async fn delete_consortium_controller(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let microserviceRes = delete_consortium(id).await;

    match microserviceRes {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

#[post("/consortium/{id}/participants/{participant_id}")]
async fn add_participant_to_consortium_controller(path: web::Path<(i32, i32)>) -> impl Responder {
    let (id, participant_id) = path.into_inner();
    let microserviceRes = add_participant_to_consortium(id, participant_id).await;

    match microserviceRes {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

#[post("/consortium/{id}/contemplate")]
async fn contemplate_consortium_controller(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let microserviceRes = contemplate_consortium(id).await;

    match microserviceRes {
        Ok(response) => HttpResponse::Ok().body(response),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}