use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The path to the root of the Nexcenter app.
    #[structopt(short, long, parse(from_os_str), default_value = ".")]
    pub path: PathBuf,

    /// The pattern to filter folders in node_modules.
    #[structopt(short, long, default_value = ".*")]
    pub filter: String,

    /// Sort the results alphabetically
    #[structopt(short, long)]
    pub sort: bool,

    /// Show only direct dependencies
    #[structopt(long = "dependencies-only")]
    pub direct_dep: bool,

    /// Show only direct devDependencies
    #[structopt(long = "devDependencies-only")]
    pub direct_dev: bool,

    /// Show only direct peerDependencies
    #[structopt(long = "peerDependencies-only")]
    pub direct_peer: bool,
}
