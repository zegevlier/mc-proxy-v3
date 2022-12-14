mod bool;
mod chat;
mod identifier;
mod nums;
mod option;
mod string;
mod varint;
mod varlong;
mod vec;

pub use chat::Chat;
pub use identifier::Identifier;
pub use varint::varint_size;
pub use varint::Varint;

pub struct Compression {
    pub threshold: i32,
}

impl Default for Compression {
    fn default() -> Self {
        Self { threshold: -1 }
    }
}
