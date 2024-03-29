use actix_web::{
    get, post, put, web::{ Json, Data }, HttpRequest, HttpResponse
};
use serde::{ Serialize, Deserialize };
use chrono::Utc;
use qstring::QString;
use log::{ info, error };

use crate::{
    repository::Repository,
    server::db::DBError, model::user::User, auth::AuthMiddleware,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    login_id: String,
    password: String,
    name: String,
    email: String,
    user_role: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponseMessage {
    success: bool,
    message: String,
    user_id: Option<u32>,
}

#[derive(Serialize)]
pub struct UserListResponseMessage {
    success: bool,
    message: String,
    users: Vec<EditUserRequest>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditUserRequest {
    pub id: u32,
    pub login_id: String,
    pub name: String,
    pub email: String,
    pub user_role: u8,
}

#[post("/api/admin/user")]
pub async fn create_user(
    repo: Data<dyn Repository + Sync + Send>,
    req_obj: Json<CreateUserRequest>,
    _: AuthMiddleware
) -> HttpResponse {
    let user = User {
        id: 0, // id is auto generated, so it does not matter
        name: req_obj.name.clone(),
        login_id: req_obj.login_id.clone(),
        password: req_obj.password.clone(),
        email: req_obj.email.clone(),
        user_role: req_obj.user_role.clone(),
        created_by: 0,
        modified_by: 0,
        created_on: Utc::now(),
        modified_on: Utc::now(),
        last_login_on: Utc::now(),
    };

    match repo.get_user_repo() {
        Ok(mut user_repo) => {
            match user_repo.add(user) {
                Ok (id) => {
                    info!("Adding user: (id: {})", id);

                    HttpResponse::Ok().json(UserResponseMessage{
                        success: true,
                        message: String::from("User Created!"),
                        user_id: Some(id),
                    })
                }

                Err (e) => {
                    error!("Error while adding user: {}", e);

                    HttpResponse::Ok().json(UserResponseMessage {
                        success: false,
                        message: String::from(
                            "Some error occured, please try again!"
                        ),
                        user_id: None,
                    })
                }
            }
        }

        Err(e) => {
            error!("Error while getting user repository: {}", e);

            HttpResponse::Ok().json(UserResponseMessage {
                success: false,
                message: String::from("Internal Server Error!"),
                user_id: None,
            })
        }
    }
}

/**
 * Edits user by replacing current user data with supplied data.
 *
 * Note: Using `UserListing` struct here since it resembles the field we want
 * to submit when changing user attributes.
 */
#[put("/api/admin/user")]
pub async fn edit_user(
    repo: Data<dyn Repository + Sync + Send>,
    req_obj: Json<EditUserRequest>,
    _: AuthMiddleware
) -> HttpResponse {
    match repo.get_user_repo() {
        Ok(mut user_repo) => {
            match user_repo.update(User {
                id: req_obj.id,
                name: req_obj.name.clone(),
                login_id: req_obj.login_id.clone(),
                password: String::from(""),
                email: req_obj.email.clone(),
                user_role: req_obj.user_role,
                created_by: 0,
                modified_by: 0,
                created_on: Utc::now(),
                modified_on: Utc::now(),
                last_login_on: Utc::now(),
            }) {
                Ok (_) => HttpResponse::Ok().body("User updated!"),
                Err (e) => HttpResponse::InternalServerError().body(e),
            }
        }

        Err(e) => {
            error!("Error while getting user repository: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[get("/api/admin/user/{login_id}")]
pub async fn get_user(
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest,
    _: AuthMiddleware
) -> HttpResponse {
    let req_path: String = req.match_info().get("login_id")
        .unwrap().parse().unwrap();

    let mut user_repo;

    let parsed_num = req_path.parse::<u32>();

    match repo.get_user_repo() {
        Ok(u_repo) => { user_repo = u_repo; }

        Err(e) => {
            error!("Error while getting user repository: {}", e);

            return HttpResponse::InternalServerError()
                .body("Some internal error occured!");
        }
    }

    match parsed_num {
        Ok(user_id) => {
            match user_repo.get(user_id) {
                Ok(user) => {
                    HttpResponse::Ok().json(user)
                }

                Err(e) => {
                    match e {
                        DBError::NotFound => {
                            return HttpResponse::NotFound()
                                .json(UserResponseMessage {
                                    success: false,
                                    message: String::from("404 - Not found"),
                                    user_id: None,
                                });
                        }

                        _ => {
                            return HttpResponse::InternalServerError()
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

        Err(_e) => {
            match user_repo.get_from_login_id(req_path) {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(e) => {
                    match e {
                        DBError::NotFound => HttpResponse::NotFound()
                            .json(UserResponseMessage {
                                success: false,
                                message: String::from("404 - Not found"),
                                user_id: None,
                            }),
                        _ => HttpResponse::InternalServerError()
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
}

#[get("/api/admin/users")]
pub async fn get_user_list(
    repo: Data<dyn Repository + Sync + Send>,
    _: AuthMiddleware
) -> HttpResponse {
    match repo.get_user_repo() {
        Ok(mut user_repo) => {
            match user_repo.get_all() {
                Ok (users) => {
                    let mut user_list: Vec<EditUserRequest> = vec![];

                    for user in users.iter() {
                        user_list.push(EditUserRequest {
                            id: user.id.clone(),
                            login_id: user.login_id.clone(),
                            name: user.name.clone(),
                            email: user.email.clone(),
                            user_role: user.user_role.clone(),
                        });
                    }

                    HttpResponse::Ok().json(UserListResponseMessage {
                        success: true,
                        message: String::from("Got Users"),
                        users: user_list,
                    })
                }

                Err (e) => {
                    error!("Error while fetching all users: {}", e);

                    HttpResponse::InternalServerError().json(
                        UserListResponseMessage {
                            success: false,
                            message: String::from("Internal Server Error"),
                            users: vec![],
                        })
                }
            }
        }

        Err(e) => {
            error!("Error while getting user repository: {}", e);

            HttpResponse::InternalServerError().json(UserListResponseMessage {
                success: false,
                message: String::from("Internal Server Error"),
                users: vec![],
            })
        }
    }
}

#[get("/api/admin/search/user")]
pub async fn search_user(
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest,
    _: AuthMiddleware
) -> HttpResponse {
    let qs = QString::from(req.query_string());

    let user_query = qs.get("name").unwrap().trim();

    if user_query.is_empty() {
        return HttpResponse::NotFound().body("Not Found");
    }

    match repo.get_user_repo() {
        Ok(mut user_repo) => {
            match user_repo.search_from_name(String::from(user_query), 10) {
                Ok (su) => HttpResponse::Ok().json(su),

                Err(e) => {
                    match e {
                        DBError::NotFound => HttpResponse::NotFound()
                            .body("Not Found"),
                        _ => HttpResponse::InternalServerError()
                            .body("Internal Server Error")
                    }
                }
            }
        }

        Err(e) => {
            error!("Error while getting user repository: {}", e);

            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

