use crate::types::cli::Cli;
use crate::types::dependency_detail::DepDetail;
use crate::types::pjson_detail::PjsonDetail;
use regex::Regex;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

/// Holds the name and version values from the package.json files.
#[derive(Debug)]
pub struct AppDetail {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub peer_dependencies: Option<HashMap<String, String>>,
    pub dependency_details: Vec<DepDetail>,
    pub args: Cli,
}

impl AppDetail {
    /// Returns a new AppDetail type.
    pub fn new(args: Cli) -> Result<AppDetail, Error> {
        let pjson_details = PjsonDetail::new(&args.path)?;
        let dependency_details = Vec::new();

        let mut new_app = AppDetail {
            name: pjson_details.name,
            version: pjson_details.version,
            dependencies: pjson_details.dependencies,
            dev_dependencies: pjson_details.dev_dependencies,
            peer_dependencies: pjson_details.peer_dependencies,
            dependency_details,
            args,
        };
        let mut base_path = PathBuf::from(&new_app.args.path);
        base_path.push("node_modules");

        new_app.get_dependencies(&base_path)?;

        Ok(new_app)
    }

    /// Loops through the node_modules directory and pushes the details into a Vec.
    fn get_dependencies(&mut self, base_path: &PathBuf) -> Result<(), Error> {
        let node_modules = match base_path.read_dir() {
            Ok(node_modules) => node_modules,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "node_modules folder not found.",
                ));
            }
        };

        for entry in node_modules {
            if let Ok(entry) = entry {
                let folder_name = entry.file_name().into_string().unwrap();

                if folder_name.starts_with('.') {
                    continue;
                }
                let mut dep_path = PathBuf::from(&base_path);
                if folder_name.starts_with('@') {
                    dep_path.push(&folder_name);
                    AppDetail::get_dependencies(self, &dep_path)?;
                } else {
                    dep_path.push(&folder_name);

                    match DepDetail::new(&dep_path, self) {
                        Ok(detail) => {
                            if self.filter_by_name(&detail) && self.filter_by_flags(&detail) {
                                self.dependency_details.push(detail);
                            }
                        }
                        Err(err) => println!("Error getting {:?}: {}", dep_path, err),
                    }
                }
            }
        }

        if !self.args.dont_sort {
            self.dependency_details.sort_by(|a, b| a.name.cmp(&b.name));
        }

        Ok(())
    }

    fn filter_by_name(&self, detail: &DepDetail) -> bool {
        let name_filter = Regex::new(&self.args.filter).unwrap();
        name_filter.is_match(&detail.name)
    }

    fn filter_by_flags(&self, detail: &DepDetail) -> bool {
        let Cli { direct_dep, .. } = self.args;

        if direct_dep && detail.is_direct_dep.is_none() {
            return false;
        }

        true
    }
}
