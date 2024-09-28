pub mod addon;
pub mod commands;

use anyhow::{anyhow, Context, Result};
use std::path::Path;

pub fn is_bds_exist<P: AsRef<Path>>(path: P) -> Result<bool> {
    let extries = path
        .as_ref()
        .read_dir()
        .with_context(|| "Unable to read working directory")?;

    for entry in extries.flatten() {
        if let Some(str) = entry.file_name().to_str() {
            if str.starts_with("bedrock_server.exe") {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn check_bds_exist<P: AsRef<Path>>(path: P) -> Result<()> {
    if is_bds_exist(path.as_ref())? {
        Ok(())
    } else {
        Err(anyhow!(
            "bds does not exist in the current working directory"
        ))
    }
}
#[inline]
pub fn is_bedrock_level<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.join("levelname.txt").exists()
        && path.join("db").exists()
        && path.join("level.dat").exists()
        && path.join("level.dat_old").exists()
}
#[inline]
pub fn is_bds_root<P: AsRef<Path>>(path: P) -> bool {
    if let Ok(true) = is_bds_exist(&path) {
        return true;
    }
    path.as_ref().join("worlds").exists()
        && path.as_ref().join("bedrock_server_how_to.html").exists()
}

#[derive(Debug, PartialEq)]
pub enum WorkDirType {
    Bds,
    Level,
}

#[derive(thiserror::Error, Debug)]
#[error("The current working path is neither the bds server root nor the archive")]
pub struct IllegalWorkingPaths;

pub fn get_work_path_type<P: AsRef<Path>>(path: P) -> Result<WorkDirType, IllegalWorkingPaths> {
    if is_bedrock_level(&path) {
        Ok(WorkDirType::Level)
    } else if is_bds_root(&path) {
        Ok(WorkDirType::Bds)
    } else {
        Err(IllegalWorkingPaths)
    }
}
