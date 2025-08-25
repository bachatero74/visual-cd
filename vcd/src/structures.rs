use std::{
    cell::RefCell,
    ffi::OsString,
    path::PathBuf,
    rc::{Rc, Weak},
};

use log::info;

use crate::filesystem::read_dir;

pub struct FileNode {
    pub name: OsString,
}

pub struct TreeNode {
    pub file_node: FileNode,
    pub subnodes: RefCell<Option<Vec<Rc<TreeNode>>>>,
    parent: Weak<TreeNode>,
}

impl TreeNode {
    pub fn new(file_node: FileNode) -> Self {
        Self {
            file_node,
            subnodes: RefCell::new(None),
            parent: Weak::new(),
        }
    }

    pub fn load(self: &Rc<TreeNode>) {
        let mut opt_nodes = self.subnodes.borrow_mut();
        if opt_nodes.is_none() {
            *opt_nodes = Some(
                read_dir("")
                    .map(|mut n| {
                        n.parent = Rc::downgrade(self);
                        Rc::new(n)
                    })
                    .collect(),
            );
        }
    }

    pub fn get_path(self: &Rc<TreeNode>) -> PathBuf {
        fn collect_path(node: &Rc<TreeNode>, path: &mut PathBuf) {
            if let Some(parent) = node.parent.upgrade() {
                collect_path(&parent, path);
            }
            path.push(&node.file_node.name);
        }

        let mut path = PathBuf::new();
        collect_path(self, &mut path);
        path
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
