use rustc_version::version;

pub struct Version {
    branch: String,
    commit_hash: String,
    date: String,
    version: String,
}

impl Version {

    pub fn new() -> Version {
        Version{
            branch: "".to_string(),
            commit_hash: "".to_string(),
            date: "".to_string(),
            version: "".to_string(),
        }
    }

    pub fn show(&self) {
        println!("Version: {}", self.version);
        println!("Build Time: {}", self.date);
        println!("Build Branch: {}", self.branch);
        println!("Git Commit: {}", self.commit_hash);
        println!("Rust Version: {}",  version().unwrap());
        println!("Os/Arch: {}/{}", std::env::consts::OS, std::env::consts::ARCH);
    }
}