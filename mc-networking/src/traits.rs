use std::io::{Cursor, Read, Write};

use crate::types::{varint::varint_size, Compression, Varint};

pub trait McEncodable: Sized {
    fn decode(buf: &mut Cursor<&[u8]>) -> color_eyre::Result<Self>;
    fn encode(&self, buf: &mut impl Write) -> color_eyre::Result<()>;
}

pub trait Packet: Sized + McEncodable {
    // TODO: Add verison parameter
    fn id(&self) -> i32;

    fn read_packet(buf: &mut Cursor<&[u8]>) -> color_eyre::Result<Self> {
        let length = Varint::decode(buf)?.value() as usize;
        let id = Varint::decode(buf)?.value();
        let mut packet_buf = vec![0u8; length - varint_size(id)? as usize];
        buf.read_exact(&mut packet_buf)?;
        let mut packet_buf = Cursor::new(packet_buf.as_slice());
        Self::decode(&mut packet_buf)
    }

    fn write_packet(
        &self,
        buf: &mut impl Write,
        compression: Compression,
    ) -> color_eyre::Result<()> {
        assert!(compression.threshold == -1);
        let mut packet_buf = Cursor::new(Vec::new());
        self.encode(&mut packet_buf)?;
        let packet_buf = packet_buf.into_inner();
        Varint::from(packet_buf.len() as i32 + varint_size(self.id())?).encode(buf)?;
        Varint::from(self.id()).encode(buf)?;
        buf.write_all(&packet_buf)?;
        Ok(())
    }
}
