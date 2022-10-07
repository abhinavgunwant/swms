use actix_web::{ web::{ Json }, HttpResponse, cookie::Cookie, post, get };
use serde::{ Serialize, Deserialize };
use crate::db::DBError;
use crate::repository::{
    project::{
        Project, ProjectRepository, get_project_repository, validate_project
    },
    user::{ get_user_repository, User, UserRepository }
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
pub async fn get_projects_for_user(project: Json<UserProjectRequest>) -> HttpResponse {
    let repo = get_project_repository();

    let projects_wrapped = repo.get_user_projects(project.user_id.clone());

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
