use crate::{
    derive::{McEncodable, VarintEnum},
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

impl From<State> for crate::types::State {
    fn from(val: State) -> Self {
        match val {
            State::Status => crate::types::State::Status,
            State::Login => crate::types::State::Login,
        }
    }
}
