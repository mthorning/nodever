use std::cmp::Ordering;
use std::io::Error;
use std::path::PathBuf;

use prettytable::{Cell, Row, Table};
use regex::Regex;

use crate::node_module::*;
use crate::pjson_detail::PjsonDetail;
use crate::semver::Semver;

#[derive(Clone)]
pub struct RequiredByModule {
    pub name: String,
    pub version: Option<Semver>,
    pub dep_type: DepType,
    required_by: Option<Vec<String>>,
    pub required_versions: Option<HashMap<String, String>>,
}

impl RequiredByModule {
    pub fn find_required_versions(&mut self, dependencies: &Vec<RequiredByModule>) {
        match &self.required_by {
            None => return,
            Some(required_by) => {
                for name in required_by {
                    for dependency in dependencies {
                        // println!("hi {:?} : {:?}", name, dependency.name);
                    }
                }       
            }
        }
    }
}

impl NodeModule for RequiredByModule {
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

    fn order(&self, to_compare: &RequiredByModule) -> Ordering {
        self.name.cmp(&to_compare.name)
    }

}

impl PrintTable for RequiredByModule {
    fn add_to_table(&self, table: &mut Table) {
        let version = match &self.version {
            Some(version) => format!("({})", version.to_string()),
            None => String::new(),
        };

        table.add_row(Row::new(vec![
            new_cell(&format!("{} {}", &self.name, version)).with_hspan(2),
        ]));

        if let Some(required_by) = &self.required_by {
            for requirement in required_by {
                table.add_row(Row::new(vec![
                    Cell::new(&requirement),
                    Cell::new("2.0.0")
                ]));
            }
        }
    }
}

impl Default for RequiredByModule {
    fn default() -> Self {
        RequiredByModule {
            name: String::new(),
            version: None,
            dep_type: DepType::ChildDependency,
            required_by: None,
            required_versions: None,
        }
    }
}
