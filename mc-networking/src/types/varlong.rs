use std::{
    fmt::{Debug, Display},
    io::Read,
};

use crate::traits::McEncodable;

#[derive(PartialEq, Eq)]
pub struct Varlong {
    value: i64,
}

impl Varlong {
    pub fn from(v: i64) -> Varlong {
        Self { value: v }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl Display for Varlong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for Varlong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl McEncodable for Varlong {
    fn decode(buf: &mut impl Read) -> color_eyre::Result<Self> {
        let mut num_read = 0;
        let mut result: i64 = 0;
        let mut read: u8;
        loop {
            let mut byte_buf = [0u8; 1];
            buf.read_exact(&mut byte_buf)?;
            read = byte_buf[0];
            let value: i64 = (read & 0x7F) as i64;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 10 {
                return Err(color_eyre::eyre::eyre!("VarLong is too big"));
            }
            if (read & 0x80) == 0 {
                break;
            }
        }
        Ok(Self::from(result))
    }

    fn encode(&self, buf: &mut impl std::io::Write) -> color_eyre::Result<()> {
        let mut value = u64::from_le_bytes(self.value.to_le_bytes());
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
mod varlong_test {
    use crate::McEncodable;

    const TEST: &[(i64, &[u8])] = &[
        (0, &[0x00]),
        (1, &[0x01]),
        (127, &[0x7f]),
        (128, &[0x80, 0x01]),
        (255, &[0xff, 0x01]),
        (25565, &[0xdd, 0xc7, 0x01]),
        (2097151, &[0xff, 0xff, 0x7f]),
        (2147483647, &[0xff, 0xff, 0xff, 0xff, 0x07]),
        (
            9223372036854775807,
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
        ),
        (
            -1,
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
        ),
        (
            -2147483648,
            &[0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
        ),
        (
            -9223372036854775808,
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        ),
    ];

    #[test]
    fn test_varlong_decode() {
        for (expected, bytes) in TEST.iter() {
            let mut buf = std::io::Cursor::new(bytes.to_owned());
            let varlong = super::Varlong::decode(&mut buf).unwrap();
            assert_eq!(varlong.value(), *expected);
        }
    }

    #[test]
    fn test_varlong_encode() {
        for (expected, bytes) in TEST.iter() {
            let mut buf = Vec::new();
            let varlong = super::Varlong::from(*expected);
            varlong.encode(&mut buf).unwrap();
            assert_eq!(buf, *bytes);
        }
    }
}
