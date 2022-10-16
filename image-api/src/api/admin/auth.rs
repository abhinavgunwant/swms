use actix_web::{ web::{ Json }, HttpResponse, cookie::Cookie, post };
use serde::{ Serialize, Deserialize };
use crate::repository::user::{ get_user_repository, User, UserRepository };
use crate::auth::{
    pwd_hash::{ verify_password },
    token::{
        RefreshToken, create_session_token, create_refresh_token
    }
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
