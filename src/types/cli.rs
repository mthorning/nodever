use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The pattern to filter folders in node_modules.
    #[structopt(default_value = ".*")]
    pub filter: String,

    /// The path to the root of the Nexcenter app.
    #[structopt(short, long, parse(from_os_str), default_value = ".")]
    pub path: PathBuf,

    /// Sort the results alphabetically
    #[structopt(long, short = "x")]
    pub dont_sort: bool,

    /// Show only direct dependencies
    #[structopt(long = "dependencies-only", short = "S")]
    pub direct_dep: bool,

    /// Show only direct devDependencies
    #[structopt(long = "devDependencies-only", short = "D")]
    pub direct_dev: bool,

    /// Show only direct peerDependencies
    #[structopt(long = "peerDependencies-only", short = "P")]
    pub direct_peer: bool,
}
