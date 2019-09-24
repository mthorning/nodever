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
    #[structopt(short = "D", long)]
    pub dependencies: bool,
}
