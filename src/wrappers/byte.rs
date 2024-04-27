pub struct Byte(pub u8);

impl Byte {
    pub fn from_u8(v: u8) -> Self {
        Self(v)
    }

    pub fn from_u8_vec(v: Vec<u8>) -> Vec<Self> {
        v.into_iter().map(|x| Byte(x)).collect()
    }
}
