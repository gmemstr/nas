// For now, this mirrors disk_provider.rs for testing. Proper S3 implementation to be done soon.
use std::fs;
use std::collections::HashMap;
use crate::file_providers::{FileProvider, Provider};

pub struct S3Provider(pub Provider);

impl FileProvider for S3Provider {
    fn setup(&self) -> bool {
        match fs::create_dir(&self.0.location) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn get_name(&self) -> &str {
        &self.0.name
    }

    fn get_directory(&self, path: String) -> Vec<String> {
        let paths = fs::read_dir(format!("{}/{}", self.0.location, path)).unwrap();
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
