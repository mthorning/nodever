use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The pattern to filter folders in node_modules.
    #[structopt(default_value = ".*")]
    pub filter: String,

    /// The path to the root of the Nexcenter app.
    #[structopt(long, parse(from_os_str), default_value = ".")]
    pub path: PathBuf,

    /// Prevent alphabetically sorting of results.
    #[structopt(long = "dont-sort", short = "x")]
    pub dont_sort: bool,

    /// Show only direct dependencies.
    #[structopt(long = "direct-only", short = "d")]
    pub direct_dep: bool,
}

//#[derive(StructOpt)]
//pub enum Compare {
//  #[structopt(parse(from_os_str), default_value = ".")]
//  pub dir_one,
//
//  #[structopt(parse(from_os_str), default_value = ".")]
//  pub dir_two
//
//}
