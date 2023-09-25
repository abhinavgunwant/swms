//! auth
//!
//! Contains Authentication and Authorization related code.

use actix_web::{
    web::{ Json, Data }, HttpRequest, HttpResponse, get, post,
    cookie::{ time::Duration as ActixWebDuration, Cookie },
};
use serde::{ Serialize, Deserialize };

use crate::{
    db::DBError, model::role::Role, server_state::ServerState,
    repository::{
        user::{ get_user_repository, UserRepository },
        role::{ get_role_repository, RoleRepository },
    },
    auth::{
        AuthMiddleware, pwd_hash::verify_password,
        token::{
            create_session_token, create_refresh_token,
            create_session_token_from_refresh_token, TokenError,
            create_refresh_token_cookie, RefreshTokenData, get_expiry_from_now,
        },
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
pub async fn auth(
    req_obj: Json<AuthRequest>,
    srv_state: Data<ServerState>,
) -> HttpResponse {
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
        let ref_token_data = RefreshTokenData {
            user_id,
            role_id: user_role.id,
            username: req_obj.username.clone(),
            name: name.clone(),
            expiry: get_expiry_from_now()
        };

        let mut loop_counter: u8 = 0;

        loop {
            let rt = create_refresh_token();

            if !srv_state.refresh_token_exists(&rt) {
                srv_state.insert_refresh_token(rt.clone(), ref_token_data);

                return HttpResponse::Ok().cookie(
                    create_refresh_token_cookie(rt)
                ).json(AuthMessage {
                    success: true,
                    s: create_session_token(
                        req_obj.username.clone(), name, user_id, user_role
                    ),
                    message: String::from("Login Successful!"),
                });
            }

            // after looping 100 times, just give up!
            if loop_counter > 98 {
                break;
            }

            loop_counter += 1;
        }

        return HttpResponse::InternalServerError().json(AuthMessage {
            success: false,
            s: String::from(""),
            message: String::from("An internal error occured."),
        });
    }

    HttpResponse::NotFound().json(AuthMessage {
        success: false,
        s: String::from(""),
        message: String::from("Username/Password combination is invalid")
    })
}

#[get("/logout")]
pub async fn auth_logout(req: HttpRequest) -> HttpResponse {
    let ref_token_cookie_exp: Cookie = Cookie::build("r", "")
        .path("/")
        .domain("localhost") // TODO: make this configurable
        // .secure(true) // TODO: uncomment this for secure cookie!
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();
    
    let srv_state = req.app_data::<Data<ServerState>>().unwrap();

    // delete the refresh token from the hash map.
    match req.cookie("r") {
        Some(cookie) => {
            let val = String::from(cookie.value());
            println!("found refresh token in cookie: {}", val);

            srv_state.remove_refresh_token(val);
            HttpResponse::Ok().cookie(ref_token_cookie_exp).body("Logged out")
        }

        None => {
            println!("No refresh cookie found in the request!");
            HttpResponse::BadRequest().body("You're not signed in!")
        }
    }
}

#[get("/refresh")]
pub async fn auth_refresh(req: HttpRequest, _: AuthMiddleware) -> HttpResponse {
    if let Some(cookie) = req.cookie("r") {
        let val = String::from(cookie.value());

        if val.is_empty() {
            return HttpResponse::BadRequest().body("You're not signed in!");
        }

        let srv_state = req.app_data::<Data<ServerState>>().unwrap();

        if let Some(ref_token) = srv_state.get_refresh_token_data(val.clone()) {
            match create_session_token_from_refresh_token(ref_token) {
                Ok(token) => {
                    srv_state.reset_refresh_token_expiry(val);
                    return HttpResponse::Ok().body(token);
                }

                Err(e) => {
                    match e {
                        TokenError::InvalidToken => {
                            return HttpResponse::UnprocessableEntity()
                                .body("You session is either invalid or expired, please login again!");
                        }

                        TokenError::RoleNotFound => {
                            return HttpResponse::UnprocessableEntity()
                                .body("User role could not be found. Please contact your administrator!");
                        }

                        _ => {
                            return HttpResponse::InternalServerError()
                                .body("Error 500: Internal Server Error");
                        }
                    }
                }
            }
        }
    }

    println!("Refresh token cookie not set.");
    HttpResponse::BadRequest().body("You're not signed in!")
}

/**
 * Gets permissions for the logged in user.
 */
#[get("/api/admin/auth/permissions")]
pub async fn get_user_permissions(am: AuthMiddleware) -> HttpResponse {
    match get_user_repository().get_permissions(am.login_id) {
        Ok (perms) => HttpResponse::Ok().json(perms),
        Err (e) => HttpResponse::Forbidden().body(e),
    }
}

