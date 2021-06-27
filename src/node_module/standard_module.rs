use std::io::Error;
use std::path::PathBuf;
use std::cmp::Ordering;

use regex::Regex;
use crate::pjson_detail::PjsonDetail;
use crate::cli::Cli;
use crate::node_module::*;

pub struct StandardModule {
    pub name: String,
    pub version: String,
    pub dep_type: DepType,
}

impl NodeModule for StandardModule {
    fn filter_by_regex(&self, re: &Regex) -> bool {
        re.is_match(&self.name)
    }

    fn filter_by_args(&self, cli: &Cli) -> bool {
       if cli.direct_deps && self.dep_type == DepType::ChildDependency {
           return false
        } 
        true
    }

    fn print(&self) -> String {
        format!("{} = {}", self.name, self.version)
    }

    fn populate(&mut self, path: &PathBuf, _cli: &Cli, app_pjson: Option<&PjsonDetail>) -> Result<(), Error> {

        let PjsonDetail { name, version, .. } = PjsonDetail::new(path)?;

        self.dep_type = get_dep_type(&name, app_pjson.unwrap());

        self.name = name;
        self.version = version;

        Ok(())
    }

    fn order(&self, to_compare: &StandardModule) -> Ordering {
        self.name.cmp(&to_compare.name)
    }
}

impl Default for StandardModule {
    fn default() -> Self {
        StandardModule {
            name: String::new(),
            version: String::new(),
            dep_type: DepType::ChildDependency, 
        }
    }
}
