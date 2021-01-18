// This file handles the routing and starting of the webserver. Routes are dispatched to their
// respective modules.
use actix_web::{get, post, web, App, HttpServer, Responder, Result, HttpRequest};
use actix_multipart::Multipart;
use crate::file_providers::{Provider, Providers, ObjectType};
use serde_json::json;
use std::cell::Cell;
use std::borrow::Borrow;
use std::path::{PathBuf, Path};
use actix_files::NamedFile;
use actix_web::rt::blocking::BlockingError;
use serde::export::fmt::Error;
use std::fs::File;
use futures::{StreamExt, TryStreamExt};

#[derive(Clone)]
struct AppState {
    providers: Vec<Providers>,
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let mut path = PathBuf::new();
    let mut filename: String = req.match_info().query("filename").parse().unwrap();
    if filename == "" {
        filename = "index.html".to_string();
    }
    path.push("web");
    path.push(filename);
    Ok(NamedFile::open(path)?)
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
    let (provider_name, path) = extract_from_path(&p);

    for provider in providers {
        if provider.get_name().to_string() == provider_name {
            return match provider.get_object(path) {
                ObjectType::Directory(list) => format!("{}", json!(list)),
                ObjectType::File(file) => format!("{}", file),
                ObjectType::Missing => format!("Not Found"),
            }
        }
    }

    format!("[]")
}

#[post("/files/{path:.*}")]
async fn save_object(mut payload: Multipart, p: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let providers = &data.providers;
    let (provider_name, path) = extract_from_path(&p);

    let mut real_provider = Provider::new();

    for provider in providers {
        if provider.get_name().to_string() == provider_name {
            real_provider = provider.clone()
        }
    }

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("{}/{}", path, filename);

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let provider = real_provider.clone();
            let filepath = filepath.clone();
            let contents = chunk.unwrap();

            let r = web::block(move || provider.save_object(&filepath, &*contents)).await;
        }
    };

    format!("[]")
}

// Extract providers and (possible) directory path.
fn extract_from_path(full_string: &str) -> (String, String) {
    // We can assume the first item is our provider.
    let mut splitter = full_string.splitn(2, '/');
    let provider_name = splitter.next().unwrap();
    let path = match splitter.next() {
        None => "",
        Some(x) => x,
    };

    return (provider_name.to_string(), path.to_string())
}

#[actix_web::main]
pub async fn run_server(port: i32, providers: Vec<Providers>) -> std::io::Result<()> {
    let app_state = AppState {
        providers,
    };
    HttpServer::new(move || App::new()
        .service(web::scope("/api")
            .service(dir_index)
            .service(provider_index)
            .service(save_object)
        )
        .route("/{filename:.*}", web::get().to(index))
        .data(app_state.clone()))
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}
