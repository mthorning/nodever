mod functions;
mod types;

use exitfailure::ExitFailure;
use regex::Regex;
use structopt::StructOpt;
use types::cli::Cli;

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let mut path = args.path;
    let filter = Regex::new(&args.filter)?;
    let dependencies = args.dependencies;
    let sort = args.sort;

    let app_details = functions::get_dependency_details(&path)?;
    println!("{:?}", app_details);

    path.push("node_modules");

    let mut details = Vec::new();
    functions::get_dependencies(&mut details, &path, &filter)?;

    //if dependencies {
    //    details = details
    //        .into_iter()
    //        .filter(|detail| detail.name.starts_with('a'))
    //        .collect();
    //}

    if sort {
        details.sort_by(|a, b| a.name.cmp(&b.name));
    }
    //functions::print_details(app_details, details)?;

    Ok(())
}
