mod bool;
mod chat;
mod identifier;
mod nums;
mod option;
mod string;
mod uuid;
mod varint;
mod varlong;
mod vec;

pub use chat::Chat;
pub use identifier::Identifier;
pub use uuid::Uuid;
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

#[derive(Debug, Clone, Copy)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Clientbound,
    Serverbound,
}
