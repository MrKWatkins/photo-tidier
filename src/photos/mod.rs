use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use exif::Exif;
use once_cell::unsync::OnceCell;

pub struct Photo
{
    path: PathBuf,
    exif: OnceCell<Exif>
}

impl Photo
{
    pub fn new(path: &PathBuf) -> Photo
    {
        if path.is_file()
        {
            return Photo { path: path.clone(), exif: OnceCell::new() };
        }

        panic!("path is not a file.");
    }

    pub fn get_path_string(&self) -> &str
    {
        return self.path.file_name().unwrap().to_str().unwrap();
    }

    fn get_exif(&self) -> &Exif
    {
        return self.exif.get_or_init(|| read_exif(&self.path));
    }

    pub fn print_exif(&self)
    {
        let exif = self.get_exif();

        let mut fields : Vec<_> = exif.fields().collect();
        fields.sort_by(|x, y| x.tag.to_string().cmp(&y.tag.to_string()));

        for f in &fields
        {
            println!("{} ({}): {}", f.tag, f.ifd_num, f.display_value().with_unit(exif));
        }
    }
}

fn read_exif(path: &PathBuf) -> Exif
{
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    return exif_reader.read_from_container(&mut buf_reader).unwrap();
}