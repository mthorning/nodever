pub mod standard_module;
pub mod global_module;
pub mod diffed_pair;

use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use std::cmp::Ordering;

use regex::Regex;
use prettytable::{row, Row, Cell};

use crate::pjson_detail::PjsonDetail;
use crate::cli::Cli;

#[derive(Debug, Clone, PartialEq)]
pub enum DepType {
    Dependency(String),
    DevDependency(String),
    ChildDependency,
}

pub trait NodeModule {
    fn populate(&mut self, base_path: &PathBuf, cli: &Cli, app_pjson: Option<&PjsonDetail>) -> Result<(), Error>;

    fn filter_by_regex(&self, _re: &Regex) -> bool {
        true
    }

    fn order(&self, _to_compare: &Self) -> Ordering {
        Ordering::Equal
    }
    
    fn filter_by_args(&self, _cli: &Cli) -> bool {
        true
    }
}

pub trait PrintTable {
    fn table_row(&self) -> Row {
        row![]
    }
}



pub fn get_dep_type(name: &str, app_pjson: &PjsonDetail) -> DepType {
    match get_pjson_details(name, &app_pjson.dependencies) {
        Some(required_version) => return DepType::Dependency(required_version),
        None => match get_pjson_details(name, &app_pjson.dev_dependencies) {
            Some(required_version) => return DepType::DevDependency(required_version),
            None => DepType::ChildDependency
        }
    }
}

pub fn get_pjson_details(
    dep_name: &str,
    required_dependencies: &Option<HashMap<String, String>>,
) -> Option<String> {

    match required_dependencies {
        Some(deps) => {
            match deps.get(dep_name) {
                Some(required_version) => Some(required_version.to_string()),
                None => None,
            }
        },
        None => None,
    }
}

pub fn new_cell(value: &str) -> Cell {
    let txt = match value {
        "" => "-",
        _ => value,
    };
    let mut cell = Cell::new(txt);
    cell.align(prettytable::format::Alignment::CENTER);
    cell
}

pub fn get_pjson_version_cell(pjson_version: &Option<String>, dep_type: &DepType) -> Cell {
        let cell = new_cell(pjson_version.as_ref().map_or("", |version| &version));
        match dep_type {
            DepType::ChildDependency => cell,
            DepType::Dependency(_) => cell.style_spec("BbFd"),
            DepType::DevDependency(_) => cell.style_spec("BmFd"),
        }
}

pub fn standard_filter(dep_type: &DepType, cli: &Cli) -> bool {
        match dep_type {
            DepType::ChildDependency => {
                if cli.dev ||  cli.dep { return false; }
            }
            DepType::Dependency(_) => {
                if cli.dev && !cli.dep { return false; }
            }
            DepType::DevDependency(_) => {
                if cli.dep && ! cli.dev { return false; }
            }
        }
        true
}
