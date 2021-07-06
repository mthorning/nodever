pub mod standard_module;
pub mod global_module;
pub mod diffed_pair;

use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use std::cmp::Ordering;

use regex::Regex;
use prettytable::{row, Row, Cell, Attr, color};

use crate::pjson_detail::PjsonDetail;
use crate::cli::Cli;
use crate::semver::Semver;

#[derive(Debug, Clone, PartialEq)]
pub enum DepType {
    Dependency(Semver),
    DevDependency(Semver),
    ChildDependency,
}

pub trait NodeModule {
    fn populate(&mut self, base_path: &PathBuf, app_pjson: Option<&PjsonDetail>) -> Result<(), Error>;

    fn filter_by_regex(&self, _re: &Regex) -> bool {
        true
    }

    fn order(&self, _to_compare: &Self) -> Ordering {
        Ordering::Equal
    }
    
    fn filter_by_args(&self) -> bool {
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
) -> Option<Semver> {

    match required_dependencies {
        Some(deps) => {
            match deps.get(dep_name) {
                Some(required_version) => Some(Semver::from(required_version.to_string())),
                None => None,
            }
        },
        None => None,
    }
}

pub fn new_cell(value: &str) -> Cell {
    let mut cell = Cell::new(value);
    cell.align(prettytable::format::Alignment::CENTER);
    cell
}

pub fn get_pjson_version_cell(pjson_version: &Option<Semver>, dep_type: &DepType) -> Cell {
        let cell = new_cell(pjson_version.as_ref().map_or("", |version| &version.to_string()));
        match dep_type {
            DepType::ChildDependency => cell,
            DepType::Dependency(_) => cell
                .with_style(Attr::BackgroundColor(color::BLUE))
                .with_style(Attr::ForegroundColor(color::BLACK)),
            DepType::DevDependency(_) => cell
                .with_style(Attr::BackgroundColor(color::MAGENTA))
                .with_style(Attr::ForegroundColor(color::BLACK)),
        }
}

pub fn standard_filter(dep_type: &DepType) -> bool {
    let cli = Cli::get();
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
