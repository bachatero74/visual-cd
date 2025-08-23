use std::{cell::RefCell, rc::Rc};

pub struct FileNode {
    pub name: String,
}

pub struct TreeNode {
    pub file_node: FileNode,
    pub subnodes: RefCell<Option<Vec<Rc<TreeNode>>>>,
}

impl TreeNode {
    pub fn new(file_node: FileNode) -> Self {
        Self {
            file_node,
            subnodes: RefCell::new(None),
        }
    }
}

pub struct TVItem {
    pub tree_node: Rc<TreeNode>,
    pub drawing: String,
}
