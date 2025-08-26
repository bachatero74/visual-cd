use std::{
    fs::{self, DirEntry},
    path::Path,
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

