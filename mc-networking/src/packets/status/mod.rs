use crate::packets;

use self::{
    clientbound::{PingResponse, StatusResponse},
    serverbound::{PingRequest, StatusRequest},
};

pub mod clientbound;
pub mod serverbound;

packets! {
    StatusPacket {
        ClientboundStatusPacket {
            0x00 -> StatusResponse(StatusResponse),
            0x01 -> PingResponse(PingResponse)
        }
        ServerboundStatusPacket {
            0x00 -> StatusRequest(StatusRequest),
            0x01 -> PingRequest(PingRequest)
        }
    }
}
