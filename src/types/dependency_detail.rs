use crate::types::application_detail::AppDetail;
use crate::types::pjson_detail::PjsonDetail;
use std::collections::HashMap;
use std::io::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum DepType {
    Dependency,
    DevDependency,
    PeerDependency,
    ChildDependency,
}

#[derive(Clone)]
pub enum DepKey {
    Version,
    DepType,
    PjsonVersion,
}
#[derive(Clone)]
pub enum DepTuple {
    Main(DepKey),
    Diff(DepKey),
}

/// Holds the name and version values from the package.json files.
#[derive(Debug, Clone)]
pub struct DepDetail {
    pub name: String,
    pub version: String,
    pub pjson_version: Option<String>,
    pub dep_type: DepType,
}

impl DepDetail {
    /// Returns a new DepDetail type.
    pub fn new(base_path: &Path, app_details: &AppDetail) -> Result<DepDetail, Error> {
        let path = PathBuf::from(base_path);
        let pjson_details = PjsonDetail::new(&path)?;
        let PjsonDetail { name, version, .. } = pjson_details;

        // is a direct dependency?
        match Self::get_pjson_details(&name, &app_details.dependencies) {
            Some(pjson_version) => Ok(DepDetail {
                name,
                version,
                pjson_version: Some(pjson_version),
                dep_type: DepType::Dependency,
            }),
            // is a direct devDependency?
            None => match Self::get_pjson_details(&name, &app_details.dev_dependencies) {
                Some(pjson_version) => Ok(DepDetail {
                    name,
                    version,
                    pjson_version: Some(pjson_version),
                    dep_type: DepType::DevDependency,
                }),
                // is a direct peerDependency?
                None => match Self::get_pjson_details(&name, &app_details.peer_dependencies) {
                    Some(pjson_version) => Ok(DepDetail {
                        name,
                        version,
                        pjson_version: Some(pjson_version),
                        dep_type: DepType::PeerDependency,
                    }),
                    // must be a childDependency
                    None => Ok(DepDetail {
                        name,
                        version,
                        pjson_version: None,
                        dep_type: DepType::ChildDependency,
                    }),
                },
            },
        }
    }

    fn get_pjson_details(
        dep_name: &str,
        app_dependencies: &Option<HashMap<String, String>>,
    ) -> Option<String> {
        match app_dependencies {
            Some(deps) => match deps.get(dep_name) {
                Some(pjson_version) => Some(pjson_version.to_owned()),
                None => None,
            },
            None => None,
        }
    }
}
