#![warn(clippy::nursery)]

use std::{fmt::format, io::Cursor};

use color_eyre::eyre::Result;
use mc_networking::{
    packets::{
        handshaking::serverbound::Handshake,
        status::{clientbound::{StatusResponse, PingResponse}, serverbound::{StatusRequest, PingRequest}},
    },
    traits::Packet,
    types::{Varint, varint_size},
    versions::Version,
    McEncodable,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

mod state;
use state::{ConnInfo, State};

async fn process_socket(mut socket: TcpStream) -> Result<()> {
    // do work with socket here

    let mut buf = Vec::new();

    let mut conn_info = ConnInfo::default();

    loop {
        let read = socket.read_buf(&mut buf).await?;
        if read == 0 {
            println!("No bytes read, closing connection");
            break;
        }

        let bytes_read = buf.as_slice();
        let mut cursor = Cursor::new(bytes_read);

        if let Ok(length) = Varint::decode(&mut cursor) {
            let length = length.value() as usize;
            if (length as usize) > bytes_read.len() {
                continue;
            }
            
            let bytes = buf.drain(..(length + varint_size(length as i32)? as usize));
            let mut cursor = Cursor::new(&bytes.as_slice()[bytes.len()-length..]);
            let id = Varint::decode(&mut cursor).unwrap().value();
            println!("id: {}", id);
            match conn_info.state {
                State::Handshaking => {
                    assert_eq!(id, 0x00);
                    if let Ok(packet) = Handshake::read_packet(&mut cursor) {
                        println!("{:?}", packet);
                        conn_info.protocol_version =
                            Version::from_id(packet.protocol_version.into());
                        match packet.next_state {
                        mc_networking::packets::handshaking::serverbound::handshake::State::Status => {
                            conn_info.state = State::Status;
                        }
                        mc_networking::packets::handshaking::serverbound::handshake::State::Login => {
                            conn_info.state = State::Login;                            
                        }
                    }
                    }
                }
                State::Status => {
                    match id {
                        0x00 =>
                            if let Ok(packet) = StatusRequest::read_packet(&mut cursor) {
                                println!("{:?}", packet);
                                let status_packet = StatusResponse {
                                    json_response: format!(
                                        "{{\"version\":{{\"name\":\"1.19.2\",\"protocol\":{}}},\"players\":{{\"max\":1,\"online\":0,\"sample\":[]}},\"description\":{{\"text\":\"Proxy\"}}}}", 
                                        conn_info.protocol_version.map_or(-1, |version| version.to_id().unwrap())),
                                };
                                println!("{}", &status_packet.json_response);
                                let mut buf = Vec::new();
                                status_packet.write_packet(&mut buf, Default::default())?;
                                socket.write_all(&buf).await?;
                            }
                        0x01 => if let Ok(packet) = PingRequest::read_packet(&mut cursor) {
                            println!("{:?}", packet);
                            let status_packet = PingResponse {
                                payload: packet.payload,
                            };
                            let mut buf = Vec::new();
                            status_packet.write_packet(&mut buf, Default::default())?;
                            socket.write_all(&buf).await?;
                        },
                        _ => unreachable!()
                    }
                },
                State::Login => todo!(),
                State::Play => todo!(),
            }
        }
    }
    println!("Connection closed");
    Ok(())
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
            println!("process_socket finished");
        });
    }

    // let handshake_packet = Handshake {
    //     protocol_version: Varint::from(754),
    //     server_host: "play.schoolrp.net".to_string(),
    //     server_port: 25565,
    //     next_state: mc_networking::packets::handshaking::serverbound::handshake::State::Status,
    // };

    // let status_packet = StatusRequest {};

    // let thread = tokio::spawn(async move {
    //     let mut stream = tokio::net::TcpStream::connect("play.schoolrp.net:25565")
    //         .await
    //         .unwrap();

    //     let mut buf = Vec::new();

    //     handshake_packet
    //         .write_packet(&mut buf, Default::default())
    //         .unwrap();
    //     status_packet
    //         .write_packet(&mut buf, Default::default())
    //         .unwrap();

    //     stream.write_all(&buf).await.unwrap();

    //     let mut bytes = Vec::new();
    //     loop {
    //         let read = stream.read_buf(&mut bytes).await.unwrap();
    //         if read == 0 {
    //             break;
    //         }
    //         let bytes_read = bytes.as_slice();
    //         let mut cursor = Cursor::new(bytes_read);
    //         if let Ok(length) = Varint::decode(&mut cursor) {
    //             let length = length.value() as usize;
    //             if (length as usize) > bytes_read.len() {
    //                 continue;
    //             }
    //             let id = Varint::decode(&mut cursor).unwrap().value();
    //             assert_eq!(id, 0x00);
    //             if let Ok(packet) = StatusResponse::read_packet(&mut cursor) {
    //                 println!("{}", packet.json_response);
    //                 break;
    //             }
    //         }
    //     }
    // });

    // thread.await.unwrap();
}
