
pub enum SocksVersion {
    Version4,
    Version5,
}

pub struct SocksStream {
    pub version: SocksVersion,
}


#[cfg(test)]
mod test {

}