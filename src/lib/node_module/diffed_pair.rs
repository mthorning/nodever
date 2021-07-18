use prettytable::{Attr, color};

use crate::node_module::*;
use crate::node_module::standard::StandardModule;
use crate::semver::Semver;

pub struct DiffedPair<'a> {
    pub name: &'a str,
    pub version: (&'a Option<Semver>, &'a Option<Semver>),
    pub dep_type: (&'a DepType, &'a DepType),
}

impl<'a> DiffedPair<'a>{
    pub fn from(dependency: &'a StandardModule) -> Self {
        DiffedPair {
            name: &dependency.name,
            version: (&dependency.version, &None),
            dep_type: (&dependency.dep_type, &DepType::ChildDependency),
        }
    }

    pub fn get_pairs(dependencies: &'a Vec<StandardModule>, diff_dependencies: &'a Vec<StandardModule>) -> Vec<Self> {
        let mut diffed_pairs = Vec::new();
        let mut found_deps = Vec::new();

        for dependency in dependencies {
            let mut new_pair = DiffedPair::from(&dependency);

            for diff_dependency in diff_dependencies.iter() {
                if dependency.name == diff_dependency.name {
                        new_pair.version.1 = &diff_dependency.version;
                        found_deps.push(diff_dependency.name.clone());
                        break;
                }
                if dependency.name < diff_dependency.name {
                    break;
                }
            }
            diffed_pairs.push(new_pair);
        }

        //Add left over diff_dependencies
        for diff_dependency in diff_dependencies {
            if !found_deps.contains(&diff_dependency.name) {
                diffed_pairs.push(DiffedPair{ 
                    name: &diff_dependency.name, 
                    dep_type: (&DepType::ChildDependency, &diff_dependency.dep_type),
                    version: (&None, &diff_dependency.version),
                });
            }
        }

        diffed_pairs.sort_by(|a, b| a.name.cmp(&b.name));
        diffed_pairs
    }
}

impl<'a> PrintTable for DiffedPair<'a> {
    fn table_row(&self) -> Row {
        let (version_one, version_two) = diffed_cells(&self.version.0, &self.version.1);
        Row::new(vec![
            new_cell(&self.name),
            get_pjson_version_cell(&self.dep_type.0),
            version_one,
            get_pjson_version_cell(&self.dep_type.1),
            version_two,
        ])
   }
}

fn diffed_cells(version_one: &Option<Semver>, version_two: &Option<Semver>) -> (Cell, Cell) {

    fn cell(version: &Option<Semver>) -> Cell {
        let cell_contents = match &version {
            Some(contents) => contents.to_string(),
            None => String::new(),
        };
        new_cell(&cell_contents)
    }

    let mut cells = (cell(version_one), cell(version_two));

    if version_one.is_none() || version_two.is_none() { return cells; }

    fn coloured_cell(cell: Cell, color: color::Color) -> Cell {
        cell.with_style(Attr::ForegroundColor(color))
    }

    if version_one < version_two {
        cells = (
            coloured_cell(cells.0, color::RED),
            coloured_cell(cells.1, color::GREEN),
        )
    }
    if version_one > version_two {
        cells = (
            coloured_cell(cells.0, color::GREEN),
            coloured_cell(cells.1, color::RED),
        )
    }

    cells
}
