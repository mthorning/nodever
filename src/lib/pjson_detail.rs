use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PjsonDetail {
    pub name: String,
    pub version: String,

    #[serde(default = "default_to_none")]
    pub dependencies: Option<HashMap<String, String>>,

    #[serde(default = "default_to_none")]
    pub dev_dependencies: Option<HashMap<String, String>>,

    #[serde(default = "default_to_none")]
    pub peer_dependencies: Option<HashMap<String, String>>,

    #[serde(rename(deserialize = "_requiredBy"), default = "default_to_none")]
    pub required_by: Option<Vec<String>>,
}

fn default_to_none<T>() -> Option<T> {
    None
}

impl PjsonDetail {
    /// Returns the PjsonDetail type.
    pub fn from(base_path: &PathBuf) -> Result<PjsonDetail, Error> {
        let mut path = PathBuf::from(base_path);
        path.push("package.json");

        let pjson_string = Self::get_pjson(&path)?;
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
}
