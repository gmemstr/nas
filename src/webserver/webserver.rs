// This file handles the routing and starting of the webserver. Routes are dispatched to their
// respective modules.
use actix_web::{get, web, App, HttpServer, Responder};
use crate::file_providers::FileProvider;
use serde_json::json;
use std::cell::Cell;
use std::borrow::Borrow;

#[derive(Clone)]
struct AppState {
    providers: Cell<Vec<Box<dyn FileProvider>>>,
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello, world!")
}

#[get("/providers")]
async fn provider_index() -> impl Responder {
    format!("X")
}

#[get("/files/{provider}/{directory:.*}")]
async fn directory_file_index(web::Path((provider, directory)): web::Path<(String, String)>) -> impl Responder {
    format!("provider:{}\ndirectory:{}", provider, directory)
}

#[get("/files/{provider}")]
async fn dir_index(data: web::Data<AppState>, p: web::Path<String>) -> impl Responder {
    let providers = &data.providers;
    for provider in providers.borrow().into_inner() {
        if provider.get_name().to_string() == p.to_string() {
            return format!("{}", json!(provider.get_directory("".to_string())))
        }
    }

    format!("[]")
}

#[actix_web::main]
pub async fn run_server(port: i32, providers: Vec<Box<dyn FileProvider>>) -> std::io::Result<()> {
    let app_state = AppState {
        providers: Cell::new(providers),
    };
    HttpServer::new(|| App::new()
        .service(index)
        .service(web::scope("/api")
            .service(dir_index)
            .service(provider_index)
            .service(directory_file_index)
        )
        .data(app_state.clone()))
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}
