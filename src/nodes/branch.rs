use crate::{nodes::Node, wrappers::byte::Byte};

pub struct Branch {
    pub branches: [Box<Node>; 16],
    pub value: Vec<Byte>,
}
