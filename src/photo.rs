use crate::file::{validate_directory_exists, validate_file_exists};
use chrono::NaiveDateTime;
use exif::{Exif, In, Tag};
use once_cell::unsync::OnceCell;
use std::ffi::OsStr;
use std::fs::{copy, File};
use std::io::{BufReader, Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

pub struct Photo {
    path: PathBuf,
    exif: OnceCell<Option<Exif>>,
}

impl Photo {
    pub fn new(file: &Path) -> Result<Photo> {
        validate_file_exists(file)?;

        if file.extension().is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, format!("The path {:?} does not have an extension.", file)));
        }

        return Ok(Photo {
            path: file.to_path_buf(),
            exif: OnceCell::new(),
        });
    }

    pub fn get_current_path(&self) -> &Path {
        return self.path.as_path();
    }

    #[allow(dead_code)]
    pub fn get_current_file_name(&self) -> &Path {
        return Path::new(self.path.file_name().unwrap());
    }

    #[allow(dead_code)]
    pub fn get_current_extension(&self) -> &str {
        return get_string(self.path.extension());
    }

    pub fn get_new_file_name(&self) -> PathBuf {
        let file_stem = match self.get_timestamp() {
            Some(date_time) => date_time.format("%Y-%m-%d_%H-%M-%S").to_string(),
            None => get_string(self.path.file_stem()).to_string(),
        };

        let extension = get_string(self.path.extension()).to_ascii_lowercase();

        let mut result = PathBuf::new();
        result.push(file_stem);
        result.set_extension(extension);
        return result;
    }

    pub fn get_timestamp(&self) -> Option<NaiveDateTime> {
        let exif = self.get_exif().as_ref();

        let field = exif.map(|e| e.get_field(Tag::DateTime, In::PRIMARY)).flatten();

        return field.map(|f| NaiveDateTime::parse_from_str(&f.display_value().to_string(), "%Y-%m-%d %H:%M:%S").unwrap());
    }

    fn get_exif(&self) -> &Option<Exif> {
        return self.exif.get_or_init(|| read_exif(&self.path));
    }

    #[allow(dead_code)]
    pub fn print_exif(&self) {
        let exif = self.get_exif().as_ref().unwrap();

        let mut fields: Vec<_> = exif.fields().collect();
        fields.sort_by(|x, y| x.tag.to_string().cmp(&y.tag.to_string()));

        for f in &fields {
            println!("{} ({}): {}", f.tag, f.ifd_num, f.display_value().with_unit(exif));
        }
    }

    pub fn copy_to(&self, target_directory: &Path) -> Result<()> {
        validate_directory_exists(target_directory)?;

        let current_path = self.get_current_path();
        let new_path = self.get_new_path(target_directory);
        println!("Copying {:?} to {:?}...", current_path, new_path);

        copy(current_path, new_path)?;

        return Ok(());
    }

    fn get_new_path(&self, target_directory: &Path) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(target_directory);
        path.push(self.get_new_file_name());
        return path;
    }
}

fn read_exif(path: &Path) -> Option<Exif> {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    return exif_reader.read_from_container(&mut buf_reader).ok();
}

fn get_string(os_string: Option<&OsStr>) -> &str {
    return os_string.unwrap().to_str().unwrap();
}
