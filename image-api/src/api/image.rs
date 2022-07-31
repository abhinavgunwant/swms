use std::path::PathBuf;
use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{ get, post, web::{Data}, HttpResponse, Responder };
use actix_form_data::{handle_multipart, Error, Field, FilenameGenerator, Form};
use uuid::Uuid;

// #[post("/api/image")]
// pub async fn upload(mut payload: Multipart, file_path: String) -> impl Responder {
//     while let Ok(Some(mut field)) = payload.try_next().await {
//         let id: Uuid = Uuid::new_v4();
//         let extension = file_path.split('.').collect::<Vec<&str>>().last().unwrap();
//         let new_file_name = format!("{}", Uuid::new_v4());
//         // new_file_name.push_str(format!("{}", id));
//         new_file_name.push_str(".");
//         new_file_name.push_str(extension);

//         println!("{}", new_file_name);
//         // let new_file_name = format!("{}.{}", id, String::from(extension));
        
//         let mut f = web::block(|| std::fs::File::create(new_file_name)).await.unwrap();
        
//         while let Some(chunk) = field.next().await {
//             let data = chunk.unwrap();
//             f = web::block(move || f.write_all(&data).map(|_| f))
//                 .await
//                 .unwrap();
//         }
//     }

//     HttpResponse::Ok()
// }

pub struct FileNamer;

impl FilenameGenerator for FileNamer {
    fn next_filename(&self) -> Option<PathBuf> {
        let mut p = PathBuf::new();
        p.push(format!("uploaded-images/{}.jpg", Uuid::new_v4()));
        Some(p)
    }
}

#[post("/api/image")]
pub async fn upload((mp, state): (Multipart, Data<Form>)) -> impl Responder {
    // Box::new(
        handle_multipart(mp, state.get_ref().clone()).map(|uploaded_content| {
            println!("Uploaded Content: {:?}", uploaded_content);
        });
    // )
    HttpResponse::Ok()
}
