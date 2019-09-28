use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PjsonDetail {
    name: String,
    version: String,

    #[serde(default = "default_to_none")]
    dependencies: Option<HashMap<String, String>>,

    #[serde(default = "default_to_none")]
    dev_dependencies: Option<HashMap<String, String>>,

    #[serde(default = "default_to_none")]
    peer_dependencies: Option<HashMap<String, String>>,
}

fn default_to_none() -> Option<HashMap<String, String>> {
    None
}

/// Returns the PjsonDetail type.
fn get_pjson_details(path: &mut PathBuf) -> Result<PjsonDetail, Error> {
    path.push("package.json");

    let pjson_string = match get_pjson(&path) {
        Ok(pjson_string) => pjson_string,
        Err(_) => return Err(Error::new(ErrorKind::NotFound, "package.json not found.")),
    };
    let pjson_details: PjsonDetail = serde_json::from_str(&pjson_string[..])?;

    Ok(pjson_details)
}

/// Returns the data from the package.json file.
fn get_pjson(path: &PathBuf) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub mod application_detail {

    use std::collections::HashMap;
    use std::io::Error;
    use std::path::{Path, PathBuf};

    /// Holds the name and version values from the package.json files.
    #[derive(Debug)]
    pub struct AppDetail {
        pub name: String,
        pub version: String,
        pub path: PathBuf,
        pub dependencies: Option<HashMap<String, String>>,
        pub dev_dependencies: Option<HashMap<String, String>>,
        pub peer_dependencies: Option<HashMap<String, String>>,
    }

    impl AppDetail {
        /// Returns a new AppDetail type.
        pub fn new(base_path: &Path) -> Result<AppDetail, Error> {
            let mut path = PathBuf::from(base_path);
            let pjson_details = super::get_pjson_details(&mut path)?;

            Ok(AppDetail {
                path,
                name: pjson_details.name,
                version: pjson_details.version,
                dependencies: pjson_details.dependencies,
                dev_dependencies: pjson_details.dev_dependencies,
                peer_dependencies: pjson_details.peer_dependencies,
            })
        }
    }
}

pub mod dependency_detail {

    use super::application_detail::AppDetail;
    use super::PjsonDetail;
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
            let pjson_details = super::get_pjson_details(&mut path)?;
            let PjsonDetail { name, version, .. } = pjson_details;

            // is direct dependency?
            let mut is_direct_dep =
                Self::check_is_direct(&name, &app_details.dependencies, DepType::Dependency);

            // is direct devDependency?
            if is_direct_dep.is_none() {
                is_direct_dep = Self::check_is_direct(
                    &name,
                    &app_details.dev_dependencies,
                    DepType::DevDependency,
                );
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
}
