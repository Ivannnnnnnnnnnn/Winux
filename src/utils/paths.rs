use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("winux")
}

pub fn ensure_config_dir() {
    let dir = config_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir).ok();
    }
}

pub fn config_file() -> PathBuf {
    config_dir().join("config.json")
}
