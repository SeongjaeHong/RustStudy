use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

pub const MSG_SIZE: usize = 256;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub addr: SocketAddr,
    pub msg: String,
}
