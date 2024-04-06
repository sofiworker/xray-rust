use std::error::Error;
use log::{log, warn};
use crate::core::feature::Feature;

// 采用接口的形式，定义一组接口，让所有的 stream 实现该接口
pub trait Stream {
    async fn start(&self) -> Result<(), Box<dyn Error>> {
        warn!("not impl");
        Ok(())
    }
    fn close(&self) -> Result<(), Box<dyn Error>> {
        warn!("not impl");
        Ok(())
    }
    // to get the stream feature
    // fn feature(&self) -> impl Feature;

    fn handle(bs: Vec<u8>) -> Result<(), Box<dyn Error>> {
        warn!("not impl");
        Ok(())
    }

    async fn dail() -> Result<(), Box<dyn Error>> {
        return Ok(())
    }
}