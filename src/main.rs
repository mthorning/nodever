mod functions;
mod types;

use exitfailure::ExitFailure;
use regex::Regex;
use structopt::StructOpt;
use types::cli::Cli;
use types::detail::application_detail::AppDetail;

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let mut path = args.path;
    let dependencies = args.dependencies;
    let filter = Regex::new(if dependencies { ".*" } else { &args.filter })?;
    let sort = args.sort;

    let app_details = AppDetail::new(&path)?;
    println!("{:?}", app_details);

    path.push("node_modules");

    let mut details = Vec::new();

    //functions::get_dependencies(&mut details, &path, &filter)?;

    // Build the Vec then filter later
    functions::get_dependencies(&mut details, &path, &app_details)?;

    if sort {
        details.sort_by(|a, b| a.name.cmp(&b.name));
    }
    //functions::print_details(app_details, details)?;

    Ok(())
}
