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
    pub required_by: Option<Vec<String>>,
}

impl NodeModule for StandardModule {
    fn populate(&mut self, path: &PathBuf, app_pjson: Option<&PjsonDetail>) -> Result<(), Error> {
        let PjsonDetail {
            name,
            version,
            required_by,
            ..
        } = PjsonDetail::from(path)?;

        self.dep_type = get_dep_type(&name, app_pjson.unwrap());

        self.name = name;
        self.version = Semver::from(version);
        self.required_by = required_by;

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

        let mut row = Row::new(vec![
            new_cell(&self.name),
            get_pjson_version_cell(&self.dep_type),
            new_cell(&version),
        ]);

        if let Some(required_by) = &self.required_by {
            row.add_cell(Cell::new(&required_by.join("\n").trim()));
        }

        row
    }
}

impl Default for StandardModule {
    fn default() -> Self {
        StandardModule {
            name: String::new(),
            version: None,
            dep_type: DepType::ChildDependency,
            required_by: None,
        }
    }
}
