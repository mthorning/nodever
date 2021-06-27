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

use node_module::NodeModule;
use node_module::standard_module::StandardModule;
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

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::from_args();

    let path = match cli.global {
        true => get_global_path(),
        false => cli.path.clone(),
    };
    let mut base_path = PathBuf::from(path);
    base_path.push("node_modules");

    let app_pjson = PjsonDetail::new(&cli.path)?;
    let mut dependencies: Vec<StandardModule> = Vec::new();
    collect_dependencies(&base_path, &app_pjson, &cli, &mut dependencies)?;

    for dependency in dependencies {
        println!("{}", dependency.print());
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


fn collect_dependencies<T: NodeModule + Default>(base_path: &PathBuf, pjson: &PjsonDetail, cli: &Cli, dependencies: &mut Vec<T>) -> Result<(), Error> {
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
                return collect_dependencies(&dep_path, pjson, cli, dependencies);
            } else {
                let mut detail: T = Default::default();
                detail.populate(&dep_path, pjson, cli)?;
                if detail.filter_by_regex(&filter_re) && detail.filter_by_args(&cli) {
                    dependencies.push(detail)
                }
            }
        }
    }

    // self.dependency_details.sort_by(|a, b| a.get_comparison_field().cmp(&b.get_comparison_field()));

    Ok(())
}
