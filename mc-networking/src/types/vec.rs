use crate::traits::McEncodable;

use super::Varint;

impl<T> McEncodable for Vec<T>
where
    T: McEncodable,
{
    fn decode(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        let length = crate::types::Varint::decode(buf)?.value() as usize;
        let mut vec = Vec::new();
        for _ in 0..length {
            vec.push(T::decode(buf)?);
        }
        Ok(vec)
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        Varint::from(self.len() as i32).encode(buf)?;
        for item in self {
            item.encode(buf)?;
        }
        Ok(())
    }
}
