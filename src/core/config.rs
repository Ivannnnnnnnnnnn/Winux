use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub theme: String,
    pub keybinds: Vec<(String, String)>,
}

pub fn load_config() -> Config {
    let path = config_path();
    if let Ok(contents) = fs::read_to_string(&path) {
        if let Ok(parsed) = serde_json::from_str(&contents) {
            return parsed;
        }
    }

    Config {
        theme: "default".to_string(),
        keybinds: vec![
            ("Super+D".to_string(), "launcher".to_string())
        ],
    }
}

fn config_path() -> PathBuf {
    let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".winux").join("config.json")
}
