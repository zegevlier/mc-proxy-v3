use std::io::{Read, Write};

use color_eyre::{eyre::bail, Result};
use mc_networking_macros::PacketEncoder;

use crate::{
    traits::Packet,
    types::{Compression, Direction},
    McEncodable,
};

use self::serverbound::Handshake;

pub mod clientbound;
pub mod serverbound;

#[derive(Debug, PacketEncoder)]
pub enum HandshakingPacket {
    Clientbound(ClientboundHandshakingPacket),
    Serverbound(ServerboundHandshakingPacket),
}

#[derive(Debug)]
pub enum ClientboundHandshakingPacket {}

impl ClientboundHandshakingPacket {
    fn write_packet(&self, _buf: &mut impl Write, _compression: Compression) -> Result<()> {
        bail!("There are no clientbound packets in the handshaking state");
    }
}

#[derive(Debug, PacketEncoder)]
pub enum ServerboundHandshakingPacket {
    Handshake(Handshake),
}

pub fn decode_handshaking_packet(
    direction: Direction,
    packet_id: i32,
    buf: &mut impl Read,
) -> Result<HandshakingPacket> {
    Ok(match direction {
        Direction::Clientbound => {
            bail!("There are no clientbound packets in the handshaking state")
        }
        Direction::Serverbound => {
            let packet = match packet_id {
                0 => ServerboundHandshakingPacket::Handshake(Handshake::decode(buf)?),
                _ => bail!("Unknown packet id: {}", packet_id),
            };
            HandshakingPacket::Serverbound(packet)
        }
    })
}

pub fn encode_handshaking_packet(
    packet: HandshakingPacket,
    buf: &mut impl std::io::Write,
) -> Result<()> {
    match packet {
        HandshakingPacket::Serverbound(packet) => match packet {
            ServerboundHandshakingPacket::Handshake(packet) => packet.encode(buf),
        },
        _ => bail!("There are no clientbound packets in the handshaking state"),
    }
}
