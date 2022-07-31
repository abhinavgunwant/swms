mod api;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use actix_form_data::{Field, Form};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
// #[post("/image")]
// async fn image() -> impl Responder {
//     HttpResponse::Ok().body("This API shall return a processed image")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let form = Form::new()
        .field("files", Field::array(Field::file(api::image::FileNamer)));
    HttpServer::new(|| {
        let generated = generate();
        App::new()
            .data(form.clone())
            .service(api::echo)
            .service(api::image::upload)
            // .service(hello)
            // .service(image)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
