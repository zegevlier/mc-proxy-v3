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

#[macro_export]
macro_rules! packets {
    ($packet:ident {
        $clientbound:ident {
            $($id:literal -> $name:ident($ty:ty)),*
        }
        $serverbound:ident {
            $($id2:literal -> $name2:ident($ty2:ty)),*
        }
    }) => {
        use $crate::{traits::Packet, types::Direction, McEncodable};

        #[derive(Debug)]
        pub enum $packet {
            Clientbound($clientbound),
            Serverbound($serverbound),
        }

        impl $packet {
            #[allow(unused_variables, unreachable_code)]
            pub(crate) fn write_packet(&self, buf: &mut impl std::io::Write, compression: $crate::types::Compression) -> color_eyre::Result<()> {
                match self {
                    $packet::Clientbound(packet) => packet.write_packet(buf, compression)?,
                    $packet::Serverbound(packet) => packet.write_packet(buf, compression)?,
                }
                Ok(())
            }
        }

        #[derive(Debug)]
        pub enum $clientbound {
            $($name($ty)),*
        }

        impl $clientbound {
            #[allow(unused_variables, unreachable_code, unreachable_patterns)]
            pub(crate) fn write_packet(&self, buf: &mut impl std::io::Write, compression: $crate::types::Compression) -> color_eyre::Result<()> {
                match self {
                    $($clientbound::$name(packet) => packet.write_packet(buf, compression)?,)*
                    _ => color_eyre::eyre::bail!("Unexpected packet: {:?}", self),
                }
                Ok(())
            }
        }

        #[derive(Debug)]
        pub enum $serverbound {
            $($name2($ty2)),*
        }

        impl $serverbound {
            pub(crate) fn write_packet(&self, buf: &mut impl std::io::Write, compression: $crate::types::Compression) -> color_eyre::Result<()> {
                match self {
                    $($serverbound::$name2(packet) => packet.write_packet(buf, compression)?),*
                }
                Ok(())
            }
        }

        impl $packet {
            pub fn decode_packet(
                direction: $crate::types::Direction,
                packet_id: i32,
                buf: &mut impl std::io::Read,
            ) -> color_eyre::Result<Self> {
                Ok(match direction {
                    Direction::Clientbound => {
                        #[allow(unused_variables)]
                        let packet = match packet_id {
                            $($id => $clientbound::$name(<$ty>::decode(buf)?),)*
                            _ => color_eyre::eyre::bail!("Unknown packet id: {}", packet_id),
                        };
                        #[allow(unreachable_code)]
                        $packet::Clientbound(packet)
                    }
                    Direction::Serverbound => {
                        let packet = match packet_id {
                            $($id2 => $serverbound::$name2(<$ty2>::decode(buf)?),)*
                            _ => color_eyre::eyre::bail!("Unknown packet id: {}", packet_id),
                        };
                        $packet::Serverbound(packet)
                    }
                })
            }
        }
    }
}
