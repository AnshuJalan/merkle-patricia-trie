use crate::wrappers::{byte::Byte, nibble::Nibble};

pub struct Leaf {
    pub path: Vec<Nibble>,
    pub value: Vec<Byte>,
}
