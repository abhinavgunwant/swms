use actix_web::{ web::{ Json }, HttpResponse, HttpRequest, post, get };
use serde::{ Serialize, Deserialize };
use qstring::QString;
use crate::{
    db::DBError,
    repository::{
        project::{ ProjectRepository, get_project_repository },
        user::{ get_user_repository, UserRepository }
    },
    model::{ user::User, project::{ Project, validate_project } },
};

#[derive(Serialize)]
pub struct ProjectResponse {
    success: bool,
    message: Vec::<String>,
    projects: Vec::<Project>
}

#[derive(Deserialize)]
pub struct UserProjectRequest {
    user_id: u32,
}

#[derive(Deserialize)]
pub struct AddUserToProjectRequest {
    project_id: u32,
    users: Vec<u32>,
}

#[get("/api/admin/project")]
pub async fn get_projects() -> HttpResponse {
    let repo = get_project_repository();

    let projects_wrapped = repo.get_all();

    match projects_wrapped {
        Ok (projects) => {
            HttpResponse::Ok().json(ProjectResponse {
                success: true,
                message: vec![String::from("Found projects")],
                projects
            })
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().json(ProjectResponse {
                    success: false,
                    message: vec![String::from("Not Found")],
                    projects: vec![],
                });
            }

            HttpResponse::InternalServerError().json(ProjectResponse {
                success: false,
                message: vec![String::from("Internal Server Error")],
                projects: vec![],
            })
        }
    }
}

#[get("/api/admin/projects-for-user")]
pub async fn get_projects_for_user(req: HttpRequest) -> HttpResponse {
    let repo = get_project_repository();

    let auth_header = req.headers().get("Authorization")
        .unwrap().to_str().unwrap();

    if auth_header.chars().count() < 8 {
        return HttpResponse::Unauthorized().json(ProjectResponse {
            success: false,
            message: vec![String::from("Anauthorized")],
            projects: vec![],
        });
    }

    // TODO: Extract the user id here...
    // for now, since jwt setup is pending, i'm extracting it from the repo.
    let login_id = format!("{}", &auth_header[7..]);
    println!("Login ID: {}", login_id);
    let user_repo = get_user_repository();
    let user_res: Result<User, DBError> = user_repo.get_from_login_id(login_id);

    let user: User;

    match user_res {
        Ok (usr) => {
            user = usr;

            println!("user: {} {}", &user.id, user.name.clone());
        }

        Err (_e) => {
            return HttpResponse::Unauthorized().json(ProjectResponse {
                success: false,
                message: vec![String::from("Anauthorized")],
                projects: vec![],
            });
        }
    }

    let projects_wrapped = repo.get_user_projects(user.id);

    match projects_wrapped {
        Ok (projects) => {
            HttpResponse::Ok().json(ProjectResponse {
                success: true,
                message: vec![String::from("Found projects")],
                projects
            })
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().json(ProjectResponse {
                    success: false,
                    message: vec![String::from("Not Found")],
                    projects: vec![],
                });
            }

            HttpResponse::InternalServerError().json(ProjectResponse {
                success: false,
                message: vec![String::from("Internal Server Error")],
                projects: vec![],
            })
        }
    }
}

#[post("/api/admin/project")]
pub async fn add_project(project: Json<Project>) -> HttpResponse {
    let repo = get_project_repository();

    println!("{}", project);

    let (project_is_valid, validation_messages) = validate_project(&project.0);

    if project_is_valid {
        repo.add(project.0);
    
        return HttpResponse::Ok().json(ProjectResponse {
            success: true,
            message: vec![String::from("Project added successfully.")],
            projects: vec![],
        });
    }

    HttpResponse::BadRequest().json(ProjectResponse {
        success: false,
        message: validation_messages,
        projects: vec![],
    })
}

#[post("/api/admin/project/add-users")]
pub async fn add_users_to_project(req_obj: Json<AddUserToProjectRequest>) -> HttpResponse {
    let repo = get_project_repository();

    repo.add_users_to_project(req_obj.project_id, &req_obj.users);

    HttpResponse::Ok().json(ProjectResponse {
        success: true,
        message: vec![String::from("Project added successfully.")],
        projects: vec![],
    })
}

#[get("/api/admin/project/validate-slug")]
pub async fn validate_slug(req: HttpRequest) -> HttpResponse {
    let qs = QString::from(req.query_string());

    match qs.get("slug") {
        Some (slug_qs) => {
            let slug = String::from(slug_qs).to_lowercase();

            let repo = get_project_repository();

            match repo.validate_project_slug(slug) {
                Ok (valid) => HttpResponse::Ok().json(valid),
                Err (_e) => HttpResponse::InternalServerError()
                    .body("Internal Server Error")
            }
        }

        None => {
            HttpResponse::BadRequest().body("Bad Request")
        }
    }
}

