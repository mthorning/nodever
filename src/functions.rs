use crate::types::application_detail::AppDetail;
use crate::types::dependency_detail::DepDetail;
use std::io::{self, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

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
