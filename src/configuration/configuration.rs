use std::fs;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Configuration {
    pub general: Option<GeneralConfiguration>,
    pub providers: HashMap<String, ProviderConfiguration>,
    authentication: Option<HashMap<String, AuthConfiguration>>
}

#[derive(Deserialize)]
pub struct GeneralConfiguration {
    pub port: i32,
}

#[derive(Deserialize)]
pub struct AuthConfiguration {
    from: String,
    config: HashMap<String, String>
}

#[derive(Deserialize)]
pub struct ProviderConfiguration {
    pub provider: String,
    pub path: String,
    pub config: Option<HashMap<String, String>>,
}

pub fn load_config(path: Option<String>) -> Result<Configuration, String> {
    let config_file = path.unwrap_or(String::from("config.toml"));
    let config_raw = fs::read_to_string(config_file);
    let c = match config_raw {
        Ok(v) => {
            let config: Configuration = toml::from_str(&v).unwrap();
            config
        },
        Err(_) => {
            return Err(String::from("Unable to open configuration file."));
        }
    };

    Ok(c)
}
