use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host : String,
    pub port : u16,
    pub key  : [u8;16]
}
