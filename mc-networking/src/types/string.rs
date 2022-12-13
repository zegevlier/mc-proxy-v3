use std::io::Read;

use crate::traits::McEncodable;

use super::Varint;

impl McEncodable for String {
    fn decode(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        let length = crate::types::Varint::decode(buf)?.value() as usize;
        let mut byte_buf = vec![0u8; length];
        buf.read_exact(&mut byte_buf)?;
        Ok(String::from_utf8(byte_buf)?)
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        Varint::from(self.len() as i32).encode(buf)?;
        buf.write_all(self.as_bytes())?;
        Ok(())
    }
}
