use std::future::{ready, Ready};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use crate::dto::user::Claims;
use dotenv::dotenv;
use env_logger;
use std::env;

use crate::helpers::load_env;


use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;



//const SECRET: &[u8] = b"my_secret_key"; // Carregar do .env mais tarde

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
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
    
        // Load the secret key from .env
        let secret = load_env("SECRET".to_string());
    
        // Example usage with jwt encode/decode
        let my_secret_key = &secret; // Use it as &[u8]

        let auth_header = req.headers().get("Authorization");

        if let Some(header_value) = auth_header {
            if let Ok(auth_str) = header_value.to_str() {
                let token = auth_str.replace("Bearer ", "");

                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(my_secret_key),
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