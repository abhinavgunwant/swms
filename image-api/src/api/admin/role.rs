use actix_web::{
    get, post, put, delete, web::Json, HttpRequest, HttpResponse
};
use serde::{ Serialize, Deserialize };
use chrono::Utc;

use crate::{
    repository::role::{ get_role_repository, RoleRepository },
    db::DBError,
    model::role::Role,
};

/**
 * Gets all roles.
 */
#[get("/api/admin/roles")]
pub async fn get_all_roles() -> HttpResponse {
    match get_role_repository().get_all() {
        Ok (roles) => HttpResponse::Ok().json(roles),
        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().body("Not Found");
            }

            HttpResponse::InternalServerError().body("Some Error Occurred")
        }
    }
}

#[post("/api/admin/role")]
pub async fn set_role(role: Json<Role>) -> HttpResponse {
    match get_role_repository().add(Role {
        id: role.id,
        role_name: role.role_name.clone(),
        permissions: role.permissions,
    }) {
        Ok (msg) => HttpResponse::Ok().body(msg),
        Err (msg) => HttpResponse::InternalServerError().body(msg)
    }
}

#[put("/api/admin/role")]
pub async fn update_role(role: Json<Role>) -> HttpResponse {
    match get_role_repository().update(Role {
        id: role.id,
        role_name: role.role_name.clone(),
        permissions: role.permissions,
    }) {
        Ok (msg) => HttpResponse::Ok().body(msg),
        Err (msg) => HttpResponse::InternalServerError().body(msg)
    }
}

#[delete("/api/admin/role")]
pub async fn delete_role(role: Json<Role>) -> HttpResponse {
    match get_role_repository().remove(Role {
        id: role.id,
        role_name: role.role_name.clone(),
        permissions: role.permissions,
    }) {
        Ok (msg) => HttpResponse::Ok().body(msg),
        Err (msg) => HttpResponse::InternalServerError().body(msg)
    }
}

