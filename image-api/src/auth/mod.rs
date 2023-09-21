use std::future::{ ready, Ready };

use actix_web::{
    error::ErrorUnauthorized,
    FromRequest, Error as ActixError, HttpRequest, dev::Payload, http::header,
};

use crate::auth::token::decode_jwt;

pub mod pwd_hash;
pub mod token;
pub mod utils;

#[derive(Default)]
pub struct AuthMiddleware {
    pub authorized: bool,
    pub login_id: String, // a.k.a the "username"
}

/// Handles Request authorization
impl FromRequest for AuthMiddleware {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(token) = Self::get_session_token(req) {
            if let Ok(token_data) = decode_jwt(&token) {
                return ready(Ok(Self {
                    authorized: true,
                    login_id: token_data.sub
                }));
            }
        }

        if req.path() == "/api/admin/auth/refresh"
            || req.path() == "/api/admin/auth/login" {
            return ready(Ok(Self::default()));
        }

        ready(Err(ErrorUnauthorized("UNAUTHORIZED")))
    }
}

impl AuthMiddleware {
    /// Returns the session token if it exists on `Authorization` header.
    /// `None` otherwise.
    fn get_session_token(req: &HttpRequest) -> Option<String> {
        if let Some(auth_hdr_opt) = req.headers().get(header::AUTHORIZATION) {
            if let Ok(auth_header) = auth_hdr_opt.to_str() {
                if !auth_header.is_empty() && auth_header.len() > 7 {
                    return Some(auth_header.split_at(7).1.to_string());
                }
            }
        }

        None
    }
}

