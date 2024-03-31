use std::string;

use crate::conf;
use log::{debug, error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::conf::AppConfig;
use crate::core::stream::{DownStream, UpStream};

#[derive(Debug)]
pub struct Server {
    app_config: AppConfig,
    down_stream: Vec<DownStream>,
    up_stream: Vec<UpStream>
}

impl Server {
    pub fn new(c: AppConfig) -> Self {
        Server {
            app_config: c,
            down_stream: Vec::new(),
            up_stream: Vec::new(),
        }
    }

    pub async fn start(&self) -> Result<(), ()> {
        let addr = format!("{}:{}", self.app_config.net.addr, self.app_config.net.port);
        debug!("use [{}] as transport, bind [{}]", self.app_config.net.transport, addr);;
        Ok(())
    }
}
