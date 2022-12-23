use crate::{packet, packets};

use self::{
    clientbound::{PingResponse, StatusResponse},
    serverbound::{PingRequest, StatusRequest},
};

pub mod clientbound;
pub mod serverbound;

packet!(StatusResponse, {
    0..=761 => 0x00,
});
packet!(PingResponse, {
    0..=761 => 0x01,
});

packet!(StatusRequest, {
    0..=761 => 0x00,
});
packet!(PingRequest, {
    0..=761 => 0x01,
});

packets! {
    StatusPacket {
        ClientboundStatusPacket {
            StatusResponse,
            PingResponse,
            @0x00 => {
                0..=761 => StatusResponse,
            }
            @0x01 => {
                0..=761 => PingResponse,
            }
        }
        ServerboundStatusPacket {
            StatusRequest,
            PingRequest,
            @0x00 => {
                0..=761 => StatusRequest,
            }
            @0x01 => {
                0..=761 => PingRequest,
            }
        }
    }
}
