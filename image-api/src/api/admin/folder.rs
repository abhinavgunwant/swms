use actix_web::{ HttpResponse, HttpRequest, get, post, put, delete, web::Json };
use crate::{
    db::DBError,
    repository::folder::{ FolderRepository, get_folder_repository },
    model::folder::Folder,
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
    let folder_id: u32 = req.match_info().get("folder_id").unwrap().parse()
        .unwrap();

    match get_folder_repository().remove_item(folder_id) {
        Ok (success) => HttpResponse::Ok().body(success),
        Err (error_msg) => HttpResponse::InternalServerError().body(error_msg),
    }
}

