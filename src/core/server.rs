use std::string;

use crate::conf;
use log::error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct Server {
    version: String,
    port: i32,
    protocol: String,
    netcard: String,
    addr: String,
}

impl Server {
    pub fn new(c: conf::AppConfig) -> Self {
        Server {
            version: "v1.0.0".to_string(),
            port: 3389,
            protocol: "http".to_string(),
            netcard: "lo".to_string(),
            addr: "127.0.0.1".to_string(),
        }
    }

    #[tokio::main]
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.addr, self.protocol);
        let listener = TcpListener::bind(addr).await?;
        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            error!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        error!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    }
}
