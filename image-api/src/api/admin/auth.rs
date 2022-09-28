use actix_web::{ post, web::{ Json } };
use serde::{ Serialize, Deserialize };
use crate::authtools;

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
pub async fn auth(req_obj: Json<AuthRequest>) -> Json<AuthMessage> {
    if req_obj.username.eq("abhii") {
        return Json(AuthMessage {
            success: true,
            message: format!(
                "Password Hash is: {}",
                authtools::generate_password_hash(req_obj.password.clone())
            )
        })
    }

    Json(AuthMessage {
        success: false,
        message: String::from(
            "The username and password combination is not valid!"
        )
    })
}
