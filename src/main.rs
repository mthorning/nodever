mod print;
mod types;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use types::application_detail::{AppDetail, Args};
use types::cli::Cli;
use types::output_schema::{Schema, Schematic};

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();
    let primary_args = Args {
        filter: cli.filter.clone(),
        path: cli.path.clone(),
        dont_sort: cli.dont_sort,
        direct_dep: cli.direct_dep,
    };
    let app_details_one = AppDetail::new(primary_args)?;

    if let Some(diff_path) = cli.diff {
        let secondary_args = Args {
            filter: cli.filter,
            path: diff_path,
            dont_sort: cli.dont_sort,
            direct_dep: cli.direct_dep,
        };
        let app_details_two = AppDetail::new(secondary_args)?;
    //print::print_diff(&app_details_one, &app_details_two)?;
    } else {
        match cli.direct_dep {
            true => print::print_details(Schema::new(Schematic::Direct(&app_details_one)))?,
            false => print::print_details(Schema::new(Schematic::Plain(&app_details_one)))?,
        }
    };

    Ok(())
}
