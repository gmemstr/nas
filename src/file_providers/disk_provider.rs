use std::fs;
use std::collections::HashMap;
use crate::file_providers::{Provider, Providers, ObjectType};
use serde_json::value::Value::Object;
use std::fs::{Metadata, File, OpenOptions};
use std::io::{Error, Bytes, Write};

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
        let mut vec: Vec<HashMap<String, String>> = Vec::new();
        for path in paths {
            let file = path.unwrap();
            let name = file.file_name().to_str().unwrap().to_string();
            let meta = match file.metadata() {
                Ok(m) => m,
                Err(_) => { return ObjectType::Missing }
            };
            let mut object: HashMap<String, String> = HashMap::new();
            object.insert("name".to_string(), name);
            object.insert("is_dir".to_string(), meta.is_dir().to_string());

            vec.push(object);
        }

        ObjectType::Directory(vec)
    } else {
        let contents = match fs::read(&location) {
            Ok(c) => {c}
            Err(_) => { Vec::new() }
        };
        ObjectType::File(contents)
    }
}

pub fn save_object(path: &String, contents: &[u8]) -> bool {
    let mut f: File = match fs::metadata(path) {
        Ok(_) => match OpenOptions::new().write(true).open(path) {
            Ok(f) => f,
            Err(_) => return false
        },
        Err(_) => match File::create(path) {
            Ok(f) => f,
            Err(_) => return false
        }
    };

    let r = f.write_all(contents);
    match r {
        Ok(_) => true,
        Err(_) => false
    }
}

fn create_directory(path: String) -> bool {
    unimplemented!()
}

fn delete(path: String) -> bool {
    unimplemented!()
}
