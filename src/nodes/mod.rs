pub mod branch;
pub mod extension;
pub mod leaf;

pub enum Node {
    Null,
    Branch(branch::Branch),
    Leaf,
    Extension,
}
