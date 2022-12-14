use crate::{
    derive::{McEncodable, VarintEnum},
    packet,
    types::Varint,
};

#[derive(Debug, PartialEq, Eq, McEncodable)]
pub struct Handshake {
    pub protocol_version: Varint,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: State,
}

#[derive(Debug, PartialEq, Eq, VarintEnum)]
pub enum State {
    Status = 1,
    Login,
}

packet!(Handshake, 0x00);
