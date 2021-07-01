use std::cmp::Ordering;

use crate::node_module::*;
use crate::node_module::standard_module::StandardModule;

pub struct DiffedPair<'a> {
    pub name: &'a str,
    pub pjson_version_one: &'a Option<String>,
    pub pjson_version_two: &'a Option<String>,
    pub version_one: &'a str,
    pub version_two: &'a str,
    pub dep_type: &'a DepType,
}

impl<'a> DiffedPair<'a>{
    pub fn get_pairs(dependencies: &'a Vec<StandardModule>, diff_dependencies: &'a Vec<StandardModule>) -> Vec<Self> {
        let mut diffed_pairs = Vec::new();
        let mut found_deps = Vec::new();

        for dependency in dependencies {
            let mut version_two = "";
            let mut pjson_version_two = &None;

            for diff_dependency in diff_dependencies.iter() {
                match dependency.name.cmp(&diff_dependency.name) {
                    Ordering::Equal => {
                        version_two = &diff_dependency.version;
                        pjson_version_two = &diff_dependency.pjson_version;
                        found_deps.push(diff_dependency.name.clone());
                        break;
                    }
                    Ordering::Less => break,
                    Ordering::Greater => (),
                }
            }
            let new_pair = DiffedPair{ 
                name: &dependency.name, 
                dep_type: &dependency.dep_type, 
                pjson_version_one: &dependency.pjson_version,
                version_one: &dependency.version,
                pjson_version_two, 
                version_two,
            };
            diffed_pairs.push(new_pair);
        }

        //Add left over diff_dependencies
        for diff_dependency in diff_dependencies {
            if !found_deps.contains(&diff_dependency.name) {
                diffed_pairs.push(DiffedPair{ 
                    name: &diff_dependency.name, 
                    dep_type: &diff_dependency.dep_type, 
                    pjson_version_one: &None,
                    pjson_version_two: &diff_dependency.pjson_version,
                    version_one: "",
                    version_two: &diff_dependency.version 
                });
            }
        }

        diffed_pairs.sort_by(|a, b| a.name.cmp(&b.name));
        diffed_pairs
    }
}

impl<'a> PrintTable for DiffedPair<'a> {
    fn table_row(&self) -> Row {
        Row::new(vec![
            new_cell(&self.name),
            get_pjson_version_cell(&self.pjson_version_one, &self.dep_type),
            new_cell(&self.version_one),
            get_pjson_version_cell(&self.pjson_version_two, &self.dep_type),
            new_cell(&self.version_two),
        ])
   }
}
