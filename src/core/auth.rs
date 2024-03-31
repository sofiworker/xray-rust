
pub enum AuthKind {
    None,
    Password,
    PublicKey,
}

pub struct Auth {
    kind: AuthKind,
    uuid: String,
    name: String,
    password: String,
}