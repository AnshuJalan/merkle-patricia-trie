use crate::{nodes::Node, wrappers::path::Path};

#[derive(Clone, Debug)]
pub struct Extension {
    pub path: Path,
    pub next: Node,
}

impl Extension {
    pub fn new(path: Path, next: Node) -> Self {
        Self { path, next }
    }
}
