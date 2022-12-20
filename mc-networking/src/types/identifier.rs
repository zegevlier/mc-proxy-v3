use std::{
    fmt::{Debug, Display},
    io::Read,
};

use crate::traits::McEncodable;

#[derive(PartialEq, Eq)]
pub struct Identifier {
    contents: String,
}

impl Identifier {
    pub fn from(s: String) -> Identifier {
        Self { contents: s }
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl McEncodable for Identifier {
    fn decode(buf: &mut impl Read) -> color_eyre::Result<Self> {
        String::decode(buf).map(Self::from)
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        self.contents.encode(buf)?;
        Ok(())
    }
}
