mod print;
mod types;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use types::application_detail::{AppDetail, Args};
use types::cli::Cli;
use types::output_schema::{Schema, Schematic};

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();
    let app_args = Args {
        filter: cli.filter.clone(),
        path: cli.path.clone(),
        dont_sort: cli.dont_sort,
        direct_dep: cli.direct_dep,
    };
    let app_details = AppDetail::new(app_args)?;

    if let Some(diff_path) = cli.diff {
        let diff_app_args = Args {
            path: diff_path,
            dont_sort: false,
            filter: cli.filter,
            direct_dep: cli.direct_dep,
        };
        let diff_app_details = AppDetail::new(diff_app_args)?;
        print::print_details(Schema::new(Schematic::Diff(
            &app_details,
            &diff_app_details,
        )))?;
    } else {
        match cli.direct_dep {
            true => print::print_details(Schema::new(Schematic::Direct(&app_details)))?,
            false => print::print_details(Schema::new(Schematic::Plain(&app_details)))?,
        }
    };

    Ok(())
}
