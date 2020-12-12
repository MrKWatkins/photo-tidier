use std::fs::read_dir;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

pub fn read_files_in_dir_sorted<P: AsRef<Path>>(path: P) -> Result<impl Iterator<Item = PathBuf>> {
    let mut entries = read_dir(path)?
        .filter_map(|r| r.ok())
        .map(|d| d.path())
        .filter(|p| p.is_file())
        .collect::<Vec<PathBuf>>();

    entries.sort();

    return Ok(entries.into_iter());
}

pub fn validate_file_exists(file: &Path) -> Result<()> {
    validate_exists(file)?;

    if !file.is_file() {
        return Err(Error::new(ErrorKind::InvalidInput, format!("The path {:?} is not a file.", file)));
    }

    return Ok(());
}

pub fn validate_directory_exists(directory: &Path) -> Result<()> {
    validate_exists(directory)?;

    if !directory.is_dir() {
        return Err(Error::new(ErrorKind::InvalidInput, format!("The path {:?} is not a directory.", directory)));
    }

    return Ok(());
}

fn validate_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("The path {:?} does not exist.", path)));
    }

    return Ok(());
}
