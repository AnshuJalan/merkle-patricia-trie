#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Nibble(u8);

impl Nibble {
    pub fn from_byte(byte: u8) -> (Self, Self) {
        (Self(byte >> 4), Self(byte % 16))
    }

    pub fn to_u8(self) -> u8 {
        self.0
    }

    pub fn eq(&self, a: &Nibble) -> bool {
        return *self == *a;
    }
}
