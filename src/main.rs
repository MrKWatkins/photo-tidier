mod configuration;

use std::fs;
use std::io;
use std::path::Path;

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
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str())
            {
                println!("{}", file_name);
            }
        }
    }
    Ok(())
}