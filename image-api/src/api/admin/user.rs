use actix_web::{ get, post, web::{ Json }, HttpRequest, HttpResponse };
use serde::{ Serialize, Deserialize };
use chrono::Utc;

use crate::{
    repository::user::{ get_user_repository, UserRepository },
    db::DBError,
    model::user::User
};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    login_id: String,
    password: String,
    name: String,
    email: String
}

#[derive(Serialize)]
pub struct Message {
    success: bool,
    message: String
}

#[post("/api/admin/user")]
pub async fn create_user(req_obj: Json<CreateUserRequest>) -> Json<Message> {
    let repo = get_user_repository();

    let user = User {
        id: 0, // id is auto generated, so it does not matter
        name: req_obj.name.clone(),
        login_id: req_obj.login_id.clone(),
        password: req_obj.password.clone(),
        email: req_obj.email.clone(),
        user_role: 0,
        created_by: 0,
        modified_by: 0,
        created_on: Utc::now(),
        modified_on: Utc::now(),
        last_login_on: Utc::now(),
    };

    repo.add(user);

    Json(Message{ success: true, message: String::from("User Created!") })
}

#[get("/api/admin/user/{login_id}")]
pub async fn get_user(req: HttpRequest) -> HttpResponse {
    let req_path: String = req.match_info().get("login_id")
        .unwrap().parse().unwrap();

    let repo = get_user_repository();
    
    let parsed_num = req_path.parse::<u32>();

    match parsed_num {
        Ok(user_id) => {
            match repo.get(user_id) {
                Ok(user) => {
                    HttpResponse::Ok().json(user)
                }

                Err(e) => {
                    if e == DBError::NOT_FOUND {
                        return HttpResponse::NotFound()
                            .json(Message {
                                success: false,
                                message: String::from("404 - Not found")
                            });
                    }

                    return HttpResponse::InternalServerError()
                        .json(Message {
                            success: false,
                            message: String::from("500 - Internal Server Error")
                        })
                }
            }
        }
        Err(e) => {
            match repo.get_from_login_id(req_path) {
                Ok(user) => {
                    HttpResponse::Ok().json(user)
                }

                Err(e) => {
                    if e == DBError::NOT_FOUND {
                        return HttpResponse::NotFound()
                            .json(Message {
                                success: false,
                                message: String::from("404 - Not found")
                            });
                    }

                    HttpResponse::InternalServerError()
                        .json(Message {
                            success: false,
                            message: String::from("500 - Internal Server Error")
                        })
                }
            }
        }
    }
}
