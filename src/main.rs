#[macro_use] extern crate prettytable;
mod print;
mod node_module;
mod cli;
mod pjson_detail;

use std::default::Default;
use std::io::Error;
use std::path::PathBuf;

use which::which;
use structopt::StructOpt;
use exitfailure::ExitFailure;
use regex::Regex;
use prettytable::Table;

use node_module::NodeModule;
use node_module::standard_module::StandardModule;
use node_module::global_module::GlobalModule;
use pjson_detail::PjsonDetail;
use cli::Cli;
// use types::output_schema::{Schema, Schematic};

fn get_global_path() -> PathBuf {
    let mut node_path = which("node").unwrap();
    node_path.pop();
    node_path.pop();
    node_path.push("lib");
    node_path
}

fn get_node_modules_path(path: &PathBuf) -> PathBuf {
        let mut base_path = PathBuf::from(path);
        base_path.push("node_modules");
        base_path
}

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();

    if cli.global {
        let base_path = get_node_modules_path(&get_global_path());
        let mut dependencies = Vec::<GlobalModule>::new();
        collect_dependencies(&base_path, &cli, &mut dependencies, None)?;
        print_table(dependencies);
    } else {
        let base_path = get_node_modules_path(&cli.path);
        let app_pjson = PjsonDetail::new(&cli.path)?;
        let mut dependencies = Vec::<StandardModule>::new();
        collect_dependencies(&base_path, &cli, &mut dependencies, Some(&app_pjson))?;
        print_table(dependencies);
    }

    // if let Some(diff_path) = cli.diff {
    //     let diff_app_args = Args {
    //         path: diff_path,
    //         global: false,
    //         filter: cli.filter,
    //         direct_deps: cli.direct_deps,
    //     };
    //     let diff_app_details = AppDetail::new(diff_app_args)?;
    //     print::print_details(Schema::new(Schematic::Diff(
    //         &app_details,
    //         &diff_app_details,
    //     )))?;
    // } else {
    //     match cli.direct_deps {
    //         true => print::print_details(Schema::new(Schematic::Direct(&app_details)))?,
    //         false => print::print_details(Schema::new(Schematic::Plain(&app_details)))?,
    //     }
    // };

    Ok(())
}

fn collect_dependencies<T: NodeModule + Default>(base_path: &PathBuf,  cli: &Cli, dependencies: &mut Vec<T>, app_pjson: Option<&PjsonDetail>) -> Result<(), Error> {
    let node_modules = base_path.read_dir()?;

    let filter_re = Regex::new(&cli.filter).unwrap();

    for entry in node_modules {
        if let Ok(entry) = entry {
            let folder_name = entry.file_name().into_string().unwrap();

            if folder_name.starts_with('.') {
                continue;
            }
            let mut dep_path = base_path.clone();
            dep_path.push(&folder_name);

            if folder_name.starts_with('@') {
                collect_dependencies(&dep_path, cli, dependencies, app_pjson)?;
            } else {
                let mut detail: T = Default::default();
                detail.populate(&dep_path, cli, app_pjson)?;
                if detail.filter_by_regex(&filter_re) && detail.filter_by_args(&cli) {
                    dependencies.push(detail)
                }
            }
        }
    }

    dependencies.sort_by(|a, b| a.order(&b));

    Ok(())
}

fn print_table<T: NodeModule>(dependencies: Vec<T>) {
    let mut table = Table::new();
    for dependency in dependencies {
        table.add_row(dependency.table_row());
    }
    table.printstd();
}

fn print_dependencies<T: NodeModule>(dependencies: Vec<T>) {
    for dependency in dependencies {
        println!("{:?}", dependency.print());
    }
}
