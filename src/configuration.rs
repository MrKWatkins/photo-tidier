use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "photo-tidier", about = "A simple application to tidy up photos.")]
pub struct CommandLineArguments {
    /// The source folder.
    #[structopt(parse(from_os_str))]
    pub source: PathBuf,

    /// The target folder.
    #[structopt(parse(from_os_str))]
    pub target: PathBuf,
}

pub fn get() -> CommandLineArguments {
    return CommandLineArguments::from_args();
}
