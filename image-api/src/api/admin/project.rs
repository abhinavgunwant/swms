use actix_web::{
    web::{ Json, Data }, HttpResponse, HttpRequest, post, get, delete,
};
use serde::{ Serialize, Deserialize };
use qstring::QString;
use log::{ debug, error, info };

use crate::{
    auth::AuthMiddleware, server::db::DBError, repository::Repository,
    model::{ user::User, project::{ Project, validate_project } },
};

#[derive(Serialize)]
pub struct ProjectResponse {
    success: bool,
    message: Vec::<String>,
    projects: Vec::<Project>
}

#[derive(Deserialize)]
pub struct AddUserToProjectRequest {
    project_id: u32,
    users: Vec<u32>,
}

#[get("/api/admin/projects")]
pub async fn get_projects(
     repo: Data<dyn Repository + Sync + Send>,   
    _: AuthMiddleware
) -> HttpResponse {
    match repo.get_project_repo() {
        Ok(mut proj_repo) => {
            match proj_repo.get_all() {
                Ok (projects) => {
                    HttpResponse::Ok().json(ProjectResponse {
                        success: true,
                        message: vec![String::from("Found projects")],
                        projects
                    })
                }

                Err (e) => {
                    match e {
                        DBError::NotFound => HttpResponse::NotFound().json(
                            ProjectResponse {
                                success: false,
                                message: vec![String::from("Not Found")],
                                projects: vec![],
                        }),

                        _ => HttpResponse::InternalServerError().json(
                            ProjectResponse {
                            success: false,
                            message: vec![String::from("Internal Server Error")],
                            projects: vec![],
                        })
                    }
                }
            }
        }

        Err(e) => {
            error!("Error while getting user repository: {}", e);

            HttpResponse::InternalServerError().json(ProjectResponse {
                success: false,
                message: vec![String::from("Internal Server Error")],
                projects: vec![],
            })
        }
    }
}

#[delete("/api/admin/project/{project_id}")]
pub async fn remove_project(
    repo: Data<dyn Repository + Sync + Send>,
    _: AuthMiddleware,
    req: HttpRequest,
) -> HttpResponse {
    let project_id: u16;

    match req.match_info().get("project_id") {
        Some(p_id_str) => {
            match p_id_str.parse::<u16>() {
                Ok(p_id_u16) => { project_id = p_id_u16; }
                Err(e) => {
                    error!("Error getting project id from request: {}", e);

                    return HttpResponse::BadRequest()
                        .body("Invalid project id");
                }
            }
        }

        None => {
            return HttpResponse::BadRequest().body("Invalid project id");
        }
    }

    match repo.get_project_repo() {
        Ok(mut proj_repo) => {
            match proj_repo.remove_item(project_id) {
                Ok(msg) => HttpResponse::Ok().body(msg),
                Err(msg) => HttpResponse::InternalServerError().body(msg),
            }
        }

        Err(e) => {
            error!("Error while getting project repo: {}", e);

            HttpResponse::InternalServerError()
                .body("Could not process this request due to an internal \
                    server error.")
        }
    }
}

#[get("/api/admin/projects-for-user")]
pub async fn get_projects_for_user(
    repo: Data<dyn Repository + Sync + Send>,
    auth: AuthMiddleware,
    _: AuthMiddleware
) -> HttpResponse {
    debug!("getting projects for user");

    if !auth.authorized {
        info!("User is unauthorized.");

        return HttpResponse::Unauthorized().json(ProjectResponse {
            success: false,
            message: vec![String::from("Unauthorized")],
            projects: vec![],
        });
    }

    let user: User;

    match repo.get_user_repo() {
        Ok(mut user_repo) => {
            match user_repo.get_from_login_id(auth.login_id) {
                Ok (usr) => {
                    user = usr;

                    debug!("user: {} {}", &user.id, user.name.clone());
                }

                Err (_e) => {
                    return HttpResponse::InternalServerError().json(
                        ProjectResponse {
                            success: false,
                            message: vec![
                                String::from("Some unknown error occured!")
                            ],
                            projects: vec![],
                        });
                }
            }
        }

        Err(e) => {
            error!("Error while getting user repository: {}", e);

            return HttpResponse::InternalServerError().json(ProjectResponse {
                success: false,
                message: vec![String::from("Internal server error!")],
                projects: vec![],
            });
        }
    }


    match repo.get_project_repo() {
        Ok(mut proj_repo) => {
            match proj_repo.get_user_projects(user.id) {
                Ok (projects) => {
                    HttpResponse::Ok().json(ProjectResponse {
                        success: true,
                        message: vec![String::from("Found projects")],
                        projects
                    })
                }

                Err (e) => {
                    match e {
                        DBError::NotFound => HttpResponse::NotFound().json(
                            ProjectResponse {
                                success: false,
                                message: vec![String::from("Not Found")],
                                projects: vec![],
                        }),

                        _ => HttpResponse::InternalServerError().json(ProjectResponse {
                            success: false,
                            message: vec![String::from("Internal Server Error")],
                            projects: vec![],
                        }),
                    }
                }
            }
        }

        Err(e) => {
            error!("Error while getting project repository: {}", e);

            HttpResponse::InternalServerError().json(ProjectResponse {
                success: false,
                message: vec![String::from("Internal Server Error")],
                projects: vec![],
            })
        }
    }
}

#[post("/api/admin/project")]
pub async fn add_project(
    repo: Data<dyn Repository + Sync + Send>,
    project: Json<Project>,
    _: AuthMiddleware
) -> HttpResponse {
    debug!("{}", project);

    let (project_is_valid, validation_messages) = validate_project(&project.0);

    if project_is_valid {
        match repo.get_project_repo() {
            Ok(mut proj_repo) => {
                proj_repo.add(project.0);
            
                return HttpResponse::Ok().json(ProjectResponse {
                    success: true,
                    message: vec![String::from("Project added successfully.")],
                    projects: vec![],
                });
            }

            Err(e) => {
                error!("Error while getting project repository: {}", e);

                return HttpResponse::InternalServerError()
                    .body("Some internal error occured");
            }
        }
    }

    HttpResponse::BadRequest().json(ProjectResponse {
        success: false,
        message: validation_messages,
        projects: vec![],
    })
}

#[post("/api/admin/project/add-users")]
pub async fn add_users_to_project(
    repo: Data<dyn Repository + Sync + Send>,
    req_obj: Json<AddUserToProjectRequest>,
    _: AuthMiddleware
) -> HttpResponse {
    match repo.get_project_repo() {
        Ok(mut proj_repo) => {
            proj_repo.add_users_to_project(req_obj.project_id, &req_obj.users);

            HttpResponse::Ok().json(ProjectResponse {
                success: true,
                message: vec![String::from("Project added successfully.")],
                projects: vec![],
            })
        }

        Err(_) => {
            HttpResponse::InternalServerError().json(ProjectResponse {
                success: false,
                message: vec![String::from("Internal server error")],
                projects: vec![],
            })
        }
    }
}

#[get("/api/admin/project/validate-slug")]
pub async fn validate_slug(
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest,
    _: AuthMiddleware
) -> HttpResponse {
    let qs = QString::from(req.query_string());

    match qs.get("slug") {
        Some (slug_qs) => {
            let slug = String::from(slug_qs).to_lowercase();

            match repo.get_project_repo() {
                Ok(mut proj_repo) => {
                    match proj_repo.is_valid_new_slug(slug) {
                        Ok (valid) => HttpResponse::Ok().json(valid),
                        Err (_e) => HttpResponse::InternalServerError()
                            .body("Internal Server Error")
                    }
                }

                Err (_e) => HttpResponse::InternalServerError()
                    .body("Internal Server Error")
            }
        }

        None => {
            HttpResponse::BadRequest().body("Bad Request")
        }
    }
}

