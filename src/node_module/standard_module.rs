use std::io::Error;
use std::path::PathBuf;
use std::cmp::Ordering;

use regex::Regex;

use crate::pjson_detail::PjsonDetail;
use crate::cli::Cli;
use crate::node_module::*;

#[derive(Debug)]
pub struct StandardModule {
    pub name: String,
    pub version: String,
    pub dep_type: DepType,
}

impl NodeModule for StandardModule {
    fn populate(&mut self, path: &PathBuf, _cli: &Cli, app_pjson: Option<&PjsonDetail>) -> Result<(), Error> {

        let PjsonDetail { name, version, .. } = PjsonDetail::from(path)?;

        self.dep_type = get_dep_type(&name, app_pjson.unwrap());

        self.name = name;
        self.version = version;

        Ok(())
    }

    fn filter_by_regex(&self, re: &Regex) -> bool {
        re.is_match(&self.name)
    }

    fn filter_by_args(&self, cli: &Cli) -> bool {
        standard_filter(&self.dep_type, &cli)
    }

    fn order(&self, to_compare: &StandardModule) -> Ordering {
        self.name.cmp(&to_compare.name)
    }
}

impl PrintTable for StandardModule {
    fn table_row(&self, _row_type: RowType) -> Row {
        Row::new(vec![
            get_name_cell(&self.name, &self.dep_type),
            Cell::new(&self.version),
        ])
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
