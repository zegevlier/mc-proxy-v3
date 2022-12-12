pub mod packets;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{packets::handshaking::serverbound::PacketSetProtocol, types::Varint};

    #[test]
    fn it_works() {
        let handshaking_packet: PacketSetProtocol = PacketSetProtocol {
            protocol_version: Varint::from_value(3),
            server_host: "Hello world".to_string(),
            server_port: 25565,
            next_state: Varint::from_value(2),
        };

        println!("{}", handshaking_packet.server_host);
    }
}
