use std::iter::Map;
use std::fmt::Error;
use std::collections::HashMap;
use crate::configuration::load_config;
use crate::file_providers::disk_provider::DiskProvider;
use crate::file_providers::s3_provider::S3Provider;
use std::fs::File;
use std::any::Any;

#[path = "disk_provider.rs"] pub mod disk_provider;
#[path = "s3_provider.rs"] pub mod s3_provider;

struct Provider {
    name: String,
    location: String,
    properties: Option<HashMap<String, String>>
}

trait FileProvider {
    fn from_provider(provider: Provider) -> Self;
    fn setup(&self) -> bool;
    fn get_directory(&self, path: String) -> Vec<String>;
    fn save_file(&self, path: String, contents: String) -> bool;
    fn create_directory(&self, path: String) -> bool;
    fn delete(&self, path: String) -> bool;
}

pub fn init() -> Result<(), bool> {
    let config = load_config(None);
    let c = match config {
        Ok(v) => v,
        Err(_) => { return Err(false) }
    };

    for (name, p_config) in c.providers {
        let p: Provider = Provider {
            name,
            location: p_config.path,
            properties: p_config.config,
        };
        let x: dyn Any = match p_config.provider.as_str() {
            "disk" => DiskProvider::from_provider(p),
            "s3" => S3Provider::from_provider(p),
            _ => Provider::from(p),
        };
    }

    Ok(())
}
