// This file handles the routing and starting of the webserver. Routes are dispatched to their
// respective modules.
use actix_web::{get, web, App, HttpServer, Responder};
use crate::file_providers::{Provider, Providers, ObjectType};
use serde_json::json;
use std::cell::Cell;
use std::borrow::Borrow;

#[derive(Clone)]
struct AppState {
    providers: Vec<Providers>,
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello, world!")
}

#[get("/providers")]
async fn provider_index(data: web::Data<AppState>) -> impl Responder {
    let providers = &data.providers;
    let provider_names: Vec<_> = providers.iter().map(|x| x.get_name()).collect();
    format!("{}", json!(provider_names))
}

#[get("/files/{path:.*}")]
async fn dir_index(data: web::Data<AppState>, p: web::Path<String>) -> impl Responder {
    let providers = &data.providers;

    // We can assume the first item is our provider.
    let mut splitter = p.splitn(1, "/");
    let provider_name = splitter.next().unwrap();
    let path = match splitter.next() {
        None => "",
        Some(x) => x,
    };

    for provider in providers {
        if provider.get_name() == &provider_name.to_string() {
            return match provider.get_object(path.to_string()) {
                ObjectType::Directory(list) => format!("{}", json!(list)),
                ObjectType::File(file) => format!("{}", file),
                ObjectType::Missing => format!("Not Found"),
            }
        }
    }

    format!("[]")
}

#[actix_web::main]
pub async fn run_server(port: i32, providers: Vec<Providers>) -> std::io::Result<()> {
    let app_state = AppState {
        providers,
    };
    HttpServer::new(move || App::new()
        .service(index)
        .service(web::scope("/api")
            .service(dir_index)
            .service(provider_index)
        )
        .data(app_state.clone()))
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}
