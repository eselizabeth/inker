use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use crate::config::InkerConfig;
use actix_web::rt::{spawn, time};
use std::time::{SystemTime, Duration};
use std::fs::{read_dir, metadata};
use crate::generate::{Generator};
use std::path::Path;
use actix_web::rt::time::sleep;

const CHANGE_DURATION: u64 = 3;

#[get("/change")]
async fn change() -> HttpResponse {
    sleep(Duration::from_secs(CHANGE_DURATION)).await;
    let any_change: bool = check_changes().await;
    if any_change {
        println!("{}", any_change);
        send_refresh().await
    }
    else{
        send_norefresh().await
    }
}

async fn send_refresh() -> HttpResponse {
    let message = "refresh";
    HttpResponse::build(actix_web::http::StatusCode::OK)
        .content_type("text/event-stream")
        .append_header(("Cache-Control", "no-cache"))
        .body(message)
}

async fn send_norefresh() -> HttpResponse {
    let message = "norefresh";
    HttpResponse::build(actix_web::http::StatusCode::OK)
        .content_type("text/event-stream")
        .append_header(("Cache-Control", "no-cache"))
        .body(message)
}

#[actix_web::main]
pub async fn run_server(live_reload: bool) -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    println!("web server started at: http://0.0.0.0:8080");
    HttpServer::new(|| App::new()
        .service(change)
        .service(get_posts)
        .service(fs::Files::new("/", "build").index_file("index.html"))
        .service(get_extra)
        .service(fs::Files::new("/build", "build").show_files_listing())
        .service(fs::Files::new("/page", "build/page").show_files_listing())
        .service(fs::Files::new("/static", "build/static").show_files_listing()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}


/// returns the post page or images in the posts
#[get("/posts/{post_name}")]
async fn get_posts(path: web::Path<String>) -> fs::NamedFile {
    let post_name = path.into_inner();
    // If there is a dot it means the request is a file (image)
    if post_name.contains("."){
        let mut folder_name = String::from("none");
        for folder in read_dir(InkerConfig::posts_folder()).unwrap(){
            let post_folder_name = folder.as_ref().unwrap().file_name().into_string().unwrap();
            if Path::new(&format!("build/posts/{}/{post_name}", post_folder_name.to_string())).is_file(){
                folder_name = post_folder_name.to_string().clone();
                break;
            }
        }
        let path = format!("{}/posts/{}/{}", InkerConfig::build_folder(), folder_name, post_name);
        let file = fs::NamedFile::open(path);
        return file.unwrap();
    }
    let path = format!("{}/{}/{}/index.html", InkerConfig::build_folder(), InkerConfig::posts_folder(), post_name);
    let file = fs::NamedFile::open(path);
    return file.unwrap();
}

/// returns the extra pages
#[get("/{path}")]
async fn get_extra(path: web::Path<String>) -> fs::NamedFile {
    let path_str = path.into_inner();
    let path = format!("{}/{}/index.html", InkerConfig::build_folder() ,path_str);
    let file = fs::NamedFile::open(path);
    return file.unwrap();
}

/// Checkes input files for changes
/// InkerConfig::template_folder()
/// InkerConfig::posts_folder()
/// "config.yaml"
async fn check_changes() -> bool{
    let current_time = SystemTime::now();
    let posts_folder_changed = folder_changed(InkerConfig::posts_folder().to_string(), current_time);
    let template_folder_changed = folder_changed(InkerConfig::template_folder().to_string(), current_time);
    let config_changed = file_changed("config.yaml".to_string(), current_time);
    if posts_folder_changed || template_folder_changed || config_changed{
        println!("changes has been found, reloading");
        let mut generator = Generator::new();
        generator.generate(true);
        return true;
    }
    return false;
}

/// iterates over a folder(s) recursively and checkes the if the file(s) inside has been modified in last $CHANGE_DURATION seconds, returns true if it is
fn folder_changed(folder_name: String, current_time: SystemTime) -> bool{
    let mut changed = false;
    for file in read_dir(folder_name).expect("this folder doesn't exist") {
        if file.as_ref().unwrap().file_type().unwrap().is_file()  {
            changed = changed |  file_changed(file.unwrap().path().into_os_string().into_string().unwrap(), current_time);
            }
        else if file.as_ref().unwrap().file_type().unwrap().is_dir(){
            changed = changed | folder_changed(file.unwrap().path().into_os_string().into_string().unwrap(), current_time);
            }
    }
    return changed;
}

/// returns true if the file content has been changed over last $CHANGE_DURATION seconds
fn file_changed(file_name: String, current_time: SystemTime) -> bool{
    let file_metadata = metadata(file_name).unwrap();
    if let Ok(change_time) = file_metadata.modified() {
        let time_difference = current_time.duration_since(change_time); 
        if time_difference.unwrap().as_secs() < CHANGE_DURATION{
            return true;
        };
        }
    else {
        println!("not supported on this platform");
    }
    return false;
}
