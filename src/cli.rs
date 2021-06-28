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

    /// Show only direct dependencies.
    #[structopt(long = "direct-only", short = "d")]
    pub direct_deps: bool,

    /// Show only dependencies.
    #[structopt(long)]
    pub dep: bool,

    /// Show only devDependencies.
    #[structopt(long)]
    pub dev: bool,

    /// Show only peerDependencies.
    #[structopt(long)]
    pub peer: bool,
}
