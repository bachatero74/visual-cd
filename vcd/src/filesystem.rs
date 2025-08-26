use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use log::info;

use crate::{errors::AppError, structures::FileNode};

pub fn read_dir(path: &Path) -> Result<impl Iterator<Item = FileNode>, AppError> {
    info!("reading {}", path.display());

    Ok(fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| match entry.file_type() {
            Ok(ft) => ft.is_dir(),
            Err(_) => false,
        })
        .map(|e| FileNode {
            name: e.file_name(),
        }))
}

pub fn get_current_root() -> Result<OsString, AppError> {
    let cwd = env::current_dir()?;
    let root = cwd
        .iter()
        .next()
        .ok_or_else(|| AppError::StatStr("Cannot get current dir"))?;
    Ok(root.into())
}
