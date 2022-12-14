use crate::{derive::McEncodable, packet};

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct PingResponse {
    pub payload: i64,
}

packet!(PingResponse, 0x01);
