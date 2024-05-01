use crate::wrappers::path::Path;

#[derive(Clone, Debug)]
pub struct Leaf {
    pub path: Path,
    pub value: Vec<u8>,
}

impl Leaf {
    pub fn new(p: Path, v: Vec<u8>) -> Self {
        Self { path: p, value: v }
    }
}
