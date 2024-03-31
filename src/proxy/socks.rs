
pub enum SocksVersion {
    Version4,
    Version5,
}

pub struct Socks {
    pub version: SocksVersion,
}