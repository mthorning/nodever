mod print;
mod types;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use types::application_detail::AppDetail;
use types::cli::Cli;

fn main() -> Result<(), ExitFailure> {
    let app_details = AppDetail::new(Cli::from_args())?;
    print::print_details(app_details)?;

    Ok(())
}
