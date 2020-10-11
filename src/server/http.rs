use actix_web::{get, App, HttpRequest, HttpServer, HttpResponse, web};
use std::path::PathBuf;
use actix_files::NamedFile;
use crate::server::html::generate;
use crate::directory::walk::{visit};
use actix_web::http::header::{ContentDisposition, DispositionType};
use ansi_term::Colour;

#[get("/file/{path:.*}")]
async fn files(req: HttpRequest) -> NamedFile {
    let path: PathBuf = req.match_info().query("path").parse().unwrap();
    println!("{}  [{}]",
             Colour::Red.bold().paint(":: File\t\t\t\t"),
             Colour::Cyan.bold().paint(path.to_str().unwrap().to_string()));
    NamedFile::open(path).unwrap()
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        })
}


#[get("/{path:.*}")]
async fn index(req: HttpRequest) -> HttpResponse {
    let empty_path: PathBuf = PathBuf::from("");
    let current_path: PathBuf = PathBuf::from("./");
    let path: PathBuf = req.match_info().query("path").parse().unwrap();
    println!("{}  [{}]",
             Colour::Red.bold().paint(":: Dir\t\t\t\t"),
             Colour::Yellow.bold().paint(path.to_str().unwrap().to_string()));
    let final_path = if path == empty_path { current_path } else { path };
    HttpResponse::Ok().body(generate(visit(final_path)))
}


#[actix_web::main]
pub async fn serve() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(
            web::scope("")
                .service(files)
                .service(index)
        )
    )
        .bind("0.0.0.0:8989").expect(
        &format!("{}  [{}]",
                 Colour::Red.bold().paint("PORT is Already in USE"),
                 Colour::Cyan.bold().paint("8989")))
        .run()
        .await
}