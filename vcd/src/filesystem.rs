use std::{cell::RefCell, rc::Rc};

use crate::structures::{FileNode, TreeNode};

pub fn load_tree() -> Rc<TreeNode> {
    let root = Rc::new(TreeNode::new(FileNode {
          name: String::from("/"),
    }));

    let home = Rc::new(TreeNode::new(FileNode {
        name: String::from("home"),
    }));

    let bin = Rc::new(TreeNode::new(FileNode {
        name: String::from("bin"),
    }));

    *root.subnodes.borrow_mut() = Some(vec![bin, home]);
    root
}
