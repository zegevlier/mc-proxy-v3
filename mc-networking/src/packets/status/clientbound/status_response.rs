use crate::{derive::McEncodable, packet};

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct StatusResponse {
    pub json_response: String,
}

packet!(StatusResponse, 0x00);
