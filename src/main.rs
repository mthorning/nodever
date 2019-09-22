mod types;
mod functions;

use types::cli::Cli;
use exitfailure::ExitFailure;
use regex::Regex;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let mut path = args.path;
    let filter = Regex::new(&args.filter)?;
    let sort = args.sort;

    let app_details = functions::get_dependency_details(&path)?;

    path.push("node_modules");

    let mut details = Vec::new();
    functions::get_dependencies(&mut details, &path, &filter);

    //let details = functions::get_dependency_details(&path, dependency_folders);
    
    if sort {
        details.sort_by(|a, b| a.name.cmp(&b.name));
    }
    functions::print_details(app_details, details)?;

    Ok(())
}
