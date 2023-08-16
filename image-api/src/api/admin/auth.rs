use actix_web::{
    web::Json, HttpRequest, HttpResponse, cookie::Cookie, get, post
};
use serde::{ Serialize, Deserialize };
use crate::{
    repository::user::{ get_user_repository, UserRepository },
    auth::{
        pwd_hash::verify_password,
        token::{ create_session_token, create_refresh_token },
        utils::validate_session_token,
    },
};

#[derive(Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct AuthMessage {
    success: bool,
    // Session token (JWT)
    s: String,
    r: String,
    message: String
}

#[post("/api/admin/auth")]
pub async fn auth(req_obj: Json<AuthRequest>) -> HttpResponse {
    let repo = get_user_repository();

    let pw = repo.get_password_for_login_id(req_obj.username.clone());

    match pw {
        Ok (password_hash) => {
            let valid = verify_password(
                req_obj.password.clone(),
                password_hash
            );

            if valid {
                //let refresh_token: RefreshToken = create_refresh_token(req_obj.username.clone());
                let refresh_token: String = create_refresh_token(req_obj.username.clone());

                let ref_token_cookie: Cookie = Cookie::build(
                        //"r", refresh_token.refresh_token
                        "r", refresh_token
                    ).path("/")
                    .domain("localhost") // TODO: make this configurable
                    // .secure(true) // TODO: uncomment this for secure cookie!
                    .http_only(true)
                    .finish();

                return HttpResponse::Ok()
                    .cookie(ref_token_cookie)
                    .json(AuthMessage {
                        success: true,
                        s: create_session_token(req_obj.username.clone()),
                        r: create_refresh_token(req_obj.username.clone()),
                        message: String::from("Login Successful!"),
                });
            }

            HttpResponse::NotFound().json(AuthMessage {
                success: false,
                s: String::from(""),
                r: String::from(""),
                message: String::from("Username/Password combination is invalid")
            })
        }

        Err(_e) => {
            HttpResponse::NotFound().json(AuthMessage {
                success: false,
                s: String::from(""),
                r: String::from(""),
                message: String::from("Username/Password combination is invalid")
            })
        }
    }
}

/**
 * Gets permissions for the logged in user.
 */
#[get("/api/admin/auth/permissions")]
pub async fn get_user_permissions(req_obj: HttpRequest) -> HttpResponse {
    match validate_session_token(req_obj) {
        Ok (login_id) => {
            let repo = get_user_repository();

            match repo.get_permissions(login_id) {
                Ok (perms) => {
                    HttpResponse::Ok().json(perms)
                }

                Err (e) => {
                    HttpResponse::Forbidden().body(e)
                }
            }
        }

        Err (e) => {
            HttpResponse::Forbidden().body(e)
        }
    }
}

