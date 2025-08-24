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
        if self.subnodes.borrow().is_none() {
            *self.subnodes.borrow_mut() =
                Some(read_dir("").into_iter().map(|n| Rc::new(n)).collect());
        }
        if let Some(ref v)=*self.subnodes.borrow() {
            for i in v { info!("{}",i.file_node.name); }
        }
    }
}

pub struct TVItem {
    pub tree_node: Rc<TreeNode>,
    pub drawing: String,
}
