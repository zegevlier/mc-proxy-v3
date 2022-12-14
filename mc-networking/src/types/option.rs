use crate::traits::McEncodable;

impl<T> McEncodable for Option<T>
where
    T: McEncodable,
{
    fn decode(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        match bool::decode(buf)? {
            true => Ok(Some(T::decode(buf)?)),
            false => Ok(None),
        }
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        match self {
            Some(value) => {
                true.encode(buf)?;
                value.encode(buf)?;
            }
            None => {
                false.encode(buf)?;
            }
        }
        Ok(())
    }
}
