use crate::configuration::Configuration;
use std::borrow::Borrow;

#[path = "configuration/configuration.rs"] pub mod configuration;
#[path = "file_providers/file_providers.rs"] pub mod file_providers;
#[path = "webserver/webserver.rs"] pub mod webserver;

fn main() {
    let config = configuration::load_config(None);
    let c = match config {
        Ok(x) => { println!("Configuration loaded"); x },
        Err(_) => { println!("Failed to load configuration"); return }
    };
    let port = match c.general {
        Some(e) => e.port,
        None => 8080
    };
    println!("Starting Sliproad on port {}", port);
    let p = match file_providers::init() {
        Ok(x) => {
            println!("All providers initialized");
            x
        },
        Err(_) => { println!("A provider failed to initialize"); return }
    };
    match webserver::run_server(port, p) {
        Ok(_) => println!("Shut down gracefully."),
        Err(v) => println!("Error while shutting down! {}", v)
    }
}
