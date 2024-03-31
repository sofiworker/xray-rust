use std::{fs::File, io::BufReader};

use lazy_static::lazy_static;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingConfig {
}

impl SettingConfig {
    pub fn new() -> SettingConfig {
        return  SettingConfig{};
    }
}
