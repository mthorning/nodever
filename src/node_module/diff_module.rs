use std::io::Error;
use std::path::PathBuf;
use std::cmp::Ordering;

use regex::Regex;

use crate::pjson_detail::PjsonDetail;
use crate::cli::Cli;
use crate::node_module::*;

#[derive(Debug)]
pub struct DiffModule {
    pub name: String,
    pub version: String,
    pub dep_type: DepType,
}

impl NodeModule for DiffModule {
    fn populate(&mut self, path: &PathBuf, _cli: &Cli, app_pjson: Option<&PjsonDetail>) -> Result<(), Error> {
        let PjsonDetail { name, version, .. } = PjsonDetail::new(path)?;

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

    fn order(&self, to_compare: &DiffModule) -> Ordering {
        self.name.cmp(&to_compare.name)
    }

    fn table_row(&self, row_type: RowType) -> Row {
        let name_cell = get_name_cell(&self.name, &self.dep_type);
        match row_type {
            RowType::DiffLeft => Row::new(vec![name_cell, Cell::new(&self.version)]),
            RowType::DiffRight(left_version) => {
                if left_version == self.version {
                    return row![self.version]
                } else {
                    return row![Fr => self.version]
                }
            },
            _ => row![],
        }
   }

}

impl Default for DiffModule {
    fn default() -> Self {
        DiffModule {
            name: String::new(),
            version: String::new(),
            dep_type: DepType::ChildDependency, 
        }
    }
}
