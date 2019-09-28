use crate::types::application_detail::AppDetail;
use crate::types::pjson_detail::PjsonDetail;
use std::collections::HashMap;
use std::io::Error;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum DepType {
    Dependency(String),
    DevDependency(String),
    PeerDependency(String),
}

/// Holds the name and version values from the package.json files.
#[derive(Debug)]
pub struct DepDetail {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub is_direct_dep: Option<DepType>,
}

impl DepDetail {
    /// Returns a new DepDetail type.
    pub fn new(base_path: &Path, app_details: &AppDetail) -> Result<DepDetail, Error> {
        let mut path = PathBuf::from(base_path);
        let pjson_details = PjsonDetail::new(&mut path)?;
        let PjsonDetail { name, version, .. } = pjson_details;

        // is direct dependency?
        let mut is_direct_dep =
            Self::check_is_direct(&name, &app_details.dependencies, DepType::Dependency);

        // is direct devDependency?
        if is_direct_dep.is_none() {
            is_direct_dep =
                Self::check_is_direct(&name, &app_details.dev_dependencies, DepType::DevDependency);
        }

        // is direct peerDependency?
        if is_direct_dep.is_none() {
            is_direct_dep = Self::check_is_direct(
                &name,
                &app_details.peer_dependencies,
                DepType::PeerDependency,
            );
        }

        println!("{} is {:?}", name, is_direct_dep);

        Ok(DepDetail {
            path,
            is_direct_dep,
            name,
            version,
        })
    }

    fn check_is_direct<F>(
        dep_name: &str,
        app_dependencies: &Option<HashMap<String, String>>,
        dep_type: F,
    ) -> Option<DepType>
    where
        F: Fn(String) -> DepType,
    {
        match app_dependencies {
            Some(deps) => match deps.get(dep_name) {
                Some(requested_version) => Some(dep_type(requested_version.to_string())),
                None => None,
            },
            None => None,
        }
    }
}
