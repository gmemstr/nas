#![feature(allocator_api)]

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
    println!("Starting Sliproad on port {}", c.general.port);
    match file_providers::init() {
        Ok(_) => println!("All providers initialized"),
        Err(_) => { println!("A provider failed to initialize"); return }
    }
    match webserver::run_server(c.general.port  ) {
        Ok(_) => println!("Shut down gracefully."),
        Err(v) => println!("Error while shutting down! {}", v)
    }
}
