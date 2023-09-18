use std::future::{ ready, Ready };

use actix_web::{
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

impl FromRequest for AuthMiddleware {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get(header::AUTHORIZATION) {
            Some(auth_header) => {
                match auth_header.to_str() {
                    Ok(auth_header) => {
                        // The value of authorization header is in the format:
                        // `Bearer <jwt-token>`
                        // So, get the token after 7th pos (i.e. after space)).
                        let token = auth_header.split_at(7).1.to_string();

                        match decode_jwt(&token) {
                            Ok(token_data) => {
                                return ready(Ok(Self {
                                    authorized: true,
                                    login_id: token_data.sub
                                }));
                            }

                            Err(_) => ready(Ok(Self::default())),
                        }
                    }

                    Err(_) => ready(Ok(Self::default())),
                }
            }

            None => ready(Ok(Self::default())),
        }
    }
}

