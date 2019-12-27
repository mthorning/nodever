use super::types::application_detail::AppDetail;
use super::types::dependency_detail::{DepDetail, DepType};
use prettytable::{cell, color, row, Attr, Cell, Row, Table};
use regex::Regex;
use std::io::{self, Error, Write};

#[derive(PartialEq)]
enum Mode<'a> {
    PlainList(Vec<&'a str>),
    DirectDepsList(Vec<&'a str>),
}

pub fn print_details(app_details: &AppDetail) -> Result<(), Error> {
    let mode = get_mode(app_details);
    print_table(&mode, app_details);
    print_completion_message(app_details)?;
    Ok(())
}

fn print_table(mode: &Mode, app_details: &AppDetail) {
    if !app_details.dependency_details.is_empty() {
        let mut table = Table::new();
        add_table_rows(app_details, &mut table, mode);
        table.printstd();
    }
}

fn get_mode(app_details: &AppDetail) -> Mode {
    if app_details.args.direct_dep {
        return Mode::DirectDepsList(vec!["Package", "Type", "PJSON", "Version"]);
    }
    Mode::PlainList(vec!["Package", "Version"])
}

fn add_table_rows(app_details: &AppDetail, table: &mut Table, mode: &Mode) {
    for detail in app_details.dependency_details.iter() {
        let mut row_vec = row![detail.name];
        if let Mode::DirectDepsList(_) = mode {
            add_direct_deps_cols(&detail.is_direct_dep, &mut row_vec);
        }
        row_vec.add_cell(Cell::new(&detail.version));

        format_row(&mut row_vec, detail, &mode);
        table.add_row(row_vec);
    }
}

fn add_direct_deps_cols(is_direct_dep: &DepType, row_vec: &mut Row) {
    if let Some((dep_type, version)) = get_pjson_version(is_direct_dep) {
        row_vec.add_cell(Cell::new(dep_type));
        row_vec.add_cell(Cell::new(version));
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

fn format_row(row_vec: &mut Row, detail: &DepDetail, mode: &Mode) {
    match mode {
        Mode::PlainList(_) => {
            if detail.is_direct_dep != DepType::None {
                for cell in row_vec.iter_mut() {
                    cell.style(Attr::ForegroundColor(color::BLUE));
                }
            }
        }
        Mode::DirectDepsList(_) => {
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
