use std::{ sync::Mutex, ops::DerefMut };
use actix_web::{ get, post, put, delete, web::{ Json, Data }, HttpResponse };

use crate::{
    repository::role::RoleRepository, server::config::ServerConfig,
    db::DBError, model::role::Role, auth::AuthMiddleware,
};


/// Gets all roles.
#[get("/api/admin/roles")]
pub async fn get_all_roles(_: AuthMiddleware, conf: Data<Mutex<ServerConfig>>)
    -> HttpResponse {
    let mut c_ = conf.lock().unwrap();
    let config: &mut ServerConfig = c_.deref_mut();

    if let Ok(mut repo) = config.get_role_repo() {
        match repo.get_all() {
            Ok(roles) => return HttpResponse::Ok().json(roles),
            Err(e) => {
                if e == DBError::NOT_FOUND {
                    return HttpResponse::NotFound().body("Not Found");
                }
            }
        }
        
    }

    HttpResponse::InternalServerError().body("Some Error Occurred")
}

#[post("/api/admin/role")]
pub async fn set_role(
    role: Json<Role>, _: AuthMiddleware, conf: Data<Mutex<ServerConfig>>
) -> HttpResponse {
    let mut c_ = conf.lock().unwrap();
    let config: &mut ServerConfig = c_.deref_mut();

    if let Ok(mut repo) = config.get_role_repo() {
        match repo.add(Role {
            id: role.id,
            role_name: role.role_name.clone(),
            permissions: role.permissions,
        }) {
            Ok (msg) => return HttpResponse::Ok().body(msg),
            Err (msg) => return HttpResponse::InternalServerError().body(msg),
        }
    }

    HttpResponse::InternalServerError().body("Some Error Occurred")
}

#[put("/api/admin/role")]
pub async fn update_role(
    role: Json<Role>, _: AuthMiddleware, conf: Data<Mutex<ServerConfig>>
) -> HttpResponse {
    let mut c_ = conf.lock().unwrap();
    let config: &mut ServerConfig = c_.deref_mut();

    if let Ok(mut repo) = config.get_role_repo() {
        match repo.update(Role {
            id: role.id,
            role_name: role.role_name.clone(),
            permissions: role.permissions,
        }) {
            Ok (msg) => return HttpResponse::Ok().body(msg),
            Err (msg) => return HttpResponse::InternalServerError().body(msg)
        }
    }

    HttpResponse::InternalServerError().body("Some Error Occurred")
}

#[delete("/api/admin/role")]
pub async fn delete_role(
    role: Json<Role>, _: AuthMiddleware, conf: Data<Mutex<ServerConfig>>
) -> HttpResponse {
    let mut c_ = conf.lock().unwrap();
    let config: &mut ServerConfig = c_.deref_mut();

    if let Ok(mut repo) = config.get_role_repo() {
        match repo.remove(Role {
            id: role.id,
            role_name: role.role_name.clone(),
            permissions: role.permissions,
        }) {
            Ok (msg) => return HttpResponse::Ok().body(msg),
            Err (msg) => return HttpResponse::InternalServerError().body(msg)
        }
    }

    HttpResponse::InternalServerError().body("Some Error Occurred")
}

