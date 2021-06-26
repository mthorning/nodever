use crate::types::application_detail::AppDetail;
use crate::types::pjson_detail::PjsonDetail;
use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum DepType {
    Dependency(String),
    DevDependency(String),
    PeerDependency(String),
    ChildDependency,
}

// #[derive(Clone)]
// pub enum DepKey {
//     Version,
//     DepType,
//     PjsonVersion,
// }
// #[derive(Clone)]
// pub enum DepTuple {
//     Main(DepKey),
//     Diff(DepKey),
// }

pub trait DepDetail {
    fn print(&self) -> String {
       self.name 
    }
}

/// Holds the name and version values from the package.json files.
pub struct GlobalDep {
    pub name: String,
    pub version: String,
}

impl DepDetail for GlobalDep {}

impl GlobalDep {
    fn new(path: &PathBuf, app_details: &AppDetail) -> Result<GlobalDep, Error> {
        let PjsonDetail { name, version, .. } = PjsonDetail::new(path)?;

        let new_dep_detail = GlobalDep {
            name,
            version,
        };

        Ok(new_dep_detail)
    }
}

pub struct StandardDep {
    pub name: String,
    pub version: String,
    pub dep_type: DepType,
}

impl DepDetail for StandardDep {}

impl StandardDep {
    fn new(self, path: &PathBuf, app_details: &AppDetail) -> Result<StandardDep, Error> {
        let PjsonDetail { name, version, .. } = PjsonDetail::new(path)?;

        let new_dep_detail = StandardDep {
            name,
            version,
            dep_type: self.get_dep_type(&name, app_details),
        };

        Ok(new_dep_detail)
    }
    fn get_dep_type(self, name: &str, app_details: &AppDetail) -> DepType {
        match self.get_pjson_details(name, app_details.pjson.dependencies) {
            Some(required_version) => return DepType::Dependency(required_version),
            None => match self.get_pjson_details(name, app_details.pjson.devDependencies) {
                Some(required_version) => return DepType::DevDependency(required_version),
                None => match self.get_pjson_details(name, app_details.pjson.peerDependencies) {
                    Some(required_version) => return DepType::peerDependency(required_version),
                    None => DepType::ChildDependency
                }
            }
        }
    }

    fn get_pjson_details(
        dep_name: &str,
        required_dependencies: Option<HashMap<String, String>>,
    ) -> Option<String> {
        match required_dependencies {
            Some(deps) => match deps.get(dep_name) {
                Some(required_version) => Some(required_version),
                None => None,
            },
            None => None,
        }
    }
}
