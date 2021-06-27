pub mod standard_module;

use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use regex::Regex;
use crate::pjson_detail::PjsonDetail;
use crate::cli::Cli;

#[derive(Debug, Clone, PartialEq)]
pub enum DepType {
    Dependency(String),
    DevDependency(String),
    PeerDependency(String),
    ChildDependency,
}

pub trait NodeModule {
    fn filter_by_regex(&self, re: &Regex) -> bool;
    fn filter_by_args(&self, cli: &Cli) -> bool;
    fn print(&self) -> String;
    fn populate(&mut self, base_path: &PathBuf, app_pjson: &PjsonDetail, cli: &Cli) -> Result<(), Error>;
}



pub fn get_dep_type(name: &str, app_pjson: &PjsonDetail) -> DepType {
    match get_pjson_details(name, &app_pjson.dependencies) {
        Some(required_version) => return DepType::Dependency(required_version),
        None => match get_pjson_details(name, &app_pjson.dev_dependencies) {
            Some(required_version) => return DepType::DevDependency(required_version),
            None => match get_pjson_details(name, &app_pjson.peer_dependencies) {
                Some(required_version) => return DepType::PeerDependency(required_version),
                None => DepType::ChildDependency
            }
        }
    }
}

pub fn get_pjson_details(
    dep_name: &str,
    required_dependencies: &Option<HashMap<String, String>>,
) -> Option<String> {
    match required_dependencies {
        Some(deps) => match deps.get(dep_name) {
            Some(required_version) => Some(required_version.to_string()),
            None => None,
        },
        None => None,
    }
}
