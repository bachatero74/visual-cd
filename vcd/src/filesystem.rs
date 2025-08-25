use std::ffi::OsString;

use crate::structures::{FileNode, TreeNode};

pub fn read_dir(parent: &str) -> impl Iterator<Item = TreeNode> {
    let home = TreeNode::new(FileNode {
        name: OsString::from("home"),
    });

    let bin = TreeNode::new(FileNode {
        name: OsString::from("bin"),
    });

    vec![bin, home].into_iter()
}
