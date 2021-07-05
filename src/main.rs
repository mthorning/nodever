#[macro_use] extern crate prettytable;
mod node_module;
mod cli;
mod pjson_detail;
mod semver;

use std::default::Default;
use std::io::{self, Error, Write};
use std::path::PathBuf;

use which::which;
use exitfailure::ExitFailure;
use regex::Regex;
use prettytable::Table;

use node_module::{NodeModule, PrintTable};
use node_module::standard_module::StandardModule;
use node_module::global_module::GlobalModule;
use node_module::diffed_pair::DiffedPair;
use pjson_detail::PjsonDetail;
use cli::Cli;

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::get();


    if cli.global {
        let base_path = get_node_modules_path(&get_global_path());
        let mut dependencies = Vec::<GlobalModule>::new();
        collect_dependencies(&base_path, &mut dependencies, None)?;
        print_table(&dependencies);
    } else {
        let base_path = get_node_modules_path(&cli.path);
        let app_pjson = PjsonDetail::from(&cli.path)?;
            let mut dependencies = Vec::<StandardModule>::new();
            collect_dependencies(&base_path, &mut dependencies, Some(&app_pjson))?;
        
        if let Some(path) = &cli.diff {
            let diff_path = get_node_modules_path(&path);
            let diff_pjson = PjsonDetail::from(&path)?;
            let mut diff_dependencies = Vec::<StandardModule>::new();
            collect_dependencies(&diff_path, &mut diff_dependencies, Some(&diff_pjson))?;
            let diffed_pairs = DiffedPair::get_pairs(&dependencies, &diff_dependencies);
            print_table(&diffed_pairs);
        } else {
            print_table(&dependencies);
            print_completion_message(format!(
                "\n{} matches found in version {} of {}.\n",
                dependencies.len(),
                app_pjson.version,
                app_pjson.name,
            ))?;
        }
    }

    Ok(())
}

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


fn collect_dependencies<T: NodeModule + Default>(base_path: &PathBuf, dependencies: &mut Vec<T>, app_pjson: Option<&PjsonDetail>) -> Result<(), Error> {
    let node_modules = base_path.read_dir()?;
    let cli = Cli::get();

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
                collect_dependencies(&dep_path, dependencies, app_pjson)?;
            } else {
                let mut detail: T = Default::default();
                detail.populate(&dep_path, app_pjson)?;
                if detail.filter_by_regex(&filter_re) && detail.filter_by_args() {
                    dependencies.push(detail)
                }
            }
        }
    }

    dependencies.sort_by(|a, b| a.order(&b));

    Ok(())
}

fn print_table<T: PrintTable>(dependencies: &Vec<T>) {
    let mut table = Table::new();
    if dependencies.len() == 0 { return; }

    for dependency in dependencies {
        table.add_row(dependency.table_row());
    }
    table.printstd();
}

fn print_completion_message(message: String) -> Result<(), Error> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(message.as_bytes())?;
    Ok(())
}
