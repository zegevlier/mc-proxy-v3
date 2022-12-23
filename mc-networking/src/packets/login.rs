use crate::packets;

use self::serverbound::LoginStart;

pub mod clientbound;
pub mod serverbound;

packets! {
    LoginPacket {
        ClientboundLoginPacket {
        }
        ServerboundLoginPacket {
            // Login start might have more versions, but this is the only one I've checked.
            LoginStart {
                761..=761 => 0x00,
            },
            @0x00 => {
                761..=761 => LoginStart,
            }
        }
    }
}
