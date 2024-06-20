pub mod rest;
pub mod util;
pub mod ws;

#[derive(Clone)]
pub struct Credentials {
    pub api_key: String,
    pub secret: String,
}
