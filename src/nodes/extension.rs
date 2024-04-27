use crate::{nodes::Node, wrappers::nibble::Nibble};

pub struct Extension {
    pub path: Vec<Nibble>,
    pub next: Box<Node>,
}
