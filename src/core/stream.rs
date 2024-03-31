
pub struct Sniff {

}

pub struct DownStream {
    tag: String,
    addr: String,
    port: i32,
    interfaces: Vec<String>,
    sniff: Sniff,
}


pub struct UpStream {
    tag: String,
    addr: String,
    port: i32,
    transport: String,
}