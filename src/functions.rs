use crate::types::detail::Detail;
use regex::Regex;
use std::io::{self, Error, Write};
use std::path::{Path, PathBuf};

/// Gets the details for the selected dependency.
pub fn get_dependency_details(base_path: &Path) -> Result<Detail, Error> {
    let mut path = PathBuf::from(base_path);
    path.push("package.json");
    Detail::from(path)
}

/// Loops through the node_modules directory and pushes the details into a Vec.
pub fn get_dependencies(deps: &mut Vec<Detail>, base_path: &Path, filter: &Regex) {
    for entry in base_path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let folder_name = entry.file_name().into_string().unwrap();

            if folder_name.starts_with('.') {
                continue;
            }
            if folder_name.starts_with('@') {
                let mut path = PathBuf::from(base_path);
                path.push(&folder_name);
                get_dependencies(deps, &path, filter);
            } else if filter.is_match(&folder_name) {
                let mut path = PathBuf::from(base_path);
                path.push(&folder_name);
                match get_dependency_details(&path) {
                    Ok(details) => deps.push(details),
                    Err(err) => println!("Error getting {:?}: {}", path, err),
                }
            }
        }
    }
}

pub fn print_details(app_details: Detail, dep_details: Vec<Detail>) -> Result<(), Error> {
    let mut buffer = Vec::new();
    writeln!(
        &mut buffer,
        "\n{} matches found in version {} of {}. \n",
        dep_details.len(),
        app_details.version,
        app_details.name
    )?;

    if !dep_details.is_empty() {
        for detail in dep_details {
            writeln!(&mut buffer, "{}: {}", detail.name, detail.version)?;
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(buffer.as_mut_slice())?;
    Ok(())
}
