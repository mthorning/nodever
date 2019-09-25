use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
struct PjsonDetails {
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

/// Returns the PjsonDetails type.
fn get_pjson_details(path: &mut PathBuf) -> Result<PjsonDetails, Error> {
    path.push("package.json");

    let pjson_string = match get_pjson(&path) {
        Ok(pjson_string) => pjson_string,
        Err(_) => return Err(Error::new(ErrorKind::NotFound, "package.json not found.")),
    };
    let pjson_details: PjsonDetails = serde_json::from_str(&pjson_string[..])?;

    Ok(pjson_details)
}

/// Returns the data from the package.json file.
fn get_pjson(path: &PathBuf) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

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
        let pjson_details = get_pjson_details(&mut path)?;

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
/// Holds the name and version values from the package.json files.
#[derive(Debug)]
pub struct DepDetail {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
}

impl DepDetail {
    /// Returns a new DepDetail type.
    pub fn new(base_path: &Path) -> Result<DepDetail, Error> {
        let mut path = PathBuf::from(base_path);
        let pjson_details = get_pjson_details(&mut path)?;

        Ok(DepDetail {
            path,
            name: pjson_details.name,
            version: pjson_details.version,
        })
    }
}
