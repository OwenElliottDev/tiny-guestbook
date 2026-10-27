use crate::models::ApiKey;
use actix_web::{FromRequest, HttpRequest, dev::Payload, http::header, web};
use subtle::ConstantTimeEq;

pub struct AdminAuth;

impl FromRequest for AdminAuth {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let expected = req
            .app_data::<web::Data<ApiKey>>()
            .and_then(|k| k.0.as_deref());

        let Some(expected) = expected else {
            return std::future::ready(Ok(AdminAuth));
        };

        let provided = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        let ok = provided.is_some_and(|got| expected.as_bytes().ct_eq(got.as_bytes()).into());

        std::future::ready(if ok {
            Ok(AdminAuth)
        } else {
            Err(actix_web::error::ErrorUnauthorized("unauthorized"))
        })
    }
}
