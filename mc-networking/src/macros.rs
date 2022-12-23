#[macro_export]
macro_rules! packets {
    ($packet:ident {
        $clientbound:ident {
            $($c_name:ident {
                $($c_version1:pat => $c_id1:literal),* $(,)?
            }),* $(,)?
            $(@$c_id2:literal => {
                $($c_version2:pat => $c_name2:ident),* $(,)?
            }$(,)?)* $(,)?
        }
        $serverbound:ident {
            $($s_name:ident {
                $($s_version1:pat => $s_id1:literal),* $(,)?
            }),*$(,)?
            $(@$s_id2:literal => {
                $($s_version2:pat => $s_name2:ident),* $(,)?
            }$(,)?)* $(,)?
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
            pub(crate) fn write_packet(
                &self,
                buf: &mut impl std::io::Write,
                version: $crate::versions::Version,
                compression: $crate::types::Compression,
            ) -> color_eyre::Result<()> {
                match self {
                    $packet::Clientbound(packet) => packet.write_packet(buf, version, compression)?,
                    $packet::Serverbound(packet) => packet.write_packet(buf, version, compression)?,
                }
                Ok(())
            }
        }

        #[derive(Debug)]
        pub enum $clientbound {
            $($c_name($c_name)),*
        }

        impl $clientbound {
            #[allow(unused_variables, unreachable_code, unreachable_patterns)]
            pub(crate) fn write_packet(&self, buf: &mut impl std::io::Write, version: $crate::versions::Version, compression: $crate::types::Compression) -> color_eyre::Result<()> {
                match self {
                    $($clientbound::$c_name(packet) => packet.write_packet(buf, version, compression)?,)*
                    _ => color_eyre::eyre::bail!("Unexpected packet: {:?}", self),
                }
                Ok(())
            }
        }

        #[derive(Debug)]
        pub enum $serverbound {
            $($s_name($s_name)),*
        }

        $(impl $crate::traits::Packet for $c_name {
            fn id(version: $crate::versions::Version) -> Option<i32> {
                match version.to_id() {
                    $($c_version1 => Some($c_id1),)*
                    _ => None,
                }
            }
        })*

        $(impl $crate::traits::Packet for $s_name {
            fn id(version: $crate::versions::Version) -> Option<i32> {
                match version.to_id() {
                    $($s_version1 => Some($s_id1),)*
                    _ => None,
                }
            }
        })*

        impl $serverbound {
            pub(crate) fn write_packet(&self, buf: &mut impl std::io::Write, version: $crate::versions::Version, compression: $crate::types::Compression) -> color_eyre::Result<()> {
                match self {
                    $($serverbound::$s_name(packet) => packet.write_packet(buf, version, compression)?),*
                    // _ => color_eyre::eyre::bail!("Unexpected packet: {:?}", self),
                }
                Ok(())
            }
        }

        impl $packet {
            pub fn decode_packet(
                direction: $crate::types::Direction,
                packet_id: i32,
                version: $crate::versions::Version,
                buf: &mut impl std::io::Read,
            ) -> color_eyre::Result<Self> {
                Ok(match direction {
                    Direction::Clientbound => {
                        #[allow(unused_variables)]
                        let packet = match packet_id {
                            $($c_id2 => {
                                match version.to_id() {
                                    $($c_version2 => $clientbound::$c_name2(<$c_name2>::decode(buf)?),)*
                                    _ => color_eyre::eyre::bail!("Unknown version: {:?}", version),
                                }
                            },)*
                            _ => color_eyre::eyre::bail!("Unknown packet id: {}", packet_id),
                        };
                        #[allow(unreachable_code)]
                        $packet::Clientbound(packet)
                    }
                    Direction::Serverbound => {
                        #[allow(unused_variables)]
                        let packet = match packet_id {
                            $($s_id2 => {
                                match version.to_id() {
                                    $($s_version2 => $serverbound::$s_name2(<$s_name2>::decode(buf)?),)*
                                    _ => color_eyre::eyre::bail!("Unknown version: {:?}", version),
                                }
                            },)*
                            _ => color_eyre::eyre::bail!("Unknown packet id: {}", packet_id),
                        };
                        #[allow(unreachable_code)]
                        $packet::Serverbound(packet)
                }})
            }
        }
    };
}
