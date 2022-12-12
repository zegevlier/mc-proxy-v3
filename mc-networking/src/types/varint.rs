pub struct Varint {
    value: i32,
}

impl Varint {
    pub fn from_value(v: i32) -> Varint {
        Self { value: v }
    }

    pub fn to_value(&self) -> i32 {
        self.value
    }
}
