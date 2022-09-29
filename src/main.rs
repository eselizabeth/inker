pub mod generate;
pub mod cli;
pub mod file_handler;
pub mod config;
pub mod webserver;
use crate::cli::{Cli};
use std::{env, process};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use webserver::{run_server};


fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = Cli::new(&args).unwrap_or_else(|err| {
        println!("inker failed: {err}");
        process::exit(1);
    });

    cli.handle_input();
    run_server();
}

