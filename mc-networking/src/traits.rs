use std::io::{Cursor, Read, Write};

use color_eyre::eyre::bail;

use crate::{
    types::{varint_size, Compression, Varint},
    versions::Version,
};

pub trait McEncodable: Sized {
    fn decode(buf: &mut impl Read) -> color_eyre::Result<Self>;
    fn encode(&self, buf: &mut impl Write) -> color_eyre::Result<()>;
}

pub trait Packet: McEncodable {
    // TODO: Add verison parameter
    fn id(version: Version) -> Option<i32>;

    fn read_packet(buf: &mut impl Read) -> color_eyre::Result<Self> {
        Self::decode(buf)
    }

    fn write_packet(
        &self,
        buf: &mut impl Write,
        version: Version,
        compression: Compression,
    ) -> color_eyre::Result<()> {
        assert!(compression.threshold == -1);
        let mut packet_buf = Cursor::new(Vec::new());
        self.encode(&mut packet_buf)?;
        let packet_buf = packet_buf.into_inner();
        let id = match Self::id(version) {
            Some(id) => id,
            None => bail!("No packet id for packet in version {:?}", version),
        };
        Varint::from(packet_buf.len() as i32 + varint_size(id)?).encode(buf)?;
        Varint::from(id).encode(buf)?;
        buf.write_all(&packet_buf)?;
        Ok(())
    }
}
