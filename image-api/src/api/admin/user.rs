use actix_web::{ get, post, web::Json, HttpRequest, HttpResponse };
use serde::{ Serialize, Deserialize };
use chrono::Utc;
use qstring::QString;

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
#[serde(rename_all = "camelCase")]
pub struct UserResponseMessage {
    success: bool,
    message: String,
    user_id: Option<u32>,
}

#[post("/api/admin/user")]
pub async fn create_user(req_obj: Json<CreateUserRequest>) -> HttpResponse {
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

    match get_user_repository().add(user) {
        Ok (id) => {
            HttpResponse::Ok().json(UserResponseMessage{
                success: true,
                message: String::from("User Created!"),
                user_id: Some(id),
            })
        }

        Err (_e) => {
            eprintln!("{}", _e);
            HttpResponse::Ok().json(UserResponseMessage{
                success: false,
                message: String::from("User Created!"),
                user_id: None,
            })
        }
    }
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
                            .json(UserResponseMessage {
                                success: false,
                                message: String::from("404 - Not found"),
                                user_id: None,
                            });
                    }

                    return HttpResponse::InternalServerError()
                        .json(UserResponseMessage {
                            success: false,
                            message: String::from("500 - Internal Server Error"),
                            user_id: None,
                        })
                }
            }
        }
        Err(_e) => {
            match repo.get_from_login_id(req_path) {
                Ok(user) => {
                    HttpResponse::Ok().json(user)
                }

                Err(e) => {
                    if e == DBError::NOT_FOUND {
                        return HttpResponse::NotFound()
                            .json(UserResponseMessage {
                                success: false,
                                message: String::from("404 - Not found"),
                                user_id: None,
                            });
                    }

                    HttpResponse::InternalServerError()
                        .json(UserResponseMessage {
                            success: false,
                            message: String::from("500 - Internal Server Error"),
                            user_id: None,
                        })
                }
            }
        }
    }
}

#[get("/api/admin/search/user")]
pub async fn search_user(req: HttpRequest) -> HttpResponse {
    let qs = QString::from(req.query_string());

    let user_query = qs.get("name").unwrap().trim();
    let repo = get_user_repository();
    let su_result = repo.search_from_name(String::from(user_query), 10);

    if user_query.is_empty() {
        return HttpResponse::NotFound().body("Not Found");
    }

    match su_result {
        Ok (su) => {
            HttpResponse::Ok().json(su)
        }

        Err(e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().body("Not Found");
            }

            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

