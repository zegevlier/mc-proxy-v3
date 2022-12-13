pub mod packets;
pub mod traits;
pub mod types;

pub use traits::McEncodable;

#[cfg(test)]
mod tests {
    // use crate::{packets::handshaking::serverbound::Handshake, traits::McEncodable, types::Varint};

    // #[test]
    // fn it_works() {
    //     // let handshaking_packet: Handshake = Handshake {
    //     //     protocol_version: Varint::from(3),
    //     //     server_host: "play.example.com".to_string(),
    //     //     server_port: 25565,
    //     //     next_state: Varint::from(2),
    //     // };

    //     let mut buffer = Vec::new();
    //     handshaking_packet.encode(&mut buffer).unwrap();

    //     let mut cursor = std::io::Cursor::new(buffer.as_slice());
    //     let decoded_handshaking_packet = Handshake::decode(&mut cursor).unwrap();

    //     assert_eq!(handshaking_packet, decoded_handshaking_packet);
    // }
}
