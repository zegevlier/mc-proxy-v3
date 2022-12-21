use mc_networking_macros::McEncodable;

use crate::packet;

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct StatusResponse {
    pub json_response: String,
}

packet!(StatusResponse, 0x00);

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct PingResponse {
    pub payload: i64,
}

packet!(PingResponse, 0x01);
