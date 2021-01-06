use std::fs;
use std::collections::HashMap;
use crate::file_providers::{Provider, Providers, ObjectType};
use serde_json::value::Value::Object;
use std::fs::Metadata;
use std::io::Error;

pub fn setup(provider: &Provider) -> bool {
    match fs::create_dir(provider.location.clone()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_object(path: &String) -> ObjectType {
    let location = path;
    let meta = match fs::metadata(location) {
        Ok(m) => { m }
        Err(_) => { return ObjectType::Missing }
    };

    return if meta.is_dir() {
        let paths = fs::read_dir(format!("{}", &location)).unwrap();
        let mut vec = Vec::new();
        for path in paths {
            let name = path.unwrap().file_name().to_str().unwrap().to_string();
            vec.push(name);
        }

        ObjectType::Directory(vec)
    } else {
        let contents = match fs::read_to_string(&location) {
            Ok(c) => {c}
            Err(_) => { "".to_string() }
        };
        ObjectType::File(contents)
    }
}


fn save_file(path: String, contents: String) -> bool {
    unimplemented!()
}

fn create_directory(path: String) -> bool {
    unimplemented!()
}

fn delete(path: String) -> bool {
    unimplemented!()
}
