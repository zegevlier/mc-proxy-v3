#![warn(clippy::nursery)]

use std::{io::Cursor, sync::Arc};

use color_eyre::eyre::Result;
use mc_networking::{
    packets::{
        decode_packet,
        handshaking::{HandshakingPacket, ServerboundHandshakingPacket},
        login::{LoginPacket, ServerboundLoginPacket},
        status::{
            clientbound::{PingResponse, StatusResponse},
            ServerboundStatusPacket, StatusPacket,
        },
        Packets,
    },
    types::{varint_size, Direction, Varint},
    versions::Version,
    McEncodable,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};

use crate::state::ConnInfo;

mod state;

async fn process_socket(socket: TcpStream) -> Result<()> {
    // do work with socket here

    let (c_tx, mut c_rx): (Sender<Packets>, Receiver<Packets>) = mpsc::channel(1024);

    let (mut rx, mut tx) = socket.into_split();

    let conn_info = Arc::from(Mutex::from(ConnInfo::default()));

    let mut handles = Vec::new();

    let conn_info_clone = conn_info.clone();
    handles.push(tokio::spawn(async move {
        let conn_info = conn_info_clone;
        let mut buf = Vec::new();
        loop {
            let read = rx.read_buf(&mut buf).await.unwrap();
            if read == 0 {
                println!("No bytes read, closing connection");
                break;
            }

            loop {
                let bytes_read = buf.as_slice();
                let mut cursor = Cursor::new(&bytes_read);

                let length = Varint::decode(&mut cursor);
                if length.is_err() {
                    break;
                }

                let length = length.unwrap().value() as usize;

                if (length as usize) > bytes_read.len() {
                    println!("Length is greater than bytes read");
                    break;
                }

                let bytes = buf.drain(..(length + varint_size(length as i32).unwrap() as usize));
                let mut cursor = Cursor::new(&bytes.as_slice()[bytes.len() - length..]);
                let id = Varint::decode(&mut cursor).unwrap().value();

                let conn_info_l = conn_info.lock().await;
                let packet = match decode_packet(
                    conn_info_l.state,
                    Direction::Serverbound,
                    id,
                    conn_info_l.protocol_version,
                    &mut cursor,
                ) {
                    Ok(packet) => packet,
                    Err(e) => {
                        println!("Error decoding packet: {}", e);
                        continue;
                    }
                };
                dbg!(&packet);
                drop(conn_info_l);
                handle_packet(packet, &conn_info, &c_tx).await;
            }
        }
    }));

    handles.push(tokio::spawn(async move {
        loop {
            let packet = match c_rx.recv().await {
                Some(packet) => packet,
                None => {
                    println!("Finished sending packets to the client");
                    break;
                }
            };
            dbg!(&packet);
            let mut buf = Vec::new();
            let conn_info = conn_info.lock().await;
            packet
                .write_packet(&mut buf, conn_info.protocol_version, Default::default())
                .unwrap();
            drop(conn_info);
            tx.write_all(&buf).await.unwrap();
        }
    }));

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Connection closed");
    Ok(())
}

async fn handle_packet(packet: Packets, conn_info: &Arc<Mutex<ConnInfo>>, c_tx: &Sender<Packets>) {
    match packet {
        Packets::Handshaking(handshaking_packet) => match handshaking_packet {
            HandshakingPacket::Serverbound(serverbound_packet) => match serverbound_packet {
                ServerboundHandshakingPacket::Handshake(handshake) => {
                    let mut conn_info = conn_info.lock().await;
                    match Version::from_id(handshake.protocol_version.into()) {
                        Some(version) => conn_info.protocol_version = version,
                        None => {
                            println!("Unsupported protocol version, closing connection");
                            return;
                        }
                    }
                    conn_info.state = handshake.next_state.into();
                }
            },
            _ => panic!("Unexpected packet"),
        },
        Packets::Status(status_packet) => {
            match status_packet {
                StatusPacket::Serverbound(serverbound_status_packet) => {
                    match serverbound_status_packet {
                        ServerboundStatusPacket::StatusRequest(_packet) => {
                            let conn_info = conn_info.lock().await;
                            let status_response_packet = StatusResponse {
                            json_response: format!("{{\"version\":{{\"name\":\"1.19.2\",\"protocol\":{}}},\"players\":{{\"max\":1,\"online\":0,\"sample\":[]}},\"description\":{{\"text\":\"Proxy\"}}}}",
                                                    conn_info.protocol_version.to_id())
                        };
                            c_tx.send(Packets::Status(StatusPacket::Clientbound(
                            mc_networking::packets::status::ClientboundStatusPacket::StatusResponse(
                                status_response_packet,
                            ),
                        ))).await.unwrap();
                        }
                        ServerboundStatusPacket::PingRequest(packet) => {
                            c_tx.send(Packets::Status(StatusPacket::Clientbound(
                            mc_networking::packets::status::ClientboundStatusPacket::PingResponse(
                                PingResponse {
                                    payload: packet.payload,
                                }
                            ),
                        ))).await.unwrap();
                        }
                    }
                }
                _ => panic!("Unexpected packet"),
            }
        }
        Packets::Login(login_packet) => match login_packet {
            LoginPacket::Serverbound(serverbound_login_packet) => match serverbound_login_packet {
                ServerboundLoginPacket::LoginStart(packet) => {
                    println!("Username: {}", packet.username);
                    println!("Uuid: {:?}", packet.uuid);
                }
            },
            _ => panic!("Unexpected packet"),
        },
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let lisener = tokio::net::TcpListener::bind("0.0.0.0:25566").await?;

    loop {
        let (socket, _) = lisener.accept().await?;
        tokio::spawn(async {
            if let Err(e) = process_socket(socket).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}
