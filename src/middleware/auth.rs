use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;

use crate::model::Claims; 

pub struct Auth;

impl Auth {
    pub fn new() -> Self {
        Auth
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
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
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip authentication for login route
        if req.path() == "/api/v1/auth/login" {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            });
        }

        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        if let Some(token) = token {
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
            let decoding_key = DecodingKey::from_secret(secret.as_ref());
            let validation = Validation::default();

            match decode::<Claims>(token, &decoding_key, &validation) {
                Ok(_claims) => {
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res.map_into_left_body())
                    })
                }
                Err(_) => {
                    let (http_request, _) = req.into_parts();
                    let response = HttpResponse::Unauthorized()
                        .json(json!({
                            "status": false,
                            "message": "Invalid token",
                            "data": null
                        }));
                    let service_response = ServiceResponse::new(
                        http_request,
                        response.map_into_right_body(),
                    );
                    Box::pin(async move { Ok(service_response) })
                }
            }
        } else {
            let (http_request, _) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(json!({
                    "status": false,
                    "message": "Token not provided",
                    "data": null
                }));
            let service_response = ServiceResponse::new(
                http_request,
                response.map_into_right_body(),
            );
            Box::pin(async move { Ok(service_response) })
        }
    }
} 