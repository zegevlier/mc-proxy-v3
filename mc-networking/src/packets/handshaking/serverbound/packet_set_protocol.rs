use crate::types::Varint;
use mc_networking_macros::packet;

packet! {
    struct PacketSetProtocol {
        protocol_version: Varint,
        server_host: String,
        server_port: u16,
        next_state: Varint,
    }
}

pub struct PacketSetProtocol {
    pub protocol_version: Varint,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: Array<Varint, struct sdfjk,
}
