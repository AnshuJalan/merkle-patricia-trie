use super::byte::Byte;

pub struct Nibble(u8);

impl Nibble {
    pub fn from_byte(byte: Byte) -> (Self, Self) {
        (Self(byte.0 >> 4), Self(byte.0 % 16))
    }

    pub fn from_bytes(bytes: Vec<Byte>) -> Vec<Self> {
        let mut nibbles = vec![];
        for byte in bytes {
            nibbles.push(Self(byte.0 >> 4));
            nibbles.push(Self(byte.0 % 16));
        }
        nibbles
    }
}
