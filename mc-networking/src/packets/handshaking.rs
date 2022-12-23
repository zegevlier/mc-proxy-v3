use crate::packets;

use self::serverbound::Handshake;

pub mod clientbound;
pub mod serverbound;

packets! {
    HandshakingPacket {
        ClientboundHandshakingPacket {
        }
        ServerboundHandshakingPacket {
            Handshake {
                -1..=761 => 0x00,
            },
            @0x00 => {
                -1..=761 => Handshake, // The -1 is so it is also allowed when the version is not yet known. Only allowed for this packet.
            }
        }
    }
}
