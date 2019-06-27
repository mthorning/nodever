use exitfailure::ExitFailure;
use failure::ResultExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The path to the root of the Nexcenter app.
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// The pattern to filter folders in node_modules.
    #[structopt(short = "f", long = "filter")]
    filter: String,
}

/// Holds the name and version values from the package.json files.
#[derive(Debug)]
pub struct Version {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize)]
struct PjsonDetails {
    name: String,
    version: String,
}

impl Version {
    /// Returns a new Version type.
    pub fn from(path: PathBuf) -> Result<Version, Error> {
        let pjson_string = Version::get_pjson(&path)?;
        let pjson_details: PjsonDetails = serde_json::from_str(&pjson_string[..])?;

        Ok(Version {
            name: pjson_details.name,
            version: pjson_details.version,
            path,
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

/// Loops through the node_modules directory and returns a Vec of the matching folder names.
fn get_dependency_folders(path: &Path, filter: &str) -> Result<Vec<String>, Error> {
    let mut deps = Vec::new();

    for entry in path.read_dir()? {
        if let Ok(entry) = entry {
            let folder_name = entry.file_name().into_string().unwrap();
            if folder_name.starts_with(filter) {
                deps.push(folder_name);
            }
        }
    }

    Ok(deps)
}

/// Loops through the dependency folders and creates a Vec of Version types.
fn get_dependency_versions(
    base_path: &Path,
    folder_names: Vec<String>,
) -> Result<Vec<Version>, Error> {
    let mut versions = Vec::new();

    for name in folder_names {
        let mut path = PathBuf::from(base_path);
        path.push(name);
        path.push("package.json");
        versions.push(Version::from(path)?);
    }
    Ok(versions)
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let mut path = args.path;
    let filter = args.filter;

    path.push("node_modules");

    let dependency_folders = get_dependency_folders(&path.as_path(), &filter)?;
    let versions = get_dependency_versions(&path.as_path(), dependency_folders);
    println!("{:?}", versions);

    Ok(())
}
