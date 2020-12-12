mod configuration;
mod file;
mod photo;

use crate::file::read_files_in_dir_sorted;
use crate::photo::Photo;
use std::io::Result;
use std::path::Path;

fn main() {
    let arguments = configuration::get();

    process_directory(&arguments.source, &arguments.target).unwrap_or_else(|e| println!("Unexpected error: {}", e.to_string()));
}

fn process_directory(source: &Path, target: &Path) -> Result<()> {
    println!("Processing source directory {}", source.display().to_string());

    for path in read_files_in_dir_sorted(source)? {
        println!("Processing {:?}...", path);

        let photo = Photo::new(&path)?;
        photo.copy_to(target)?;
    }
    return Ok(());
}
