#![warn(clippy::nursery)]

use std::io::Cursor;

use color_eyre::eyre::Result;
use mc_networking::{
    packets::{
        handshaking::serverbound::Handshake,
        status::{clientbound::StatusResponse, serverbound::StatusRequest},
    },
    traits::Packet,
    types::Varint,
    McEncodable,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    let handshake_packet = Handshake {
        protocol_version: Varint::from(754),
        server_host: "play.schoolrp.net".to_string(),
        server_port: 25565,
        next_state: mc_networking::packets::handshaking::serverbound::handshake::State::Status,
    };

    let status_packet = StatusRequest {};

    let thread = tokio::spawn(async move {
        let mut stream = tokio::net::TcpStream::connect("play.schoolrp.net:25565")
            .await
            .unwrap();

        let mut buf = Vec::new();

        handshake_packet
            .write_packet(&mut buf, Default::default())
            .unwrap();
        status_packet
            .write_packet(&mut buf, Default::default())
            .unwrap();

        stream.write_all(&buf).await.unwrap();

        let mut bytes = Vec::new();
        loop {
            let read = stream.read_buf(&mut bytes).await.unwrap();
            if read == 0 {
                break;
            }
            let bytes_read = bytes.as_slice();
            let mut cursor = Cursor::new(bytes_read);
            if let Ok(length) = Varint::decode(&mut cursor) {
                let length = length.value() as usize;
                if (length as usize) > bytes_read.len() {
                    continue;
                }
                let id = Varint::decode(&mut cursor).unwrap().value();
                assert_eq!(id, 0x00);
                if let Ok(packet) = StatusResponse::read_packet(&mut cursor) {
                    println!("{}", packet.json_response);
                    break;
                }
            }
        }
    });

    thread.await.unwrap();

    Ok(())
}
