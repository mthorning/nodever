pub mod diffed_pair;
pub mod global;
pub mod standard;
pub mod required_by;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;

use prettytable::{color, row, Attr, Cell, Row, Table};
use regex::Regex;

use crate::cli::Cli;
use crate::pjson_detail::PjsonDetail;
use crate::semver::Semver;

#[derive(Clone)]
pub enum DepType {
    Dependency(Option<Semver>),
    DevDependency(Option<Semver>),
    ChildDependency,
}

pub trait NodeModule {
    fn populate(
        &mut self,
        base_path: &PathBuf,
        app_pjson: Option<&PjsonDetail>,
    ) -> Result<(), Error>;

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
    fn add_to_table(&self, table: &mut Table);
}

pub fn get_dep_type(name: &str, app_pjson: &PjsonDetail) -> DepType {
    match get_pjson_details(name, &app_pjson.dependencies) {
        Some(required_version) => return DepType::Dependency(required_version),
        None => match get_pjson_details(name, &app_pjson.dev_dependencies) {
            Some(required_version) => return DepType::DevDependency(required_version),
            None => DepType::ChildDependency,
        },
    }
}

pub fn get_pjson_details(
    dep_name: &str,
    required_dependencies: &Option<HashMap<String, String>>,
) -> Option<Option<Semver>> {
    match required_dependencies {
        Some(deps) => match deps.get(dep_name) {
            Some(required_version) => Some(Semver::from(required_version.to_string())),
            None => None,
        },
        None => None,
    }
}

pub fn new_cell(value: &str) -> Cell {
    let mut cell = Cell::new(value);
    cell.align(prettytable::format::Alignment::CENTER);
    cell
}

pub fn get_pjson_version_cell(dep_type: &DepType) -> Cell {
    match dep_type {
        DepType::ChildDependency => new_cell(""),
        DepType::Dependency(pjson_version) => {
            let cell_contents = match &pjson_version {
                Some(version) => version.to_string(),
                None => String::from("???"),
            };
            new_cell(&cell_contents)
                .with_style(Attr::BackgroundColor(color::BLUE))
                .with_style(Attr::ForegroundColor(color::BLACK))
        }
        DepType::DevDependency(pjson_version) => {
            let cell_contents = match &pjson_version {
                Some(version) => version.to_string(),
                None => String::from("???"),
            };
            new_cell(&cell_contents)
                .with_style(Attr::BackgroundColor(color::MAGENTA))
                .with_style(Attr::ForegroundColor(color::BLACK))
        }
    }
}

pub fn standard_filter(dep_type: &DepType) -> bool {
    let cli = Cli::get();
    match dep_type {
        DepType::ChildDependency => {
            if cli.dev || cli.dep {
                return false;
            }
        }
        DepType::Dependency(_) => {
            if cli.dev && !cli.dep {
                return false;
            }
        }
        DepType::DevDependency(_) => {
            if cli.dep && !cli.dev {
                return false;
            }
        }
    }
    true
}
