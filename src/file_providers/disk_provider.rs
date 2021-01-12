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
