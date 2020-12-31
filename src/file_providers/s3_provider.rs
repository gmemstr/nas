use std::fs;
use std::collections::HashMap;
use crate::file_providers::{FileProvider, Provider};

pub struct S3Provider {
    name: String,
    location: String,
    properties: Option<HashMap<String, String>>
}

impl FileProvider for S3Provider {
    fn from_provider(provider: Provider) -> S3Provider {
        S3Provider {
            name: provider.name,
            location: provider.location,
            properties: provider.properties,
        }
    }

    fn setup(&self) -> bool {
        match fs::create_dir(&self.location) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn get_directory(&self, path: String) -> Vec<String> {
        let paths = fs::read_dir(format!("{}/{}", self.location, path)).unwrap();
        let mut vec = Vec::new();
        for path in paths {
            let name = path.unwrap().file_name().to_str().unwrap().to_string();
            vec.push(name);
        }
        return vec;
    }

    fn save_file(&self, path: String, contents: String) -> bool {
        unimplemented!()
    }

    fn create_directory(&self, path: String) -> bool {
        unimplemented!()
    }

    fn delete(&self, path: String) -> bool {
        unimplemented!()
    }
}
