// This file handles the routing and starting of the webserver. Routes are dispatched to their
// respective modules.
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/providers")]
async fn provider_index() -> impl Responder {
    format!("X")
}

#[get("/files/{provider}")]
async fn file_index(provider: web::Path<String>) -> impl Responder {
    format!("provider:{}", provider)
}

#[get("/files/{provider}/{directory:.*}")]
async fn directory_file_index(web::Path((provider, directory)): web::Path<(String, String)>) -> impl Responder {
    format!("provider:{}\ndirectory:{}", provider, directory)
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello, world!")
}

#[actix_web::main]
pub async fn run_server(port: i32) -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(web::scope("/api")
            .service(provider_index)
            .service(file_index)
            .service(directory_file_index)
        ))
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}
