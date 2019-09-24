use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::PathBuf;

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

/// Holds the name and version values from the package.json files.
#[derive(Debug)]
pub struct Detail {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub peer_dependencies: Option<HashMap<String, String>>,
}

impl Detail {
    /// Returns a new Detail type.
    pub fn from(path: PathBuf) -> Result<Detail, Error> {
        let pjson_string = match Detail::get_pjson(&path) {
            Ok(pjson_string) => pjson_string,
            Err(_) => return Err(Error::new(ErrorKind::NotFound, "package.json not found.")),
        };
        let pjson_details: PjsonDetails = serde_json::from_str(&pjson_string[..])?;

        Ok(Detail {
            name: pjson_details.name,
            version: pjson_details.version,
            path,
            dependencies: pjson_details.dependencies,
            dev_dependencies: pjson_details.dev_dependencies,
            peer_dependencies: pjson_details.peer_dependencies,
        })
    }

    /// Returns the data from the package.json file.
    fn get_pjson(path: &PathBuf) -> Result<String, Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
