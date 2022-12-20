use std::io::Read;

use color_eyre::{eyre::bail, Result};
use mc_networking_macros::PacketEncoder;

use crate::{traits::Packet, types::Direction, McEncodable};

use self::{
    clientbound::{PingResponse, StatusResponse},
    serverbound::{PingRequest, StatusRequest},
};

pub mod clientbound;
pub mod serverbound;

#[derive(Debug, PacketEncoder)]
pub enum StatusPacket {
    Clientbound(ClientboundStatusPacket),
    Serverbound(ServerboundStatusPacket),
}
#[derive(Debug, PacketEncoder)]
pub enum ClientboundStatusPacket {
    StatusResponse(StatusResponse),
    PingResponse(PingResponse),
}

#[derive(Debug, PacketEncoder)]
pub enum ServerboundStatusPacket {
    StatusRequest(StatusRequest),
    PingRequest(PingRequest),
}

pub fn decode_status_packet(
    direction: Direction,
    packet_id: i32,
    buf: &mut impl Read,
) -> Result<StatusPacket> {
    Ok(match direction {
        Direction::Clientbound => {
            let packet = match packet_id {
                0 => ClientboundStatusPacket::StatusResponse(StatusResponse::decode(buf)?),
                1 => ClientboundStatusPacket::PingResponse(PingResponse::decode(buf)?),
                _ => bail!("Unknown packet id: {}", packet_id),
            };
            #[allow(unreachable_code)]
            StatusPacket::Clientbound(packet)
        }
        Direction::Serverbound => {
            let packet = match packet_id {
                0 => ServerboundStatusPacket::StatusRequest(StatusRequest::decode(buf)?),
                1 => ServerboundStatusPacket::PingRequest(PingRequest::decode(buf)?),
                _ => bail!("Unknown packet id: {}", packet_id),
            };
            StatusPacket::Serverbound(packet)
        }
    })
}
