use std::{cell::RefCell, rc::Rc};

use crate::structures::{FileNode, TreeNode};

pub fn read_dir(parent: &str) -> Vec<TreeNode> {
    let home = TreeNode::new(FileNode {
        name: String::from("home"),
    });

    let bin = TreeNode::new(FileNode {
        name: String::from("bin"),
    });

    vec![bin, home]
}
