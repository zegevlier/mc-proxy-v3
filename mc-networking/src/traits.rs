use std::io::{Cursor, Write};

pub(crate) trait McEncodable: Sized {
    fn read(buf: &mut Cursor<&[u8]>) -> color_eyre::Result<Self>;
    fn write(&self, buf: &mut impl Write) -> color_eyre::Result<()>;
}
