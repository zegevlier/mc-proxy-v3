use crate::{traits::McEncodable, types::Varint};

#[derive(Debug, PartialEq, Eq)]
pub struct Handshake {
    pub protocol_version: Varint,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: Varint,
}

impl McEncodable for Handshake {
    fn read(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        Ok(Self {
            protocol_version: Varint::read(buf)?,
            server_host: String::read(buf)?,
            server_port: u16::read(buf)?,
            next_state: Varint::read(buf)?,
        })
    }

    fn write(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        self.protocol_version.write(buf)?;
        self.server_host.write(buf)?;
        self.server_port.write(buf)?;
        self.next_state.write(buf)?;
        Ok(())
    }
}
