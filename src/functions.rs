use std::io::{self, Error, Write};
use std::path::{PathBuf, Path};
use regex::Regex;
use crate::types::detail::Detail;

/// Gets the name of the app.
pub fn get_app_details(base_path: &Path) -> Result<Detail, Error> {
    let mut path = PathBuf::from(base_path);
    path.push("package.json");
    Detail::from(path)
}

/// Loops through the node_modules directory and returns a Vec of the matching folder names.
pub fn get_dependency_folders(path: &Path, filter: Regex) -> Result<Vec<String>, Error> {
    let mut deps = Vec::new();

    for entry in path.read_dir()? {
        if let Ok(entry) = entry {
            let folder_name = entry.file_name().into_string().unwrap();
            if filter.is_match(&folder_name) {
                deps.push(folder_name);
            }
        }
    }
    Ok(deps)
}

/// Loops through the dependency folders and creates a Vec of Detail types.
pub fn get_dependency_details (
    base_path: &Path,
    folder_names: Vec<String>,
) -> Result<Vec<Detail>, Error> {
    let mut details = Vec::new();

    for name in folder_names {
        let mut path = PathBuf::from(base_path);
        path.push(name);
        path.push("package.json");
        match Detail::from(path) {
            Ok(detail) => details.push(detail),
            Err(_) => ()
        }
    }
    Ok(details)
}

pub fn print_details(app_details: Detail, dep_details: Vec<Detail>) -> Result<(), Error> {
    let mut buffer = Vec::new();
    write!(&mut buffer, "\n{} at version {} uses the following components: \n\n", app_details.name, app_details.version)?;

    if dep_details.len() > 0 {
        for detail in dep_details {
            write!(&mut buffer, "{} = {} \n", detail.name, detail.version)?;
        }
    } else {
        write!(&mut buffer, "No matches found. \n\n")?;
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(buffer.as_mut_slice())?;
    Ok(())
}

