#![warn(clippy::nursery)]
// #![warn(clippy::unwrap_used, clippy::expect_used)]

use std::io::Cursor;

use bytes::{BufMut, BytesMut};
use color_eyre::eyre::Result;
use mc_networking::{
    packets::{
        handshaking::serverbound::Handshake,
        status::{clientbound::StatusResponse, serverbound::StatusRequest},
    },
    traits::Packet,
    types::Varint,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod packet;

#[tokio::main]
async fn main() -> Result<()> {
    let handshake_packet = Handshake {
        protocol_version: Varint::from(754),
        server_host: "play.schoolrp.net".to_string(),
        server_port: 25565,
        next_state: Varint::from(1),
    };

    let status_packet = StatusRequest {};

    let thread = tokio::spawn(async move {
        let mut stream = tokio::net::TcpStream::connect("play.schoolrp.net:25565")
            .await
            .unwrap();

        let buf = BytesMut::new();
        let mut writer = buf.clone().writer();

        handshake_packet
            .write_packet(&mut writer, Default::default())
            .unwrap();
        status_packet
            .write_packet(&mut writer, Default::default())
            .unwrap();

        let buf = writer.into_inner();

        stream.write_all(&buf).await.unwrap();

        let mut bytes = BytesMut::new();
        loop {
            let read = stream.read_buf(&mut bytes).await.unwrap();
            if read == 0 {
                break;
            }
            let bytes_vec = bytes.to_vec();
            let bytes_read = bytes_vec.as_slice();
            if let Ok(packet) = StatusResponse::read_packet(&mut Cursor::new(bytes_read)) {
                println!("{}", packet.json_response);
                break;
            }
        }

        println!("Returned amount: {}", bytes.len());
    });

    thread.await.unwrap();

    Ok(())
}
