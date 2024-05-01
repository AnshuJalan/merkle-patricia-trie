use crate::wrappers::path::Path;

pub mod branch;
pub mod extension;
pub mod leaf;

#[derive(Clone, Default, Debug)]
pub enum Node {
    #[default]
    Empty,
    Branch(Box<branch::Branch>),
    Leaf(Box<leaf::Leaf>),
    Extension(Box<extension::Extension>),
}

impl Node {
    pub fn new_leaf_node(path: Path, value: Vec<u8>) -> Self {
        Self::Leaf(Box::new(leaf::Leaf::new(path, value)))
    }

    pub fn new_extension_node(path: Path, next: Node) -> Self {
        Self::Extension(Box::new(extension::Extension::new(path, next)))
    }

    pub fn from_branch(br: branch::Branch) -> Self {
        Self::Branch(Box::new(br))
    }
}
