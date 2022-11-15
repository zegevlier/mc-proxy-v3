use anyhow::Result;
use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::packet::{Packet, PacketReaderWriter};

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
        let mut stream = tokio::net::TcpStream::connect("127.0.0.1:25565")
            .await
            .unwrap();
        let handshake_bytes = handshake_packet.get_packet_bytes(-1).unwrap();
        let status_bytes = status_packet.get_packet_bytes(-1).unwrap();

        stream.write_all(&handshake_bytes[..]).await.unwrap();
        stream.write_all(&status_bytes[..]).await.unwrap();

        let mut returned_bytes = vec![0; 1024];

        let returned_amount = stream.read(&mut returned_bytes).await.unwrap();

        let mut bytes = BytesMut::with_capacity(returned_amount);
        bytes.put(&returned_bytes[..returned_amount]);
        let mut returned_packet = Packet::read_from_bytes(&mut bytes).unwrap();

        println!("{}", returned_packet.read_string().unwrap());
    });

    thread.await.unwrap();

    Ok(())
}
