use std::collections::HashMap;
use std::path::Path;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Deserializer, Serialize};
use config::Config as cconfig;
use config::File as cfile;
use config::FileFormat as cfileformat;
use notify::{event, RecommendedWatcher, RecursiveMode, Watcher};
use std::thread;
use std::thread::Thread;
use std::time::Duration;
use log::debug;
use notify::Config as nconfig;
use tokio::sync::watch::channel;
use tokio::task;

static PATH: &str = "conf\\app_config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub title: String,
    pub app: App,
    pub net: Net,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    pub setting_path: String,
    pub log_level: String,
    pub allow_lan: bool,
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
