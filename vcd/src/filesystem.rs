use std::{
    env,
    ffi::OsString,
    fs::{self},
    path::{Component, Path, PathBuf},
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
    let mut path=PathBuf::new();
    for comp in env::current_dir()?.components(){
        match comp {
            Component::Prefix(p) => { path.push(p.as_os_str()); },
            Component::RootDir => { path.push(comp.as_os_str()); },
            _ => break,
        }
    }

    Ok(path.into())
}

pub fn get_current_root3() -> Result<(Option<OsString>,OsString), AppError> {
    let mut path=PathBuf::new();
    let mut prefix: Option<OsString> = None;
    for comp in env::current_dir()?.components(){
        match comp {
            Component::Prefix(p) => {
                prefix = Some(p.as_os_str().into());
                path.push(p.as_os_str()); 
            },
            Component::RootDir => { path.push(comp.as_os_str()); },
            _ => break,
        }
    }

    Ok((prefix, path.into(),))
}