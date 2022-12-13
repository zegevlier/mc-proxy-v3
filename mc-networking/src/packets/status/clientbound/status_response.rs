use crate::traits::{McEncodable, Packet};

#[derive(Debug, PartialEq, Eq)]
pub struct StatusResponse {
    pub json_response: String,
}

impl McEncodable for StatusResponse {
    fn decode(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        Ok(Self {
            json_response: String::decode(buf)?,
        })
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        self.json_response.encode(buf)?;
        Ok(())
    }
}

impl Packet for StatusResponse {
    fn id(&self) -> i32 {
        0x00
    }
}
