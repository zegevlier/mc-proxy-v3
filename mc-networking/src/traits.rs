use std::io::{Cursor, Read, Write};

use crate::types::{varint_size, Compression, Varint};

pub trait McEncodable: Sized {
    fn decode(buf: &mut impl Read) -> color_eyre::Result<Self>;
    fn encode(&self, buf: &mut impl Write) -> color_eyre::Result<()>;
}

pub trait Packet: McEncodable {
    // TODO: Add verison parameter
    fn id(&self) -> i32;

    fn read_packet(buf: &mut impl Read) -> color_eyre::Result<Self> {
        Self::decode(buf)
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

pub trait PacketEncoder {
    fn write_packet(
        &self,
        buf: &mut impl Write,
        compression: Compression,
    ) -> color_eyre::Result<()>;
}

#[macro_export]
macro_rules! packet {
    ($name:ident, $id: expr) => {
        impl $crate::traits::Packet for $name {
            fn id(&self) -> i32 {
                $id
            }
        }
    };
}
