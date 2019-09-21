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

    let app_details = functions::get_app_details(&path)?;

    path.push("node_modules");

    let dependency_folders = functions::get_dependency_folders(&path, filter);

    if let Ok(dependency_folders) = dependency_folders {
        let details = functions::get_dependency_details(&path, dependency_folders);
        
        if let Ok(mut details) = details {
            if sort {
                details.sort_by(|a, b| a.name.cmp(&b.name));
            }
            functions::print_details(app_details, details)?;
        }
    }

    Ok(())
}
