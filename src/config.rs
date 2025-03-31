use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub rofi_binary: String,
    pub prompt: String,
}

impl Config {
    pub fn load() -> Self {
        let config_path = "config.json";
        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
        eprintln!("Using default config.");
        Self {
            rofi_binary: "rofi".to_string(),
            prompt: "Select Kaomoji".to_string(),
        }
    }
}
