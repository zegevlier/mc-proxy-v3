use std::io::Read;

use crate::traits::McEncodable;

use super::Varint;

impl McEncodable for String {
    fn read(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        let length = crate::types::Varint::read(buf)?.to_value() as usize;
        let mut byte_buf = vec![0u8; length];
        buf.read_exact(&mut byte_buf)?;
        Ok(String::from_utf8(byte_buf)?)
    }

    fn write(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        Varint::from(self.len() as i32).write(buf)?;
        buf.write_all(self.as_bytes())?;
        Ok(())
    }
}
