use std::{cell::RefCell, rc::Rc};

use log::info;

use crate::filesystem::read_dir;

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

    pub fn load(&self) {
        let mut opt_nodes = self.subnodes.borrow_mut();
        if opt_nodes.is_none() {
            *opt_nodes = Some(read_dir("").map(|n| Rc::new(n)).collect());
        }
    }

    pub fn unload(&self) {
        let mut opt_nodes = self.subnodes.borrow_mut();
        *opt_nodes = None;
    }

}

pub struct TVItem {
    pub tree_node: Rc<TreeNode>,
    pub drawing: String,
}
