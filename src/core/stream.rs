
#[derive(Debug)]
pub struct Sniff {

}

#[derive(Debug)]
pub struct DownStream {
    tag: String,
    addr: String,
    port: i32,
    interfaces: Vec<String>,
    sniff: Sniff,
}

#[derive(Debug)]
pub struct UpStream {
    tag: String,
    addr: String,
    port: i32,
    transport: String,
}