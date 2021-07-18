use std::cmp::Ordering;
use std::io::Error;
use std::path::PathBuf;

use regex::Regex;

use crate::node_module::*;
use crate::pjson_detail::PjsonDetail;
use crate::semver::Semver;

pub struct StandardModule {
    pub name: String,
    pub version: Option<Semver>,
    pub dep_type: DepType,
}

impl NodeModule for StandardModule {
    fn populate(&mut self, path: &PathBuf, app_pjson: Option<&PjsonDetail>) -> Result<(), Error> {
        let PjsonDetail {
            name,
            version,
            ..
        } = PjsonDetail::from(path)?;

        self.dep_type = get_dep_type(&name, app_pjson.unwrap());

        self.name = name;
        self.version = Semver::from(version);

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
        let version = match &self.version {
            Some(version) => version.to_string(),
            None => String::new(),
        };

        Row::new(vec![
            new_cell(&self.name),
            get_pjson_version_cell(&self.dep_type),
            new_cell(&version),
        ])
    }

    fn add_to_table(&self, table: &mut Table) {
        let version = match &self.version {
            Some(version) => version.to_string(),
            None => String::new(),
        };

        table.add_row(Row::new(vec![
            new_cell(&self.name),
            get_pjson_version_cell(&self.dep_type),
            new_cell(&version),
        ]));
    }
}

impl Default for StandardModule {
    fn default() -> Self {
        StandardModule {
            name: String::new(),
            version: None,
            dep_type: DepType::ChildDependency,
        }
    }
}
