use std::io::Read;

use crate::traits::McEncodable;

macro_rules! impl_num {
    ($($num:ident),*) => {$(
        impl McEncodable for $num {
            fn decode(buf: &mut impl Read) -> color_eyre::Result<Self> {
                let mut byte_buf = [0u8; std::mem::size_of::<$num>()];
                buf.read_exact(&mut byte_buf)?;
                Ok($num::from_be_bytes(byte_buf))
            }

            fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
                Ok(buf.write_all(&self.to_be_bytes())?)
            }
        }
    )*};
}

impl_num!(i8, i16, i32, i64, u8, u16, f32, f64);
