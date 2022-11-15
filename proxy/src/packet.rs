use anyhow::{bail, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::utils::get_varint_size;

#[derive(Debug)]
pub struct Packet {
    data: BytesMut,
    packet_id: Option<i32>,
}

impl Packet {
    pub fn new() -> Self {
        Self {
            data: BytesMut::new(),
            packet_id: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: BytesMut::with_capacity(capacity),
            packet_id: None,
        }
    }

    pub fn write_bytes(&mut self, value: BytesMut) {
        self.data.put(value);
    }

    pub fn set_packet_id(&mut self, packet_id: i32) {
        self.packet_id = Some(packet_id);
    }

    fn get_size(&self) -> Result<i32> {
        if let Some(packet_id) = self.packet_id {
            Ok(self.data.len() as i32 + get_varint_size(packet_id)?)
        } else {
            Ok(self.data.len() as i32)
        }
    }

    pub fn get_packet_bytes(mut self, compressed: i32) -> Result<Bytes> {
        if compressed > 0 {
            todo!("Compression is not implemented yet");
        }

        if self.packet_id.is_none() {
            bail!("Packet ID is not set");
        }

        let mut packet = Packet::new();
        packet.write_varint(self.get_size()?);
        packet.write_varint(self.packet_id.unwrap());
        packet.write_bytes(self.data.split());

        Ok(packet.data.freeze())
    }

    pub fn read_from_bytes(bytes: &mut BytesMut) -> Result<Self> {
        let size = bytes.read_varint()?;
        let mut packet = Packet::with_capacity(size as usize);
        packet.data = bytes.split();
        let packet_id = packet.read_varint()?;
        packet.set_packet_id(packet_id);
        Ok(packet)
    }
}

pub trait PacketReaderWriter {
    fn read_boolean(&mut self) -> Result<bool>;
    fn write_boolean(&mut self, value: bool);

    fn read_byte(&mut self) -> Result<i8>;
    fn write_byte(&mut self, value: i8);

    fn read_unsigned_byte(&mut self) -> Result<u8>;
    fn write_unsigned_byte(&mut self, value: u8);

    fn read_short(&mut self) -> Result<i16>;
    fn write_short(&mut self, value: i16);

    fn read_unsigned_short(&mut self) -> Result<u16>;
    fn write_unsigned_short(&mut self, value: u16);

    fn read_int(&mut self) -> Result<i32>;
    fn write_int(&mut self, value: i32);

    fn read_long(&mut self) -> Result<i64>;
    fn write_long(&mut self, value: i64);

    fn read_float(&mut self) -> Result<f32>;
    fn write_float(&mut self, value: f32);

    fn read_double(&mut self) -> Result<f64>;
    fn write_double(&mut self, value: f64);

    fn read_string(&mut self) -> Result<String>;
    fn write_string(&mut self, value: String);

    // fn read_chat(&mut self) -> Result<Chat>;
    // fn write_chat(&mut self, value: Chat);

    // fn read_identifier(&mut self) -> Result<Identifier>;
    // fn write_identifier(&mut self, value: Identifier);

    fn read_varint(&mut self) -> Result<i32>;
    fn write_varint(&mut self, value: i32);

    fn read_varlong(&mut self) -> Result<i64>;
    fn write_varlong(&mut self, value: i64);

    // fn read_entity_metadata(&mut self) -> Result<Vec<EntityMetadata>>;
    // fn write_entity_metadata(&mut self, value: Vec<EntityMetadata>);

    // fn read_slot(&mut self) -> Result<Slot>;
    // fn write_slot(&mut self, value: Slot);

    // fn read_nbt(&mut self) -> Result<Nbt>;
    // fn write_nbt(&mut self, value: Nbt);

    // fn read_position(&mut self) -> Result<Position>;
    // fn write_position(&mut self, value: Position);

    // fn read_angle(&mut self) -> Result<Angle>;
    // fn write_angle(&mut self, value: Angle);

    // fn read_uuid(&mut self) -> Result<Uuid>;
    // fn write_uuid(&mut self, value: Uuid);

    // fn read_optional(&mut self, reader: impl Fn(&mut Self) -> Result<T>) -> Result<Option<T>>;
    // fn write_optional(&mut self, value: Option<T>, writer: impl Fn(&mut Self, T) -> Result<()>);

    // fn read_array(&mut self, size: usize, reader: impl Fn(&mut Self) -> Result<T>) -> Result<Vec<T>>;
    // fn write_array(&mut self, value: Vec<T>, writer: impl Fn(&mut Self, T) -> Result<()>);

    // fn read_varint_array(&mut self, reader: impl Fn(&mut Self) -> Result<T>) -> Result<Vec<T>>;
    // fn write_varint_array(&mut self, value: Vec<T>, writer: impl Fn(&mut Self, T) -> Result<()>);
}

impl PacketReaderWriter for BytesMut {
    fn read_boolean(&mut self) -> Result<bool> {
        Ok(self.read_byte()? != 0)
    }

    fn write_boolean(&mut self, value: bool) {
        self.write_byte(if value { 1 } else { 0 });
    }

    fn read_byte(&mut self) -> Result<i8> {
        if self.remaining() < 1 {
            bail!("Not enough bytes to read a byte");
        }

        Ok(self.get_i8())
    }

    fn write_byte(&mut self, value: i8) {
        self.put_i8(value);
    }

    fn read_unsigned_byte(&mut self) -> Result<u8> {
        if self.remaining() < 1 {
            bail!("Not enough bytes to read an unsigned byte");
        }

        Ok(self.get_u8())
    }

    fn write_unsigned_byte(&mut self, value: u8) {
        self.put_u8(value);
    }

    fn read_short(&mut self) -> Result<i16> {
        if self.remaining() < 2 {
            bail!("Not enough data to read short");
        }

        Ok(self.get_i16())
    }

    fn write_short(&mut self, value: i16) {
        self.put_i16(value);
    }

    fn read_unsigned_short(&mut self) -> Result<u16> {
        if self.remaining() < 2 {
            bail!("Not enough data to read unsigned short");
        }

        Ok(self.get_u16())
    }

    fn write_unsigned_short(&mut self, value: u16) {
        self.put_u16(value);
    }

    fn read_int(&mut self) -> Result<i32> {
        if self.remaining() < 4 {
            bail!("Not enough data to read int");
        }

        Ok(self.get_i32())
    }

    fn write_int(&mut self, value: i32) {
        self.put_i32(value);
    }

    fn read_long(&mut self) -> Result<i64> {
        if self.remaining() < 8 {
            bail!("Not enough data to read long");
        }

        Ok(self.get_i64())
    }

    fn write_long(&mut self, value: i64) {
        self.put_i64(value);
    }

    fn read_float(&mut self) -> Result<f32> {
        if self.remaining() < 4 {
            bail!("Not enough data to read float");
        }

        Ok(self.get_f32())
    }

    fn write_float(&mut self, value: f32) {
        self.put_f32(value);
    }

    fn read_double(&mut self) -> Result<f64> {
        if self.remaining() < 8 {
            bail!("Not enough data to read double");
        }

        Ok(self.get_f64())
    }

    fn write_double(&mut self, value: f64) {
        self.put_f64(value);
    }

    fn read_string(&mut self) -> Result<String> {
        let length = self.read_varint()? as usize;

        if self.remaining() < length {
            bail!("Not enough data to read string");
        }

        let bytes = match self.get(0..length) {
            Some(bytes) => bytes.to_vec(),
            None => bail!("Not enough data to read string"),
        };

        Ok(String::from_utf8(bytes)?)
    }

    fn write_string(&mut self, value: String) {
        let bytes = value.into_bytes();
        self.write_varint(bytes.len() as i32);
        self.put(&*bytes);
    }

    fn read_varint(&mut self) -> Result<i32> {
        let mut num_read = 0;
        let mut result: i32 = 0;
        let mut read: u8;
        loop {
            read = self.read_unsigned_byte()?;
            let value: i32 = (read & 0x7F) as i32;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                bail!("VarInt is too big");
            }
            if (read & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }

    fn write_varint(&mut self, value: i32) {
        let mut value = u32::from_le_bytes(value.to_le_bytes());
        loop {
            let mut temp: u8 = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0x80;
            }
            self.write_unsigned_byte(temp);
            if value == 0 {
                break;
            }
        }
    }

    fn read_varlong(&mut self) -> Result<i64> {
        let mut num_read = 0;
        let mut result: i64 = 0;
        let mut read: u8;
        loop {
            read = self.read_unsigned_byte()?;
            let value: i64 = (read & 0x7F) as i64;
            result |= value << (7 * num_read);

            num_read += 1;
            if num_read > 10 {
                bail!("VarLong is too big");
            }
            if (read & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }

    fn write_varlong(&mut self, value: i64) {
        let mut value = u64::from_le_bytes(value.to_le_bytes());
        loop {
            let mut temp: u8 = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0x80;
            }
            self.write_unsigned_byte(temp);
            if value == 0 {
                break;
            }
        }
    }
}

impl PacketReaderWriter for Packet {
    fn read_boolean(&mut self) -> Result<bool> {
        self.data.read_boolean()
    }

    fn write_boolean(&mut self, value: bool) {
        self.data.write_boolean(value);
    }

    fn read_byte(&mut self) -> Result<i8> {
        self.data.read_byte()
    }

    fn write_byte(&mut self, value: i8) {
        self.data.write_byte(value);
    }

    fn read_unsigned_byte(&mut self) -> Result<u8> {
        self.data.read_unsigned_byte()
    }

    fn write_unsigned_byte(&mut self, value: u8) {
        self.data.write_unsigned_byte(value);
    }

    fn read_short(&mut self) -> Result<i16> {
        self.data.read_short()
    }

    fn write_short(&mut self, value: i16) {
        self.data.write_short(value);
    }

    fn read_unsigned_short(&mut self) -> Result<u16> {
        self.data.read_unsigned_short()
    }

    fn write_unsigned_short(&mut self, value: u16) {
        self.data.write_unsigned_short(value);
    }

    fn read_int(&mut self) -> Result<i32> {
        self.data.read_int()
    }

    fn write_int(&mut self, value: i32) {
        self.data.write_int(value);
    }

    fn read_long(&mut self) -> Result<i64> {
        self.data.read_long()
    }

    fn write_long(&mut self, value: i64) {
        self.data.write_long(value);
    }

    fn read_float(&mut self) -> Result<f32> {
        self.data.read_float()
    }

    fn write_float(&mut self, value: f32) {
        self.data.write_float(value);
    }

    fn read_double(&mut self) -> Result<f64> {
        self.data.read_double()
    }

    fn write_double(&mut self, value: f64) {
        self.data.write_double(value);
    }

    fn read_string(&mut self) -> Result<String> {
        self.data.read_string()
    }

    fn write_string(&mut self, value: String) {
        self.data.write_string(value);
    }

    fn read_varint(&mut self) -> Result<i32> {
        self.data.read_varint()
    }

    fn write_varint(&mut self, value: i32) {
        self.data.write_varint(value);
    }

    fn read_varlong(&mut self) -> Result<i64> {
        self.data.read_varlong()
    }

    fn write_varlong(&mut self, value: i64) {
        self.data.write_varlong(value);
    }
}

#[cfg(test)]
mod varint_test {
    use anyhow::Result;
    use bytes::BytesMut;

    use super::{Packet, PacketReaderWriter};

    impl Packet {
        pub fn data_mut(&mut self) -> &mut BytesMut {
            &mut self.data
        }

        pub fn data(&self) -> &BytesMut {
            &self.data
        }
    }

    const TEST_OUTPUTS: [i32; 11] = [
        0,
        1,
        2,
        127,
        128,
        255,
        25565,
        2097151,
        2147483647,
        -1,
        -2147483648,
    ];

    const TEST_INPUTS: [[u8; 5]; 11] = [
        [0x00, 0x00, 0x00, 0x00, 0x00],
        [0x01, 0x00, 0x00, 0x00, 0x00],
        [0x02, 0x00, 0x00, 0x00, 0x00],
        [0x7F, 0x00, 0x00, 0x00, 0x00],
        [0x80, 0x01, 0x00, 0x00, 0x00],
        [0xFF, 0x01, 0x00, 0x00, 0x00],
        [0xdd, 0xC7, 0x01, 0x00, 0x00],
        [0xff, 0xFF, 0x7f, 0x00, 0x00],
        [0xff, 0xFF, 0xff, 0xff, 0x07],
        [0xff, 0xFF, 0xff, 0xff, 0x0f],
        [0x80, 0x80, 0x80, 0x80, 0x08],
    ];

    #[test]

    fn read() -> Result<()> {
        let mut packet = Packet::new();
        packet
            .data_mut()
            .extend_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);

        assert_eq!(packet.read_varint()?, 0x01);
        assert_eq!(packet.read_varint()?, 0x02);
        assert_eq!(packet.read_varint()?, 0x03);
        assert_eq!(packet.read_varint()?, 0x04);
        assert_eq!(packet.read_varint()?, 0x05);
        assert_eq!(packet.read_varint()?, 0x06);
        assert!(packet.read_varint().is_err());

        for (input, output) in TEST_INPUTS.iter().zip(TEST_OUTPUTS.iter()) {
            let mut packet = Packet::new();
            packet.data_mut().extend_from_slice(input);
            assert_eq!(packet.read_varint()?, *output);
        }

        Ok(())
    }

    #[test]
    fn write() -> Result<()> {
        let mut packet = Packet::new();
        packet.write_varint(0x01);
        packet.write_varint(0x02);
        packet.write_varint(0x03);
        packet.write_varint(0x04);
        packet.write_varint(0x05);
        packet.write_varint(0x06);
        assert_eq!(
            packet.data().to_vec(),
            &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
        );

        for (input, output) in TEST_INPUTS.iter().zip(TEST_OUTPUTS.iter()) {
            let mut packet = Packet::new();
            packet.write_varint(*output);
            let packet_data = packet.data().to_vec();
            packet_data.iter().zip(input.iter()).for_each(|(a, b)| {
                assert_eq!(a, b);
            });
        }

        Ok(())
    }
}
