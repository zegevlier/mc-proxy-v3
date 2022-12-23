use std::{
    fmt::{Debug, Display},
    io::Read,
};

use crate::traits::McEncodable;

#[derive(PartialEq, Eq)]
pub struct Uuid {
    contents: u128,
}

impl Uuid {
    pub fn from(s: u128) -> Uuid {
        Self { contents: s }
    }

    pub fn contents(&self) -> u128 {
        self.contents
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.contents)
    }
}

impl Debug for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.contents)
    }
}

impl McEncodable for Uuid {
    fn decode(buf: &mut impl Read) -> color_eyre::Result<Self> {
        let mut byte_buf = [0u8; 16];
        buf.read_exact(&mut byte_buf)?;
        Ok(Uuid::from(u128::from_be_bytes(byte_buf)))
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        Ok(buf.write_all(&self.contents.to_be_bytes())?)
    }
}
