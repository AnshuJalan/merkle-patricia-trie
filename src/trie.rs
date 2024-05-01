use crate::{
    nodes::{branch, Node},
    wrappers::path::Path,
};

#[derive(Debug)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn put(&mut self, key: Vec<u8>, val: Vec<u8>) {
        let path = Path::from_bytes(key);
        self.root = Self::insert_rec(self.root.clone(), path, 0, val)
    }

    pub fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        let path = Path::from_bytes(key);
        Self::get_rec(&self.root, path, 0)
    }

    fn get_rec(node: &Node, path: Path, path_index: usize) -> Option<Vec<u8>> {
        let path_suffix = path.suffix(path_index);
        match node {
            Node::Empty => None,
            Node::Leaf(leaf) => {
                let matched_length = Path::match_prefix(&path_suffix, &leaf.path);
                if matched_length == path_suffix.len() && matched_length == leaf.path.len() {
                    Some(leaf.value.clone())
                } else {
                    None
                }
            }
            Node::Branch(branch) => {
                if path_suffix.len() == 0 {
                    branch.value.clone()
                } else {
                    Self::get_rec(
                        &branch.branches[path_suffix.get_nibble_at(0).to_u8() as usize],
                        path,
                        path_index + 1,
                    )
                }
            }
            Node::Extension(extension) => {
                let matched_length = Path::match_prefix(&path_suffix, &extension.path);
                if matched_length == extension.path.len() {
                    Self::get_rec(&extension.next, path, path_index + matched_length)
                } else {
                    None
                }
            }
        }
    }

    fn insert_rec(node: Node, path: Path, path_index: usize, val: Vec<u8>) -> Node {
        let path_suffix = path.suffix(path_index);
        match node {
            Node::Empty => Node::new_leaf_node(path_suffix, val),
            Node::Leaf(leaf) => {
                let matched_length = Path::match_prefix(&path_suffix, &leaf.path);

                if matched_length == path_suffix.len() && matched_length == leaf.path.len() {
                    return Node::new_leaf_node(leaf.path.clone(), val.clone());
                }

                let mut branch = branch::Branch::new(Default::default(), None);

                if matched_length == path_suffix.len() {
                    branch.set_value(val.clone());
                } else if matched_length == leaf.path.len() {
                    branch.set_value(leaf.value.clone());
                }

                if matched_length < path_suffix.len() {
                    let next_leaf_node =
                        Node::new_leaf_node(path_suffix.suffix(matched_length + 1), val.clone());
                    branch.set_branch(
                        path_suffix.get_nibble_at(matched_length).to_u8() as usize,
                        next_leaf_node,
                    );
                }
                if matched_length < leaf.path.len() {
                    let next_leaf_node = Node::new_leaf_node(
                        leaf.path.suffix(matched_length + 1),
                        leaf.value.clone(),
                    );
                    branch.set_branch(
                        leaf.path.get_nibble_at(matched_length).to_u8() as usize,
                        next_leaf_node,
                    );
                }

                let branch_node = Node::from_branch(branch);
                if matched_length > 0 {
                    Node::new_extension_node(path_suffix.prefix(matched_length), branch_node)
                } else {
                    branch_node
                }
            }
            Node::Branch(mut branch) => {
                if path_suffix.len() == 0 {
                    branch.set_value(val.clone());
                    return Node::Branch(branch);
                }

                let branch_nibble = path_suffix.get_nibble_at(0).to_u8() as usize;
                let next_node = Self::insert_rec(
                    branch.branches[branch_nibble].clone(),
                    path,
                    path_index + 1,
                    val,
                );
                branch.set_branch(branch_nibble, next_node);
                Node::Branch(branch)
            }
            Node::Extension(mut extension) => {
                let matched_length = Path::match_prefix(&path_suffix, &extension.path);

                if matched_length == extension.path.len() {
                    extension.next =
                        Self::insert_rec(extension.next, path, path_index + matched_length, val);
                    return Node::Extension(extension);
                } else if matched_length == 0 {
                    let mut branch = branch::Branch::new(Default::default(), None);

                    let matched_extension_path = extension.path.suffix(matched_length);
                    let matched_extention_node_nibble =
                        matched_extension_path.get_nibble_at(0).to_u8() as usize;
                    let matched_extension_node = if matched_extension_path.len() > 1 {
                        Node::new_extension_node(matched_extension_path.suffix(1), extension.next)
                    } else {
                        extension.next
                    };
                    branch.set_branch(matched_extention_node_nibble, matched_extension_node);

                    return Self::insert_rec(Node::from_branch(branch), path, path_index, val);
                }

                extension.path = extension.path.prefix(matched_length);
                extension.next = Self::insert_rec(
                    Node::new_extension_node(extension.path.suffix(matched_length), extension.next),
                    path,
                    path_index + matched_length,
                    val,
                );
                Node::Extension(extension)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_works_correctly() {
        let mut trie = Trie { root: Node::Empty };

        let k1 = "Jake".as_bytes().to_vec();
        let k2 = "Mike".as_bytes().to_vec();

        let v1 = "Paul".as_bytes().to_vec();
        let v2 = "Tyson".as_bytes().to_vec();

        trie.put(k1.clone(), v1.clone());
        trie.put(k2.clone(), v2.clone());

        assert!(trie.get(k1).is_some_and(|v| v == v1));
        assert!(trie.get(k2).is_some_and(|v| v == v2));
        assert!(trie.get("Logan".as_bytes().to_vec()).is_none());
    }
}
