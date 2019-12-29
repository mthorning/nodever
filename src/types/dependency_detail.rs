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
    None,
}

#[derive(Clone)]
pub enum DepKey {
    Name,
    Version,
    DepType,
    PjsonVersion,
}
#[derive(Clone)]
pub enum DepTuple {
    Main(DepKey),
    Diff(DepKey),
}

#[derive(Debug, Clone)]
pub struct Dependency(pub Option<DepDetail>, pub Option<DepDetail>);

impl Dependency {
    pub fn get_record_str(self: &Self, dep_tuple: &DepTuple) -> &str {
        let (dep_data, dep_key) = match dep_tuple {
            DepTuple::Main(key) => {
                let Dependency(data, _) = self;
                (data, key)
            }
            DepTuple::Diff(key) => {
                let Dependency(_, data) = self;
                (data, key)
            }
        };

        if let Some(detail) = dep_data {
            match dep_key {
                DepKey::Name => &detail.name,
                DepKey::Version => &detail.version,
                DepKey::DepType => match detail.dep_type {
                    DepType::Dependency => "D",
                    DepType::DevDependency => "dD",
                    DepType::PeerDependency => "pD",
                    DepType::ChildDependency => "child",
                    DepType::None => "",
                },
                DepKey::PjsonVersion => {
                    if let Some(pjson_version) = &detail.pjson_version {
                        pjson_version
                    } else {
                        ""
                    }
                }
            }
        } else {
            ""
        }
    }
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
