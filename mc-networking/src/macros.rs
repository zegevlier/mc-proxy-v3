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
            $($c_name:ident),* $(,)?
            $(@$c_id:literal => {
                $($c_version:pat => $c_name2:ident),* $(,)?
            }$(,)?)* $(,)?
        }
        $serverbound:ident {
            $($s_name:ident),*$(,)?
            $(@$s_id:literal => {
                $($s_version:pat => $s_name2:ident),* $(,)?
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
                compression: $crate::types::Compression,
            ) -> color_eyre::Result<()> {
                match self {
                    $packet::Clientbound(packet) => packet.write_packet(buf, compression)?,
                    $packet::Serverbound(packet) => packet.write_packet(buf, compression)?,
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
                pub(crate) fn write_packet(&self, buf: &mut impl std::io::Write, compression: $crate::types::Compression) -> color_eyre::Result<()> {
                    match self {
                        $($clientbound::$c_name(packet) => packet.write_packet(buf, compression)?,)*
                        _ => color_eyre::eyre::bail!("Unexpected packet: {:?}", self),
                    }
                    Ok(())
                }
            }

            #[derive(Debug)]
            pub enum $serverbound {
                $($s_name($s_name)),*
            }

            impl $serverbound {
                pub(crate) fn write_packet(&self, buf: &mut impl std::io::Write, compression: $crate::types::Compression) -> color_eyre::Result<()> {
                    match self {
                        $($serverbound::$s_name(packet) => packet.write_packet(buf, compression)?),*
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
                                $($c_id => {
                                    match version.to_id() {
                                        $($c_version => $clientbound::$c_name2(<$c_name2>::decode(buf)?),)*
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
                                $($s_id => {
                                    match version.to_id() {
                                        $($s_version => $serverbound::$s_name2(<$s_name2>::decode(buf)?),)*
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
