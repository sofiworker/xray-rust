use std::{net, string};
use std::net::SocketAddr;

use crate::conf;
use log::{debug, error};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::conf::AppConfig;
use crate::core::stream::{};

// 服务端应该

pub trait Server {

}

// #[derive(Debug)]
// pub struct Server {
//     app_config: AppConfig,
//     down_stream: Vec<DownStream>,
//     up_stream: Vec<UpStream>,
// }

// impl Server {
    // pub fn new(c: AppConfig) -> Self {
    //     Server {
    //         app_config: c,
    //         down_stream: Vec::new(),
    //         up_stream: Vec::new(),
    //     }
    // }

    // pub async fn start(&self) -> io::Result<()> {
    //     let addr = format!("{}:{}", self.app_config.net.addr, self.app_config.net.port);
    //     debug!("use [{}] as transport, bind [{}]", self.app_config.net.transport, addr);
    //     let listener = TcpListener::bind(addr).await?;
    //     loop {
    //         let (stream, socket_addr) = listener.accept().await?;
    //         tokio::spawn(async move {
    //             Self::process(stream, socket_addr).await
    //         });
    //     }
    // }
    //
    // async fn process(stream: TcpStream, addr: SocketAddr) {}
// }
