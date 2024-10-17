use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::rc::Rc;

use crate::dto::user::Claims; // Import your Claims struct

// Middleware struct
pub struct JwtMiddleware {
    secret: Rc<String>, // Shared secret for JWT verification
}

impl JwtMiddleware {
    pub fn new(secret: String) -> Self {
        JwtMiddleware {
            secret: Rc::new(secret),
        }
    }
}

// Middleware Service implementation
impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService {
            service,
            secret: Rc::clone(&self.secret),
        })
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
    secret: Rc<String>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Poll readiness of the underlying service
    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    // Call the service
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = Rc::clone(&self.secret);

        Box::pin(async move {
            // Extract the Authorization header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    // Strip the "Bearer " part from the header
                    let token = auth_str.trim_start_matches("Bearer ").to_string(); // Create owned String

                    // Decode the JWT token
                    let validation = Validation::new(Algorithm::HS256);
                    match decode::<Claims>(
                        &token,
                        &DecodingKey::from_secret(secret.as_bytes()),
                        &validation,
                    ) {
                        Ok(_token_data) => {
                            // If the token is valid, proceed with the request
                            let res = self.service.call(req).await?;
                            Ok(res)
                        }
                        Err(_) => {
                            // If token is invalid, return Unauthorized response
                            Err(actix_web::error::ErrorUnauthorized("Invalid Token"))
                        }
                    }
                } else {
                    Err(actix_web::error::ErrorUnauthorized("Invalid Authorization Header"))
                }
            } else {
                // No Authorization header found
                Err(actix_web::error::ErrorUnauthorized("Missing Authorization Header"))
            }
        })
    }
}
