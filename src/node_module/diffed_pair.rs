use std::cmp::Ordering;

use prettytable::{Attr, color};

use crate::node_module::*;
use crate::node_module::standard_module::StandardModule;
use crate::semver::Semver;

pub struct DiffedPair<'a> {
    pub name: &'a str,
    pub pjson_version_one: &'a Option<String>,
    pub pjson_version_two: &'a Option<String>,
    pub version_one: Option<Semver<'a>>,
    pub version_two: Option<Semver<'a>>,
    pub dep_type: &'a DepType,
}

impl<'a> DiffedPair<'a>{
    pub fn get_pairs(dependencies: &'a Vec<StandardModule>, diff_dependencies: &'a Vec<StandardModule>) -> Vec<Self> {
        let mut diffed_pairs = Vec::new();
        let mut found_deps = Vec::new();

        for dependency in dependencies {
            let mut version_two = None;
            let mut pjson_version_two = &None;

            for diff_dependency in diff_dependencies.iter() {
                match dependency.name.cmp(&diff_dependency.name) {
                    Ordering::Equal => {
                        version_two = Some(Semver::from(&diff_dependency.version));
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
                version_one: Some(Semver::from(&dependency.version)),
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
                    version_one: None,
                    version_two: Some(Semver::from(&diff_dependency.version)),
                });
            }
        }

        diffed_pairs.sort_by(|a, b| a.name.cmp(&b.name));
        diffed_pairs
    }
}

impl<'a> PrintTable for DiffedPair<'a> {
    fn table_row(&self) -> Row {
        let (version_one, version_two) = diffed_cells(&self.version_one, &self.version_two);
        Row::new(vec![
            new_cell(&self.name),
            get_pjson_version_cell(&self.pjson_version_one, &self.dep_type),
            version_one,
            get_pjson_version_cell(&self.pjson_version_two, &self.dep_type),
            version_two,
        ])
   }
}

fn diffed_cells(version_one: &Option<Semver>, version_two: &Option<Semver>) -> (Cell, Cell) {

    let cell = |version: &Option<Semver>| {
        new_cell(&version.as_ref().map_or(String::from(""), |output| output.to_string())[..])
    };

    let cells = (cell(version_one), cell(version_two));

    if version_one.is_none() || version_two.is_none() { return cells; }

    let cell = |cell: Cell, color: color::Color| cell.with_style(Attr::ForegroundColor(color));
    
    match version_one.as_ref().unwrap().cmp(&version_two.as_ref().unwrap()) {
        Ordering::Equal => cells,
        Ordering::Less => (
            cell(cells.0, color::RED),
            cell(cells.1, color::GREEN),
        ),
        Ordering::Greater => (
            cell(cells.0, color::GREEN),
            cell(cells.1, color::RED),
        )
    }
}
