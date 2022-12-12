use std::{
    fmt::{Debug, Display},
    io::Read,
};

use crate::traits::McEncodable;

#[derive(PartialEq, Eq)]
pub struct Varint {
    value: i32,
}

impl Varint {
    pub fn from(v: i32) -> Varint {
        Self { value: v }
    }

    pub fn to_value(&self) -> i32 {
        self.value
    }
}

impl Display for Varint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for Varint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl McEncodable for Varint {
    fn read(buf: &mut std::io::Cursor<&[u8]>) -> color_eyre::Result<Self> {
        let mut num_read = 0;
        let mut result: i32 = 0;
        let mut read: u8;
        loop {
            let mut byte_buf = [0u8; 1];
            buf.read_exact(&mut byte_buf)?;
            read = byte_buf[0];
            let value: i32 = (read & 0x7F) as i32;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return Err(color_eyre::eyre::eyre!("VarInt is too big"));
            }
            if (read & 0x80) == 0 {
                break;
            }
        }
        Ok(Self::from(result))
    }

    fn write(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        let mut value = u32::from_le_bytes(self.value.to_le_bytes());
        loop {
            let mut temp: u8 = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0x80;
            }
            buf.write_all(&[temp])?;
            if value == 0 {
                break;
            }
        }
        Ok(())
    }
}
