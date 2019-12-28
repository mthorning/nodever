mod print;
mod types;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use types::application_detail::{AppDetail, Args};
use types::cli::Cli;

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();
    let primary_args = Args {
        filter: cli.filter,
        path: cli.path,
        dont_sort: cli.dont_sort,
        direct_dep: cli.direct_dep,
    };

    if let Some(diff_path) = cli.diff {
        let app_one_details = AppDetail::new(primary_args)?;
        let app_two_details = AppDetail::new(Args {
            path: diff_path,
            ..primary_args
        })?;
        print::print_details(&app_two_details)?;
    } else {
        let app_details = AppDetail::new(primary_args)?;
        print::print_details(&app_details)?;
    }

    Ok(())
}
