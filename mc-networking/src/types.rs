mod nums;
mod string;
pub mod varint;

pub use varint::Varint;

pub struct Compression {
    pub threshold: i32,
}

impl Default for Compression {
    fn default() -> Self {
        Self { threshold: -1 }
    }
}
