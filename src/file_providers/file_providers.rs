use std::iter::Map;
use std::fmt::Error;
use std::collections::HashMap;
use crate::configuration::load_config;
use std::fs::File;
use crate::file_providers::Providers::Disk;

#[path = "disk_provider.rs"] pub mod disk_provider;
#[path = "s3_provider.rs"] pub mod s3_provider;

#[derive(Clone, Debug)]
pub struct Provider {
    pub name: String,
    location: String,
    properties: Option<HashMap<String, String>>
}

#[derive(Clone, Debug)]
pub enum Providers {
    Disk(Provider),
    S3(Provider),
}

pub enum ObjectType {
    Directory(Vec<String>),
    File(String),
    Missing,
}

impl Providers {
    pub fn setup(&self) -> bool {
        match self {
            Providers::Disk(ref p) => { disk_provider::setup(p) }
            Providers::S3(p) => { true }
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            Providers::Disk(p) => { &p.name }
            Providers::S3(p) => { &p.name }
        }
    }

    pub fn get_object(&self, path: String) -> ObjectType {
        match self {
            Providers::Disk(p) => { disk_provider::get_object(&format!("{}/{}", p.location, path)) }
            Providers::S3(p) => { s3_provider::get_object(&format!("{}/{}", p.location, path)) }
        }
    }
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
            "disk" => Box::new(Providers::Disk(p)),
            "s3" => Box::new(Providers::S3(p)),
            _ => continue,
        };

        x.setup();

        provider_list.push(x);
    }

    Ok(provider_list)
}
