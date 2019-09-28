mod types;

use exitfailure::ExitFailure;
use regex::Regex;
use structopt::StructOpt;
use types::application_detail::AppDetail;
use types::cli::Cli;

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let path = args.path;
    let dependencies = args.dependencies;
    let _filter = Regex::new(if dependencies { ".*" } else { &args.filter })?;
    let _sort = args.sort;

    let app_details = AppDetail::new(path)?;
    app_details.print()?;

    Ok(())
}
