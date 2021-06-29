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

pub struct DiffedPair {
    pub name: String,
    pub version_one: String,
    pub version_two: String,
    pub dep_type: DepType,
}

impl DiffedPair {
    pub fn get_pairs(dependencies: Vec<DiffModule>, mut diff_dependencies: Vec<DiffModule>) -> Vec<Self> {
        let mut diffed_pairs = Vec::new();

        for dependency in dependencies {
            let mut version_two = String::new();
            for (i, diff_dependency) in diff_dependencies.iter().enumerate() {
                match dependency.name.cmp(&diff_dependency.name) {
                    Ordering::Equal => {
                        version_two = diff_dependency.version.clone();
                        diff_dependencies.remove(i);
                        break;
                    }
                    Ordering::Less => break,
                    Ordering::Greater => (),
                }
            }
            let new_pair = DiffedPair{ 
                name: dependency.name, 
                dep_type: dependency.dep_type, 
                version_one: dependency.version,
                version_two,
            };
            diffed_pairs.push(new_pair);
        }

        //Add left over diff_dependencies
        for diff_dependency in diff_dependencies {
            diffed_pairs.push(DiffedPair{ 
                name: diff_dependency.name, 
                dep_type: diff_dependency.dep_type, 
                version_one: String::new(), 
                version_two: diff_dependency.version 
            });
        }

        diffed_pairs.sort_by(|a, b| a.name.cmp(&b.name));
        diffed_pairs
    }
}

impl PrintTable for DiffedPair {
    fn table_row(&self, _row_type: RowType) -> Row {
        Row::new(vec![
            get_name_cell(&self.name, &self.dep_type),
            Cell::new(&self.version_one),
            Cell::new(&self.version_two),
        ])
   }
}

impl NodeModule for DiffModule {
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

    fn order(&self, to_compare: &DiffModule) -> Ordering {
        self.name.cmp(&to_compare.name)
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
