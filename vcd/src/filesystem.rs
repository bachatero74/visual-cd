use std::{ffi::OsString, fs, path::Path};

use log::info;

use crate::{
    errors::AppError,
    structures::{FileNode, TreeNode},
};

pub fn read_dir(path: &Path) -> Result<impl Iterator<Item = TreeNode>, AppError> {
    info!("reading {}", path.display());

    let home = TreeNode::new(FileNode {
        name: OsString::from("home"),
    });

    let bin = TreeNode::new(FileNode {
        name: OsString::from("bin"),
    });

    if true {
    Ok(vec![bin, home].into_iter())
    }
    else{
    Err(AppError::StatStr(""))
    }
}
