use std::{fs::File, io::BufReader};

use lazy_static::lazy_static;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    id: String,
    flow: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RealitySettings {
    show: bool,
    dest: String,
    xver: u32,
    serverNames: Vec<String>,
    privateKey: String,
    minClientVer: String,
    maxClientVer: String,
    maxTimeDiff: u32,
    shortIds: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamSettings {
    network: String,
    security: String,
    realitySettings: RealitySettings,
}

#[derive(Serialize, Deserialize, Debug)]
struct Inbound {
    listen: String,
    port: u32,
    protocol: String,
    settings: Settings,
    streamSettings: StreamSettings,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    clients: Vec<Client>,
    decryption: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingConfig {
    inbounds: Vec<Inbound>,
}

impl Default for SettingConfig {
    fn default() -> Self {
        let file_path = "conf/config.json";

        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("error is op en conf {e}"),
        };

        let reader = BufReader::new(file);

        let c: SettingConfig = serde_json::from_reader(reader).expect("parse config file failed");

        debug!("{:?}", c);
        return c;
    }
}

impl SettingConfig {
    pub fn get<'a>() -> &'a Self {
        lazy_static! {
            static ref CACHE: SettingConfig = SettingConfig::default();
        }
        &CACHE
    }
}
