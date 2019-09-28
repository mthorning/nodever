use crate::types::dependency_detail::DepDetail;
use crate::types::pjson_detail::PjsonDetail;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

/// Holds the name and version values from the package.json files.
#[derive(Debug)]
pub struct AppDetail {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub peer_dependencies: Option<HashMap<String, String>>,
    pub dependency_details: Vec<DepDetail>,
}

impl AppDetail {
    /// Returns a new AppDetail type.
    pub fn new(mut base_path: PathBuf) -> Result<AppDetail, Error> {
        let pjson_details = PjsonDetail::new(&base_path)?;
        let dependency_details = Vec::new();

        let mut new_app = AppDetail {
            name: pjson_details.name,
            version: pjson_details.version,
            dependencies: pjson_details.dependencies,
            dev_dependencies: pjson_details.dev_dependencies,
            peer_dependencies: pjson_details.peer_dependencies,
            dependency_details,
        };
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
                        Ok(detail) => self.dependency_details.push(detail),
                        Err(err) => println!("Error getting {:?}: {}", dep_path, err),
                    }
                }
            }
        }
        Ok(())
    }
}
