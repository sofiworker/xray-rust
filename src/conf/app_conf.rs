use config::File as cfile;
use config::FileFormat as cfileformat;
use notify::Watcher;
use serde::{Deserialize, Deserializer, Serialize};

static PATH: &str = "conf\\app_config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AppMode {
    Client,
    Server,
    Relay,
    Gateway,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub title: String,
    pub app: App,
    pub net: Net,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    pub mode: AppMode,
    pub setting_path: String,
    pub log_level: String,
    pub allow_lan: bool,
    pub plugin_dir: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Net {
    pub transport: String,
    pub port: i32,
    pub addr: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        Self::read_config()
    }

    fn read_config() -> AppConfig {
        let settings = config::Config::builder()
            .add_source(cfile::new(PATH, cfileformat::Toml)).build().unwrap();
        let result: Result<AppConfig, config::ConfigError> = settings.try_deserialize();
        result.unwrap()
    }

    pub fn get_data() -> AppConfig {
        Self::read_config()
    }
}
