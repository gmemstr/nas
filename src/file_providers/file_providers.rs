use std::iter::Map;
use std::fmt::Error;
use std::collections::HashMap;
use crate::configuration::load_config;
use crate::file_providers::disk_provider::DiskProvider;
use crate::file_providers::s3_provider::S3Provider;
use std::fs::File;

#[path = "disk_provider.rs"] pub mod disk_provider;
#[path = "s3_provider.rs"] pub mod s3_provider;

#[derive(Clone, Debug)]
pub enum Providers {
    D(Provider),
    Disk(DiskProvider),
    S3(S3Provider),
}

impl Providers {
    pub fn get(&self) -> Self {
        println!("{:?}", self);
        match self {
            Providers::Disk(p) => p,
            Providers::S3(p) => p,
            Providers::D(_) => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct Provider {
    pub name: String,
    location: String,
    properties: Option<HashMap<String, String>>
}

pub trait FileProvider {
    fn setup(&self) -> bool;
    fn get_name(&self) -> &str;
    fn get_directory(&self, path: String) -> Vec<String>;
    fn save_file(&self, path: String, contents: String) -> bool;
    fn create_directory(&self, path: String) -> bool;
    fn delete(&self, path: String) -> bool;
}

pub fn init() -> Result<Vec<Box<Providers>>, bool> {
    let mut provider_list: Vec<Box<Providers>> = Vec::new();
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
        let x: Box<Providers> = match p_config.provider.as_str() {
            "disk" => Box::new(Providers::Disk(DiskProvider(p))),
            "s3" => Box::new(Providers::S3(S3Provider(p))),
            _ => continue,
        };

        provider_list.push(x);
    }

    Ok(provider_list)
}
