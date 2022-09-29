use actix_files as fs;
//use actix_files::{fs, NamedFile};
use actix_web::{get, web};
use serde::Deserialize;
use crate::config::InkerConfig;

#[actix_web::main]
pub async fn run_server() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    println!("webserver started at: 127.0.0.1:8080");
    HttpServer::new(|| App::new().service(index)
        .service(post)
        .service(fs::Files::new("/build", "build").show_files_listing())
        .service(fs::Files::new("/posts/static", "build/static").show_files_listing()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Deserialize)]
struct Info {
    page: Option<String>,
}

/// returns the index page
#[get("/")]
async fn index(info: web::Query<Info>) -> fs::NamedFile {
    let path;
    match &info.page {
        Some(page_number) => path = format!("{}/index{}.html", InkerConfig::build_folder(), page_number),
        None => path = format!("{}/index1.html", InkerConfig::build_folder()),
    }
    let file = fs::NamedFile::open(path);
    return file.unwrap();
}


/// returns the post page
#[get("/posts/{post_name}")]
async fn post(path: web::Path<String>) -> fs::NamedFile {
    let post_name = path.into_inner();
    let path = format!("{}/{}/{}/{}.html", InkerConfig::build_folder(), InkerConfig::posts_folder(), post_name, post_name);
    let file = fs::NamedFile::open(path);
    return file.unwrap();
}
