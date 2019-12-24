use super::types::application_detail::AppDetail;
use super::types::dependency_detail::DepType;
use prettytable::{cell, row, Cell, Table};
use std::io::{self, Error, Write};

pub fn print_details(app_details: AppDetail) -> Result<(), Error> {
    let mut buffer = Vec::new();
    writeln!(
        &mut buffer,
        "\n{} matches found in version {} of {}. \n",
        app_details.dependency_details.len(),
        app_details.version,
        app_details.name
    )?;

    if !app_details.dependency_details.is_empty() {
        let mut table = Table::new();
        print_table_rows(app_details, &mut table);
        table.printstd();
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(buffer.as_mut_slice())?;
    Ok(())
}

fn print_table_rows(app_details: AppDetail, table: &mut Table) {
    for detail in app_details.dependency_details {
        let mut row_vec = row![detail.name];
        if app_details.args.direct_dep {
            if let Some((dep_type, version)) = add_direct_deps_col(&detail.is_direct_dep) {
                row_vec.add_cell(Cell::new(dep_type));
                row_vec.add_cell(Cell::new(version));
            }
        }
        row_vec.add_cell(Cell::new(&detail.version));
        table.add_row(row_vec);
    }
}

fn add_direct_deps_col<'a>(is_direct: &'a DepType) -> Option<(&'a str, &'a str)> {
    match is_direct {
        DepType::Dependency(pjson_version) => Some(("D", pjson_version)),
        DepType::DevDependency(pjson_version) => Some(("dD", pjson_version)),
        DepType::PeerDependency(pjson_version) => Some(("pD", pjson_version)),
        DepType::None => None,
    }
}
