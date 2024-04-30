use crate::nodes::Node;

#[derive(Clone)]
pub struct Branch {
    pub branches: [Node; 16],
    pub value: Option<Vec<u8>>,
}

impl Branch {
    pub fn new(branches: [Node; 16], value: Option<Vec<u8>>) -> Self {
        Self { branches, value }
    }

    pub fn set_branch(&mut self, branch_nibble: usize, node: Node) {
        self.branches[branch_nibble] = node;
    }

    pub fn set_value(&mut self, val: Vec<u8>) {
        self.value = Some(val);
    }
}
