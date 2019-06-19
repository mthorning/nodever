use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The path to the root of the Nexcenter app.
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn node_modules_exists(path: &PathBuf) -> bool {
    for entry in path.read_dir().expect("Unable to read Nexcenter directory") {
        if let Ok(entry) = entry {
            if entry.file_name() == "node_modules" {
                return true;
            }
        }
    }
    false
}

fn get_nex_deps(path: &PathBuf) -> Result<Vec<PathBuf>, &'static str> {
    //check if there is node_modules
    if !node_modules_exists(path) {
        return Err("No node_modules present in this directory");
    }

    //find all nexcenter folders
    let deps = Vec::new();
    Ok(deps)
}

fn main() -> Result<(), ExitFailure> {
    // accept path from args
    let args = Cli::from_args();

    //get data from package.json in each folder

    //print the result
    Ok(())
}
