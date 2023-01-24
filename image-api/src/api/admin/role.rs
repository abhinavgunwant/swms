use actix_web::{ get, post, put, web::Json, HttpRequest, HttpResponse };
use serde::{ Serialize, Deserialize };
use chrono::Utc;
use qstring::QString;

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
    get_role_repository().add(Role {
        id: role.id,
        role_name: role.role_name.clone(),
        permissions: role.permissions,
    });

    HttpResponse::Ok().body("Role Created!")
}

