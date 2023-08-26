use actix_web::{ HttpResponse, HttpRequest, get, post, put, delete, web::Json };
use crate::{
    db::DBError,
    repository::folder::{ FolderRepository, get_folder_repository },
    model::folder::Folder,
    api::service::remove::remove_folders,
};

#[get("/api/admin/folder/{folder_id}/")]
pub async fn get_folder (req: HttpRequest) -> HttpResponse {
    let folder_id: u32 = req.match_info().get("folder_id").unwrap().parse()
        .unwrap();

    match get_folder_repository().get(folder_id) {
        Ok (folder) => {
            println!("Got folder id: {}, title: {}", folder.id, folder.title);
            HttpResponse::Ok().json(folder)
        }

        Err (e) => {
            if e == DBError::NOT_FOUND {
                return HttpResponse::NotFound().body("Not Found");
            }

            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[post("/api/admin/folder/")]
pub async fn add_folder (folder: Json<Folder>) -> HttpResponse {
    match get_folder_repository().add(folder.into_inner()) {
        Ok (success) => HttpResponse::Ok().body(success),
        Err (error_msg) => HttpResponse::InternalServerError().body(error_msg),
    }
}

#[put("/api/admin/folder/")]
pub async fn update_folder (folder: Json<Folder>) -> HttpResponse {
    match get_folder_repository().update(folder.into_inner()) {
        Ok (success) => HttpResponse::Ok().body(success),
        Err (error_msg) => HttpResponse::InternalServerError().body(error_msg),
    }
}

#[delete("/api/admin/folder/{folder_id}")]
pub async fn remove_folder (req: HttpRequest) -> HttpResponse {
    let mut folder_ids: Vec<u32>;

    match req.match_info().get("folder_id") {
        Some (folder_id_str) => {
            folder_ids = folder_id_str.split(',').map(|s| s.parse().unwrap())
                .collect();
        }

        None => {
            return HttpResponse::BadRequest().body("No folders supplied");
        }
    }

    match remove_folders(&mut folder_ids) {
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

