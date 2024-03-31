use serde::{Deserialize, Serialize, Deserializer};
use crate::conf::AppConfig;
use config::File as cfile;
use config::FileFormat as cfileformat;
use notify::Watcher;

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingConfig {
    // pub down_stream_config: DownStreamConfig,
    // pub up_stream_config: UpStreamConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownStreamConfig {}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpStreamConfig {}

impl SettingConfig {
    pub fn new() -> SettingConfig {
        Self::read_config()
    }

    fn read_config() -> SettingConfig {
        let settings = config::Config::builder()
            .add_source(cfile::new(AppConfig::get_data().app.setting_path.as_str(), cfileformat::Json)).build().unwrap();
        let result: Result<SettingConfig, config::ConfigError> = settings.try_deserialize();
        result.unwrap()
    }

    pub fn get_data() -> SettingConfig {
        Self::read_config()
    }
}
