use crate::packets;

use self::serverbound::Handshake;

pub mod clientbound;
pub mod serverbound;

packets! {
    HandshakingPacket {
        ClientboundHandshakingPacket {
        }
        ServerboundHandshakingPacket {
            0x00 -> Handshake(Handshake)
        }
    }
}
