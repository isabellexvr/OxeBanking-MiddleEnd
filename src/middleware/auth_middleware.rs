use std::future::{ready, Ready};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use crate::dto::user::Claims;


use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

const SECRET: &[u8] = b"my_secret_key"; // Carregar do .env mais tarde

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
       
        /* if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
            // Convert the header value to a string and print it
            if let Ok(auth_str) = auth_header.to_str() {
                println!("Authorization: {}", auth_str);
                let token = auth_str.replace("Bearer ", "");
                println!("Token: {}", token);
                let validation = Validation {
                    validate_exp: false,
                    algorithms: vec![Algorithm::HS256],
                    ..Validation::default()
                };
                let key = DecodingKey::from_secret(secret.as_bytes());
                let token_data = decode::<Claims>(&token, &key, &validation);
            }
        }else{
            println!("Error: Authorization header is not a valid string");
            Err(actix_web::error::ErrorUnauthorized("Invalid Authorization Header"));
        } */

        let auth_header = req.headers().get("Authorization");

        if let Some(header_value) = auth_header {
            if let Ok(auth_str) = header_value.to_str() {
                let token = auth_str.replace("Bearer ", "");

                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(SECRET),
                    &Validation::new(Algorithm::HS256),
                ){
                    Ok(token_data) => println!("Token data: {:?}", token_data.claims),
                    Err(_) => println!("Error: Invalid Token")
                }

            }else{
                println!("Error: Authorization header is not a valid string");
                return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Invalid Authorization Header")) });
            }
        }else{
            println!("Error: Missing Authorization Header");
            return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Missing Authorization Header")) });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi, this be mr. middleware talking here.");
            Ok(res)
        })
    }
}