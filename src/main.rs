mod types;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use types::application_detail::AppDetail;
use types::cli::Cli;

fn main() -> Result<(), ExitFailure> {
    //let path = cli.path;
    //let dependencies = cli.dependencies;
    //let regex = Regex::new(&cli.filter)?;
    //let _sort = cli.sort;

    let app_details = AppDetail::new(Cli::from_args())?;
    app_details.print()?;

    Ok(())
}
