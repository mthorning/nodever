use crate::types::detail::application_detail::AppDetail;
use crate::types::detail::dependency_detail::DepDetail;
use std::io::{self, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

/// Loops through the node_modules directory and pushes the details into a Vec.
pub fn get_dependencies(
    deps: &mut Vec<DepDetail>,
    base_path: &Path,
    app_details: &AppDetail,
) -> Result<(), Error> {
    let node_modules = match base_path.read_dir() {
        Ok(node_modules) => node_modules,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::NotFound,
                "node_modules folder not found.",
            ))
        }
    };

    for entry in node_modules {
        if let Ok(entry) = entry {
            let folder_name = entry.file_name().into_string().unwrap();

            if folder_name.starts_with('.') {
                continue;
            }
            if folder_name.starts_with('@') {
                let mut path = PathBuf::from(base_path);
                path.push(&folder_name);
                get_dependencies(deps, &path, app_details)?;
            } else {
                let mut path = PathBuf::from(base_path);
                path.push(&folder_name);

                match DepDetail::new(&path, app_details) {
                    Ok(detail) => deps.push(detail),
                    Err(err) => println!("Error getting {:?}: {}", path, err),
                }
            }
        }
    }
    Ok(())
}

pub fn print_details(app_details: AppDetail, dep_details: Vec<DepDetail>) -> Result<(), Error> {
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
