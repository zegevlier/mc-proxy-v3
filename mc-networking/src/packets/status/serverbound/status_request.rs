use crate::traits::{McEncodable, Packet};

#[derive(Debug, PartialEq, Eq)]
pub struct StatusRequest {}

impl McEncodable for StatusRequest {
    fn decode(_buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        Ok(Self {})
    }

    fn encode(&self, _buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        Ok(())
    }
}

impl Packet for StatusRequest {
    fn id(&self) -> i32 {
        0x00
    }
}
