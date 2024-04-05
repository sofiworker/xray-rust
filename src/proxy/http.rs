use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use bytes::{Bytes, BytesMut};
use hyper::{Request, Response};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto;
use log::error;
use tokio::net::{TcpListener, TcpStream};
use crate::core::stream::Stream;
use crate::core::feature::Feature;
use std::convert::Infallible;
use std::str::FromStr;
use http_body_util::Full;

pub enum HttpVersion {
    V1,
    V2,
    V3,
}

pub struct HttpConfig {
    pub addr: String,
    pub port: u16,
    pub interface: String,
    pub version: HttpVersion,
}

pub struct HttpStream {
    config: HttpConfig,
}

impl HttpStream {
    pub fn new(config: HttpConfig) -> HttpStream {
        HttpStream {
            config
        }
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

impl Stream for HttpStream {
    async fn start(&self) -> Result<(), Box<dyn Error>> {
        let v4 = Ipv4Addr::from_str(self.config.addr.as_str()).unwrap();
        // This address is localhost
        let addr: SocketAddr = SocketAddr::new(IpAddr::V4(v4), self.config.port);
        let listener = TcpListener::bind(addr).await.unwrap();
        loop {
            // When an incoming TCP connection is received grab a TCP stream for
            // client<->server communication.
            //
            // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
            // .await point allows the Tokio runtime to pull the task off of the thread until the task
            // has work to do. In this case, a connection arrives on the port we are listening on and
            // the task is woken up, at which point the task is then put back on a thread, and is
            // driven forward by the runtime, eventually yielding a TCP stream.
            let (tcp, _) = listener.accept().await?;
            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(tcp);

            // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
            // current task without waiting for the processing of the HTTP1 connection we just received
            // to finish
            tokio::task::spawn(async move {
                // Handle the connection from the client using HTTP1 and pass any
                // HTTP requests received on that connection to the `hello` function
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(hello))
                    .await
                {
                    error!("Error serving connection: {:?}", err);
                }
            });
        }
        Ok(())
    }

    fn close(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn feature(&self) -> Box<dyn Feature> {
        Box::new(Self {
            config: HttpConfig {
                addr: "".to_string(),
                port: 0,
                interface: "".to_string(),
                version: HttpVersion::V1,
            }
        })
    }

    fn handle(bs: Vec<u8>) -> Result<(), Box<dyn Error>> {
        BytesMut::with_capacity(10);
        Ok(())
    }
}

impl Feature for HttpStream {}

#[cfg(test)]
mod test {
    use crate::core::stream::Stream;
    use crate::proxy::http::{HttpConfig, HttpStream, HttpVersion};

    #[tokio::test]
    async fn test_http_server() {
        let server = HttpStream::new(HttpConfig {
            addr: "127.0.0.1".to_string(),
            port: 80,
            interface: "".to_string(),
            version: HttpVersion::V1,
        });
        server.start().await.unwrap();
    }
}