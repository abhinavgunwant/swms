use actix_web::{ post, web::{ Json }, HttpResponse };
use serde::{ Serialize, Deserialize };
use crate::repository::user::{ get_user_repository, User, UserRepository };
use crate::auth::pwd_hash::{ verify_password };

#[derive(Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct AuthMessage {
    success: bool,
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
                return HttpResponse::Ok().json(AuthMessage {
                    success: true, message: String::from("Login Successful!")
                });
            }

            HttpResponse::NotFound().json(AuthMessage {
                success: false,
                message: String::from("Username/Password combination is invalid")
            })
        }

        Err(_e) => {
            HttpResponse::NotFound().json(AuthMessage {
                success: false,
                message: String::from("Username/Password combination is invalid")
            })
        }
    }
}
