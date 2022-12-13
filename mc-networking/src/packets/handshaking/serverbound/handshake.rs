use mc_networking_macros::VarintEnum;

use crate::{
    traits::{McEncodable, Packet},
    types::Varint,
};

#[derive(Debug, PartialEq, Eq)]
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

impl McEncodable for Handshake {
    fn decode(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        Ok(Self {
            protocol_version: Varint::decode(buf)?,
            server_host: String::decode(buf)?,
            server_port: u16::decode(buf)?,
            next_state: State::decode(buf)?,
        })
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        self.protocol_version.encode(buf)?;
        self.server_host.encode(buf)?;
        self.server_port.encode(buf)?;
        self.next_state.encode(buf)?;
        Ok(())
    }
}

impl Packet for Handshake {
    fn id(&self) -> i32 {
        0x00
    }
}
