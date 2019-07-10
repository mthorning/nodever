use exitfailure::ExitFailure;
use std::io::{self,Write};
use std::io::Error;
use std::path::{PathBuf, Path};
use nexver::detail::Detail;
use structopt::StructOpt;
use nexver::cli::Cli;

/// Gets the name of the app.
fn get_app_details(base_path: &Path) -> Result<Detail, Error> {
    let mut path = PathBuf::from(base_path);
    path.push("package.json");
    Detail::from(path)
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

/// Loops through the dependency folders and creates a Vec of Detail types.
fn get_dependency_details (
    base_path: &Path,
    folder_names: Vec<String>,
) -> Result<Vec<Detail>, Error> {
    let mut details = Vec::new();

    for name in folder_names {
        let mut path = PathBuf::from(base_path);
        path.push(name);
        path.push("package.json");
        details.push(Detail::from(path)?);
    }
    Ok(details)
}

fn print_details(app_details: Detail, dep_details: Vec<Detail>) -> Result<(), Error> {
    let mut buffer = Vec::new();
    write!(&mut buffer, "\n{} at version {} uses the following components: \n\n", app_details.name, app_details.version)?;
    for detail in dep_details {
        write!(&mut buffer, "{} = {} \n", detail.name, detail.version)?;
    }
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write(buffer.as_mut_slice())?;
    Ok(())
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let mut path = args.path;
    let filter = args.filter;

    let app_details = get_app_details(&path.as_path())?;

    path.push("node_modules");

    let dependency_folders = get_dependency_folders(&path.as_path(), &filter)?;
    let details = get_dependency_details(&path.as_path(), dependency_folders);

    if let Ok(details) = details {
        print_details(app_details, details)?;
    }

    Ok(())
}
