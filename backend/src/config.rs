use serde::{Deserialize, Serialize};

use std::io::Write;
use std::{fs, path};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    // Address with port, e.g. "localhost:8088"
    pub address: String,
    pub auth_salt: String,
    pub jwt_expiration: i64,
    pub jwt_key: String,
}

impl Config {
    // Create default config file
    pub fn new() -> Self {
        Config {
            address: "localhost:8088".to_string(),
            jwt_expiration: 48,
            jwt_key: "secret_key_change_me".to_string(),
            auth_salt: "hashing_salt_change_me".to_string(),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = get_initial_config();
}

fn get_initial_config() -> Config {
    let config_path = "config.json";
    if path::Path::new(config_path).exists() {
        info!("Found config in {}, loading it from file", config_path);
        let lines = fs::read_to_string(config_path).expect("Failed to read config.json");
        serde_json::from_str(&lines).unwrap()
    } else {
        info!("Config file not found, creating default in {}", config_path);
        let config = Config::new();
        let string = serde_json::to_string(&config).unwrap();
        let mut file = fs::File::create(config_path).unwrap();
        file.write_all(&string.into_bytes()).unwrap();
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_config() {
        let config = get_initial_config();
        assert_ne!(config.address, "".to_string());
    }

    #[test]
    fn get_config_from_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.address, "".to_string());
    }
}
