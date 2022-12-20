use std::{
    fmt::{Debug, Display},
    io::Read,
};

use crate::traits::McEncodable;

pub fn varint_size(value: i32) -> color_eyre::Result<i32> {
    if value < 0 {
        color_eyre::eyre::bail!("Varint cannot be negative");
    } else if value < 0x80 {
        Ok(1)
    } else if value < 0x4000 {
        Ok(2)
    } else if value < 0x200000 {
        Ok(3)
    } else if value < 0x10000000 {
        Ok(4)
    } else {
        Ok(5)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Varint {
    value: i32,
}

impl Varint {
    pub fn from(v: i32) -> Varint {
        Self { value: v }
    }

    pub fn value(&self) -> i32 {
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

impl From<i32> for Varint {
    fn from(v: i32) -> Self {
        Self::from(v)
    }
}

impl From<Varint> for i32 {
    fn from(v: Varint) -> Self {
        v.value
    }
}

impl McEncodable for Varint {
    fn decode(buf: &mut impl Read) -> color_eyre::Result<Self> {
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

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
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

#[cfg(test)]
mod varint_test {
    use crate::McEncodable;

    const TEST: &[(i32, &[u8])] = &[
        (0, &[0x00]),
        (1, &[0x01]),
        (127, &[0x7f]),
        (128, &[0x80, 1]),
        (255, &[0xff, 0x01]),
        (25565, &[0xdd, 0xc7, 0x01]),
        (2097151, &[0xff, 0xff, 0x7f]),
        (2147483647, &[0xff, 0xff, 0xff, 0xff, 0x07]),
        (-1, &[0xff, 0xff, 0xff, 0xff, 0x0f]),
        (-2147483648, &[0x80, 0x80, 0x80, 0x80, 0x08]),
    ];

    #[test]
    fn test_varint_decode() {
        for (expected, bytes) in TEST.iter() {
            let mut buf = std::io::Cursor::new(bytes.to_owned());
            let varint = super::Varint::decode(&mut buf).unwrap();
            assert_eq!(varint.value(), *expected);
        }
    }

    #[test]
    fn test_varint_encode() {
        for (expected, bytes) in TEST.iter() {
            let mut buf = Vec::new();
            let varint = super::Varint::from(*expected);
            varint.encode(&mut buf).unwrap();
            assert_eq!(buf, *bytes);
        }
    }
}
