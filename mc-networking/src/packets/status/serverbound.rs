use crate::{derive::McEncodable, packet};

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct StatusRequest {}

packet!(StatusRequest, 0x00);

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct PingRequest {
    pub payload: i64,
}

packet!(PingRequest, 0x01);
