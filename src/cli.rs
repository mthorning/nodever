use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// The pattern to filter folders in node_modules.
    #[structopt(default_value = ".*")]
    pub filter: String,

    /// The path to node_modules folder.
    #[structopt(long, parse(from_os_str), default_value = ".")]
    pub path: PathBuf,

    #[structopt(long)]
    pub diff: Option<PathBuf>,

    /// Search in global dependencies.
    #[structopt(long, short = "g")]
    pub global: bool,

    /// Show dependencies.
    #[structopt(short = "D" )] 
    pub dep: bool,

    /// Show devDependencies.
    #[structopt(short = "d")]
    pub dev: bool,
}
