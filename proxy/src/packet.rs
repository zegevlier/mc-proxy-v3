// use std::io::{Cursor, Read};

// use bytes::{buf::Writer, Buf, BufMut, Bytes, BytesMut};
// use color_eyre::eyre::{bail, Result};
// use mc_networking::{types::Varint, McEncodable};

// use crate::utils::get_varint_size;

// #[derive(Debug)]
// pub struct Packet {
//     data: BytesMut,
//     packet_id: Option<i32>,
// }

// impl Packet {
//     pub fn new() -> Self {
//         Self {
//             data: BytesMut::new(),
//             packet_id: None,
//         }
//     }

//     pub fn with_capacity(capacity: usize) -> Self {
//         Self {
//             data: BytesMut::with_capacity(capacity),
//             packet_id: None,
//         }
//     }

//     pub fn write_bytes(&mut self, value: BytesMut) {
//         self.data.put(value);
//     }

//     pub fn set_packet_id(&mut self, packet_id: i32) {
//         self.packet_id = Some(packet_id);
//     }

//     fn get_size(&self) -> Result<i32> {
//         if let Some(packet_id) = self.packet_id {
//             Ok(self.data.len() as i32 + get_varint_size(packet_id)?)
//         } else {
//             Ok(self.data.len() as i32)
//         }
//     }

//     pub fn writer(&mut self) -> &mut Writer<BytesMut> {
//         &mut self.data.writer()
//     }

//     pub fn get_cursor(&mut self) -> &mut Cursor<&[u8]> {
//         &mut Cursor::new(self.data.slice(..))
//     }

//     pub fn get_packet_bytes(mut self, compressed: i32) -> Result<Bytes> {
//         if compressed > 0 {
//             todo!("Compression is not implemented yet");
//         }

//         if let Some(packet_id) = self.packet_id {
//             let mut packet = Self::new();
//             Varint::from(self.get_size()?).encode(packet.writer())?;
//             Varint::from(packet_id).encode(packet.writer())?;
//             packet.write_bytes(self.data.split());

//             Ok(packet.data.freeze())
//         } else {
//             bail!("Packet ID is not set");
//         }
//     }

//     pub fn read_from_bytes(bytes: &mut BytesMut) -> Result<Self> {
//         let size: Varint = Varint::decode(bytes)?;
//         if (size as usize) > bytes.len() {
//             bail!("Not enough bytes to read");
//         }
//         let mut packet = Self::with_capacity(size as usize);
//         packet.data = bytes.split();
//         let packet_id = packet.read_varint()?;
//         packet.set_packet_id(packet_id);
//         Ok(packet)
//     }
// }
