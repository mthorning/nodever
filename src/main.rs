mod print;
mod types;

use exitfailure::ExitFailure;
use structopt::StructOpt;
use types::application_detail::{AppDetail, Args};
use types::cli::Cli;
use types::output_schema::{Schema, Schematic};
use std::path::PathBuf;
use which::which;

fn get_global_path() -> PathBuf {
    let mut node_path = which("node").unwrap();
    node_path.pop();
    node_path.pop();
    node_path.push("lib");
    node_path
}

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();

    let path = match cli.global {
        true => get_global_path(),
        false => cli.path.clone(),
    };
    let app_args = Args {
        path,
        filter: cli.filter,
        global: cli.global,
        direct_deps: cli.direct_deps,
    };
    let app_details = AppDetail::new(app_args)?;

    if let Some(diff_path) = cli.diff {
        let diff_app_args = Args {
            path: diff_path,
            global: false,
            filter: cli.filter,
            direct_deps: cli.direct_deps,
        };
        let diff_app_details = AppDetail::new(diff_app_args)?;
        print::print_details(Schema::new(Schematic::Diff(
            &app_details,
            &diff_app_details,
        )))?;
    } else {
        match cli.direct_deps {
            true => print::print_details(Schema::new(Schematic::Direct(&app_details)))?,
            false => print::print_details(Schema::new(Schematic::Plain(&app_details)))?,
        }
    };

    Ok(())
}
