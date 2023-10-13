use actix_web::{
    HttpResponse, HttpRequest, get, post, put, delete, web::{ Json, Data },
};
use qstring::QString;
use log::{ debug, error };

use crate::{
    db::DBError, auth::AuthMiddleware, server::config::ServerConfig,
    repository::Repository,
    model::folder::Folder, api::service::remove::remove_folders,
};

#[get("/api/admin/folder/{folder_id}/")]
pub async fn get_folder (
    repo: Data<dyn Repository + Sync + Send>,
    req: HttpRequest, _: AuthMiddleware
) -> HttpResponse {
    let folder_id: u32 = req.match_info().get("folder_id").unwrap().parse()
        .unwrap();

    match repo.get_folder_repo() {
        Ok(fol_repo) => {
            match fol_repo.get(folder_id) {
                Ok(folder) => {
                    debug!(
                        "Got folder id: {}, title: {}", folder.id, folder.title
                    );

                    HttpResponse::Ok().json(folder)
                }

                Err(e) => {
                    if e == DBError::NOT_FOUND {
                        return HttpResponse::NotFound().body("Not Found");
                    }

                    HttpResponse::InternalServerError()
                        .body("Internal Server Error")
                }
            }
        }

        Err(_) => HttpResponse::InternalServerError()
            .body("Internal Server Error")
    }
}

#[post("/api/admin/folder/")]
pub async fn add_folder (
    repo: Data<dyn Repository + Sync + Send>, folder: Json<Folder>,
    _: AuthMiddleware
) -> HttpResponse {
    match repo.get_folder_repo() {
        Ok(fol_repo) => {
            match fol_repo.add(folder.into_inner()) {
                Ok(success) => HttpResponse::Ok().body(success),
                Err(error_msg) => HttpResponse::InternalServerError()
                    .body(error_msg),
            }
        }
        
        Err(e) => {
            error!("Error while getting folder repo: {}", e);

            HttpResponse::InternalServerError()
                .body("Some internal error occured.")
        }
    }
}

#[put("/api/admin/folder/")]
pub async fn update_folder (
    repo: Data<dyn Repository + Sync + Send>, folder: Json<Folder>,
    _: AuthMiddleware
) -> HttpResponse {
    match repo.get_folder_repo() {
        Ok(fol_repo) => {
            match fol_repo.update(folder.into_inner()) {
                Ok (success) => HttpResponse::Ok().body(success),
                Err (error_msg) => HttpResponse::InternalServerError().body(error_msg),
            }
        }

        Err(_) => HttpResponse::InternalServerError()
            .body("Some error occured!")
    }
}

/// Deletes folder(s)
///
/// ## URL parameters:
/// - `id` - Comma-separated folder IDs.
#[delete("/api/admin/folder")]
pub async fn remove_folder (
    repo: Data<dyn Repository + Sync + Send>, req: HttpRequest,
    _: AuthMiddleware, conf: Data<ServerConfig>
) -> HttpResponse {
    let qs = QString::from(req.query_string());

    let mut folder_ids: Vec<u32>;

    match qs.get("id") {
        Some (qid) => {
            folder_ids = qid.split(',').map(|s| s.parse().unwrap()).collect();
        }

        None => {
            return HttpResponse::BadRequest().body("No folder IDs supplied");
        }
    }

    match remove_folders(
        repo,
        &mut folder_ids,
        conf.rendition_cache_dir.clone(),
        conf.upload_dir.clone(),
    ) {
        Ok (_) => {
            if folder_ids.len() > 1 {
                return HttpResponse::Ok().body("Folders deleted successfully");
            } else {
                return HttpResponse::Ok().body("Folder deleted successfully");
            }
        }

        Err (_) => {
            if folder_ids.len() > 1 {
                return HttpResponse::InternalServerError()
                    .body("Some folders could not be deleted successfully");
            } else {
                return HttpResponse::InternalServerError()
                    .body("An error occurred while deleting the folder.");
            }
        }
    }
}

