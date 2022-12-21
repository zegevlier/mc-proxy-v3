use std::io::{Read, Write};

use color_eyre::Result;

use crate::types::{Compression, Direction, State};

pub mod handshaking;
pub mod status;

#[derive(Debug)]
pub enum Packets {
    Handshaking(handshaking::HandshakingPacket),
    Status(status::StatusPacket),
}

impl Packets {
    pub fn write_packet(&self, buf: &mut impl Write, compression: Compression) -> Result<()> {
        match self {
            Packets::Handshaking(packet) => packet.write_packet(buf, compression)?,
            Packets::Status(packet) => packet.write_packet(buf, compression)?,
        }
        Ok(())
    }
}

pub fn decode_packet(
    state: State,
    direction: Direction,
    packet_id: i32,
    buf: &mut impl Read,
) -> Result<Packets> {
    Ok(match state {
        State::Handshaking => {
            let packet = handshaking::decode_handshaking_packet(direction, packet_id, buf)?;
            Packets::Handshaking(packet)
        }
        State::Status => {
            let packet = status::decode_status_packet(direction, packet_id, buf)?;
            Packets::Status(packet)
        }
        _ => unimplemented!(),
    })
}
