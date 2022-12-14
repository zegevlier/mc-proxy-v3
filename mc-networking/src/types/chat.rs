use std::fmt::{Debug, Display};

use crate::traits::McEncodable;

#[derive(PartialEq, Eq)]
pub struct Chat {
    contents: String,
}

impl Chat {
    pub fn from(s: String) -> Chat {
        Self { contents: s }
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }
}

impl Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl Debug for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl McEncodable for Chat {
    fn decode(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        String::decode(buf).map(Self::from)
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        self.contents.encode(buf)?;
        Ok(())
    }
}
