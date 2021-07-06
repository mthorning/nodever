use std::io::Error;
use std::path::PathBuf;
use std::cmp::Ordering;

use regex::Regex;

use crate::pjson_detail::PjsonDetail;
use crate::node_module::*;

#[derive(Debug)]
pub struct StandardModule {
    pub name: String,
    pub pjson_version: Option<String>,
    pub version: String,
    pub dep_type: DepType,
}

impl NodeModule for StandardModule {
    fn populate(&mut self, path: &PathBuf, app_pjson: Option<&PjsonDetail>) -> Result<(), Error> {
        let PjsonDetail { name, version, .. } = PjsonDetail::from(path)?;
        self.dep_type = get_dep_type(&name, app_pjson.unwrap());

        self.pjson_version = match &self.dep_type {
            DepType::ChildDependency => None,
            DepType::Dependency(version) => Some(version.to_owned()),
            DepType::DevDependency(version) => Some(version.to_owned()),
        };


        self.name = name;
        self.version = version;

        Ok(())
    }

    fn filter_by_regex(&self, re: &Regex) -> bool {
        re.is_match(&self.name)
    }

    fn filter_by_args(&self) -> bool {
        standard_filter(&self.dep_type)
    }

    fn order(&self, to_compare: &StandardModule) -> Ordering {
        self.name.cmp(&to_compare.name)
    }
}

impl PrintTable for StandardModule {
    fn table_row(&self) -> Row {
        Row::new(vec![
            new_cell(&self.name),
            get_pjson_version_cell(&self.pjson_version, &self.dep_type),
            new_cell(&self.version),
        ])
   }

}

impl Default for StandardModule {
    fn default() -> Self {
        StandardModule {
            name: String::new(),
            pjson_version: None,
            version: String::new(),
            dep_type: DepType::ChildDependency, 
        }
    }
}
