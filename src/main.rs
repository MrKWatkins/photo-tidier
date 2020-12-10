mod configuration;
mod photos;

use std::fs;
use std::io;
use std::path::Path;
use crate::photos::Photo;

fn main()
{
    let arguments = configuration::get();

    output_directory(arguments.source.as_path())
        .unwrap_or_else(|e| println!("Unexpected error: {}", e.to_string()));
}

fn output_directory(directory: &Path) -> io::Result<()>
{
    println!("Directory: {}", directory.display().to_string());

    for entry in fs::read_dir(directory)?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
        {
            let photo = Photo::new(&path);

            println!("{}", photo.get_path_string());
            photo.print_exif();
        }

        break;
    }
    Ok(())
}