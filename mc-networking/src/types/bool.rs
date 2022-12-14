use crate::traits::McEncodable;

impl McEncodable for bool {
    fn decode(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        Ok(u8::decode(buf)? != 0)
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        (*self as u8).encode(buf)?;
        Ok(())
    }
}
