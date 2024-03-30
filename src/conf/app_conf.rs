use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AppConfig {
}


impl Default for AppConfig {
    fn default() -> Self {
        // let file_path = "../config/config.json";

        // let mut file = match File::open(file_path) {
        //     Ok(f) => f,
        //     Err(e) => panic!("error is op en conf {e}"),
        // };

        // let reader = BufReader::new(file);

        // let c: Config = serde_json::from_reader(reader).expect("parse config file failed");
        AppConfig {}
    }
}

impl AppConfig {
    pub fn get<'a>() -> &'a Self {
        lazy_static! {
            static ref CACHE: AppConfig = AppConfig::default();
        }
        &CACHE
    }
}
