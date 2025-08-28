use std::{
    cell::RefCell,
    ffi::OsString,
    path::{Components, PathBuf},
    rc::{Rc, Weak},
};

use log::warn;

use crate::{errors::AppError, filesystem::read_dir};

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
            let my_path = self.get_path();
            if let Ok(dir_iter) =
                read_dir(&my_path).inspect_err(|_| warn!("Failed to read {}", my_path.display()))
            {
                *opt_nodes = Some(
                    dir_iter
                        .map(|n| {
                            let mut tn = TreeNode::new(n);
                            tn.parent = Rc::downgrade(self);
                            Rc::new(tn)
                        })
                        .collect(),
                );
            }
        }
    }

    pub fn find(self: &Rc<TreeNode>, components: &mut Components) -> Result<Rc<TreeNode>, AppError> {
        if let Some(next) = components.next() {
            self.load();
            let subs = self.subnodes.borrow();
            match *subs {
                Some(ref subs) => {
                    let name = next.as_os_str();
                    let found = subs
                        .iter()
                        .find(|n| n.file_node.name == name)
                        .ok_or(AppError::StatStr("Cannot find specified path"))?;
                    found.find(components)
                }
                None => Err(AppError::StatStr("Cannot find specified path")),
            }
        } else {
            Ok(Rc::clone(self))
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
