use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::future::{ready, Ready};
use crate::dto::user::Claims;
use crate::helpers::load_env;

pub struct Auth;

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

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let secret = load_env("SECRET".to_string());
        let auth_header = req.headers().get("Authorization");

        if let Some(header_value) = auth_header {
            if let Ok(auth_str) = header_value.to_str() {
                let token = auth_str.replace("Bearer ", "");

                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(token_data) => {
                        // Armazenar informações do token no request para acesso posterior
                        req.extensions_mut().insert(token_data.claims);
                    }
                    Err(_) => {
                        return Box::pin(async {
                            Err(actix_web::error::ErrorUnauthorized("Invalid Token"))
                        });
                    }
                }
            } else {
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid Authorization Header"))
                });
            }
        } else {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Missing Authorization Header"))
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
