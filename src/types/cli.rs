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

    /// Show only direct dependencies.
    #[structopt(long = "direct-only", short = "d")]
    pub direct_deps: bool,

    #[structopt(long)]
    pub diff: Option<PathBuf>,

    /// Search in global dependencies
    #[structopt(long, short = "g")]
    pub global: bool,
}
