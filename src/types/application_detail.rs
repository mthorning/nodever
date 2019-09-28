use crate::types::pjson_detail::PjsonDetail;
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
        let pjson_details = PjsonDetail::new(&mut path)?;

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
