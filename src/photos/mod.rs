use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use exif::Exif;

pub struct Photo
{
    path: PathBuf,
    exif: RefCell<Option<Exif>>
}

impl Photo
{
    pub fn new(path: PathBuf) -> Photo
    {
        if path.is_file()
        {
            return Photo { path, exif: RefCell::new(None) };
        }

        panic!("path is not a file.");
    }

    pub fn get_path_string(&self) -> &str
    {
        return self.path.file_name().unwrap().to_str().unwrap();
    }

    pub fn print_exif(&self)
    {
        self.ensure_exif();

        let borrow = self.exif.borrow();
        let exif = borrow.as_ref().unwrap();

        let mut fields : Vec<_> = exif.fields().collect();
        fields.sort_by(|x, y| x.tag.to_string().cmp(&y.tag.to_string()));

        for f in &fields
        {
            println!("{} ({}): {}", f.tag, f.ifd_num, f.display_value().with_unit(exif));
        }
    }

    fn ensure_exif(&self)
    {
        // Based on https://doc.rust-lang.org/std/cell/index.html#implementation-details-of-logically-immutable-methods
        self.exif
            .borrow_mut()
            .get_or_insert_with(|| self.create_exif());
    }

    fn create_exif(&self) -> Exif
    {
        let file = File::open(&self.path).unwrap();
        let mut buf_reader = BufReader::new(&file);
        let exif_reader = exif::Reader::new();
        return exif_reader.read_from_container(&mut buf_reader).unwrap();
    }
}