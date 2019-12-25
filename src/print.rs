use super::types::application_detail::AppDetail;
use super::types::dependency_detail::{DepDetail, DepType};
use prettytable::{cell, color, row, Attr, Cell, Row, Table};
use regex::Regex;
use std::io::{self, Error, Write};

#[derive(Debug)]
enum Mode {
    PlainList,
    DirectDepsList,
}

pub fn print_details(app_details: &AppDetail) -> Result<(), Error> {
    print_table(app_details);
    print_completion_message(app_details)?;
    Ok(())
}

fn print_table(app_details: &AppDetail) {
    let mode = get_mode(app_details);
    if !app_details.dependency_details.is_empty() {
        let mut table = Table::new();
        print_table_rows(app_details, &mut table, mode);
        table.printstd();
    }
}

fn get_mode(app_details: &AppDetail) -> Mode {
    if app_details.args.direct_dep {
        return Mode::DirectDepsList;
    }
    Mode::PlainList
}

fn print_table_rows(app_details: &AppDetail, table: &mut Table, mode: Mode) {
    for detail in app_details.dependency_details.iter() {
        let mut row_vec = row![detail.name];
        if app_details.args.direct_dep {
            if let Some((dep_type, version)) = get_pjson_version(&detail.is_direct_dep) {
                row_vec.add_cell(Cell::new(dep_type));
                row_vec.add_cell(Cell::new(version));
            }
        }
        row_vec.add_cell(Cell::new(&detail.version));

        format_row(detail, &mut row_vec, &mode);
        table.add_row(row_vec);
    }
}

fn get_pjson_version<'a>(is_direct: &'a DepType) -> Option<(&'a str, &'a str)> {
    match is_direct {
        DepType::Dependency(pjson_version) => Some(("D", pjson_version)),
        DepType::DevDependency(pjson_version) => Some(("dD", pjson_version)),
        DepType::PeerDependency(pjson_version) => Some(("pD", pjson_version)),
        DepType::None => None,
    }
}

fn format_row(detail: &DepDetail, row_vec: &mut Row, mode: &Mode) {
    match mode {
        Mode::PlainList => {
            if detail.is_direct_dep != DepType::None {
                for cell in row_vec.iter_mut() {
                    cell.style(Attr::ForegroundColor(color::BLUE));
                }
            }
        }
        Mode::DirectDepsList => {
            if let Some((_, pjson_version)) = get_pjson_version(&detail.is_direct_dep) {
                let version_re = Regex::new(&detail.version).unwrap();
                if version_re.is_match(&pjson_version) {
                    for cell in row_vec.iter_mut() {
                        cell.style(Attr::ForegroundColor(color::CYAN));
                    }
                }
            }
        }
    }
}

fn print_completion_message(app_details: &AppDetail) -> Result<(), Error> {
    let mut buffer = Vec::new();
    writeln!(
        &mut buffer,
        "\n{} matches found in version {} of {}. \n",
        app_details.dependency_details.len(),
        app_details.version,
        app_details.name
    )?;
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(buffer.as_mut_slice())?;
    Ok(())
}
