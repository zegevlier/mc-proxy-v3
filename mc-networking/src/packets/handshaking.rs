use crate::packets;

use self::serverbound::Handshake;

pub mod clientbound;
pub mod serverbound;

packets! {
    HandshakingPacket {
        ClientboundHandshakingPacket {
        }
        ServerboundHandshakingPacket {
            Handshake,
            @0x00 => {
                -1..=761 => Handshake,
            }
        }
    }
}
