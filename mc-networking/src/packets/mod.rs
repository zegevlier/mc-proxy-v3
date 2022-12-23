use std::io::{Read, Write};

use color_eyre::Result;

use crate::{
    types::{Compression, Direction, State},
    versions::Version,
};

pub mod handshaking;
pub mod login;
pub mod status;

#[derive(Debug)]
pub enum Packets {
    Handshaking(handshaking::HandshakingPacket),
    Status(status::StatusPacket),
    Login(login::LoginPacket),
}

impl Packets {
    pub fn write_packet(
        &self,
        buf: &mut impl Write,
        version: Version,
        compression: Compression,
    ) -> Result<()> {
        match self {
            Packets::Handshaking(packet) => packet.write_packet(buf, version, compression)?,
            Packets::Status(packet) => packet.write_packet(buf, version, compression)?,
            Packets::Login(packet) => packet.write_packet(buf, version, compression)?,
        }
        Ok(())
    }
}

pub fn decode_packet(
    state: State,
    direction: Direction,
    packet_id: i32,
    version: Version,
    buf: &mut impl Read,
) -> Result<Packets> {
    Ok(match state {
        State::Handshaking => {
            let packet =
                handshaking::HandshakingPacket::decode_packet(direction, packet_id, version, buf)?;
            Packets::Handshaking(packet)
        }
        State::Status => {
            let packet = status::StatusPacket::decode_packet(direction, packet_id, version, buf)?;
            Packets::Status(packet)
        }
        State::Login => {
            let packet = login::LoginPacket::decode_packet(direction, packet_id, version, buf)?;
            Packets::Login(packet)
        }
        _ => unimplemented!(),
    })
}
