use actix_web::{
    web::Json, HttpRequest, HttpResponse, get, post,
    cookie::{ time::Duration as ActixWebDuration, Cookie },
};
use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc, Duration };

use crate::{
    db::DBError,
    model::{ user::User, role::Role },
    repository::{
        user::{ get_user_repository, UserRepository },
        role::{ get_role_repository, RoleRepository },
    },
    auth::{
        pwd_hash::verify_password,
        token::{
            create_session_token, create_refresh_token, remove_refresh_token
        },
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
    //r: String,
    message: String
}

#[post("/login")]
pub async fn auth(req_obj: Json<AuthRequest>) -> HttpResponse {
    let user_repo = get_user_repository();
    let role_repo = get_role_repository();

    let user_valid: bool;
    let user_id: u32;
    let user_role: Role;
    let name: String;

    match user_repo.get_from_login_id(req_obj.username.clone()) {
        Ok (user) => {
            name = user.name;
            user_id = user.id;

            match user_repo.get_password_for_login_id(req_obj.username.clone()) {
                Ok (password_hash) => {
                    user_valid = verify_password(
                        req_obj.password.clone(),
                        password_hash
                    );

                    match role_repo.get(user.user_role) {
                        Ok (role) => { user_role = role },
                        Err (e) => {
                            eprintln!(
                                "Some error occured while getting role (user-id: {}): {}",
                                user.id,
                                e
                            );

                            user_role = Role::default()
                        },
                    }
                }

                Err (_) => {
                    return HttpResponse::InternalServerError()
                        .body("Error 500: Internal Server Error!");
                }
            }
        }

        Err (e) => {
            match e {
                DBError::NOT_FOUND => {
                    return HttpResponse::NotFound()
                        .body("Error 404: User not found!");
                }

                DBError::OtherError => {
                    return HttpResponse::InternalServerError()
                        .body("Error 500: Internal Server Error!");
                }
            }
        }
    }

    if user_valid {
        let refresh_token: String = create_refresh_token(
            user_id,
            req_obj.username.clone(),
            name.clone(),
            user_role.id
        );

        let ref_token_cookie: Cookie = Cookie::build("r", refresh_token)
            .path("/")
            .domain("localhost") // TODO: make this configurable
            // .secure(true) // TODO: uncomment this for secure cookie!
            .max_age(ActixWebDuration::new(1800, 0))
            .http_only(true)
            .finish();

        return HttpResponse::Ok().cookie(ref_token_cookie).json(AuthMessage {
                success: true,
                s: create_session_token(req_obj.username.clone(), name, user_role),
                message: String::from("Login Successful!"),
        });
    }

    HttpResponse::NotFound().json(AuthMessage {
        success: false,
        s: String::from(""),
        message: String::from("Username/Password combination is invalid")
    })
}

#[get("/logout")]
async fn auth_logout(req: HttpRequest) -> HttpResponse {
    let ref_token_cookie_exp: Cookie = Cookie::build("r", "")
        .path("/")
        .domain("localhost") // TODO: make this configurable
        // .secure(true) // TODO: uncomment this for secure cookie!
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    // delete the refresh token from the hash map.
    match req.cookie("r") {
        Some(cookie) => {
            let val = String::from(cookie.value());
            println!("found refresh token in cookie: {}", val);

            unsafe {
                remove_refresh_token(val);
            }
        }

        None => {
            println!("No refresh cookie found in the request!");
            return HttpResponse::BadRequest().body("You're not signed in!");
        }
    }

    HttpResponse::Ok().cookie(ref_token_cookie_exp).body("Logged out")
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

