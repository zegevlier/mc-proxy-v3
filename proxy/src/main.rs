#![warn(clippy::nursery)]
// #![warn(clippy::unwrap_used, clippy::expect_used)]

use anyhow::Result;
use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::packet::{Packet, ReaderWriter};

mod packet;
pub(crate) mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let mut handshake_packet = Packet::new();
    handshake_packet.set_packet_id(0x00);
    handshake_packet.write_varint(760);
    handshake_packet.write_string("127.0.0.1".to_string());
    handshake_packet.write_unsigned_short(25565);
    handshake_packet.write_varint(1);

    let mut status_packet = Packet::new();
    status_packet.set_packet_id(0x00);

    let thread = tokio::spawn(async move {
        let mut stream = tokio::net::TcpStream::connect("play.schoolrp.net:25565")
            .await
            .unwrap();
        let handshake_bytes = handshake_packet.get_packet_bytes(-1).unwrap();
        let status_bytes = status_packet.get_packet_bytes(-1).unwrap();

        let mut buf = BytesMut::new();
        buf.put(handshake_bytes);
        buf.put(status_bytes);

        stream.write_all(&buf).await.unwrap();

        let mut bytes = BytesMut::new();
        loop {
            let read = stream.read_buf(&mut bytes).await.unwrap();
            if read == 0 {
                break;
            }
            let mut bytes_read = bytes.clone();
            if let Ok(mut packet) = Packet::read_from_bytes(&mut bytes_read) {
                println!("{}", packet.read_string().unwrap());
                break;
            }
        }

        // println!("Returned amount: {}", bytes.len());

        // let mut returned_packet = Packet::read_from_bytes(&mut bytes).unwrap();

        // println!("{}", returned_packet.read_string().unwrap());
    });

    thread.await.unwrap();

    Ok(())
}
