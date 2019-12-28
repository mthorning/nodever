use super::types::application_detail::AppDetail;
use super::types::dependency_detail::{DepDetail, DepValue};
use prettytable::{color, Attr, Cell, Row, Table};
use regex::Regex;
use std::io::{self, Error, Write};

#[derive(PartialEq)]
enum Mode {
    PlainList,
    DirectDepsList,
}

struct Schema(Mode, Vec<Col>);
struct Col(&'static str, DepValue);

fn get_table_cols(app_details: &AppDetail) -> Schema {
    if app_details.args.direct_dep {
        return Schema(
            Mode::DirectDepsList,
            vec![
                Col("Package", DepValue::Name),
                Col("Type", DepValue::DepType),
                Col("PJSON", DepValue::PjsonVersion),
                Col("Version", DepValue::Version),
            ],
        );
    }
    Schema(
        Mode::PlainList,
        vec![
            Col("Package", DepValue::Name),
            Col("Version", DepValue::Version),
        ],
    )
}

pub fn print_details(app_details: &AppDetail) -> Result<(), Error> {
    let schema = get_table_cols(app_details);
    print_table(&schema, app_details);
    print_completion_message(app_details)?;
    Ok(())
}

fn print_table(schema: &Schema, app_details: &AppDetail) {
    if !app_details.dependency_details.is_empty() {
        let mut table = Table::new();
        add_table_rows(app_details, &mut table, schema);
        table.printstd();
    }
}

fn add_table_rows(app_details: &AppDetail, table: &mut Table, schema: &Schema) {
    let Schema(mode, cols) = schema;
    for detail in app_details.dependency_details.iter() {
        let mut row_vec = Vec::new();
        for col in cols.iter() {
            let Col(_, record) = col;
            row_vec.push(Cell::new(detail.get_record_str(record)));
        }

        let mut row = Row::new(row_vec);
        format_row(&mut row, detail, &mode);
        table.add_row(row);
    }
}

fn format_row(row: &mut Row, detail: &DepDetail, mode: &Mode) {
    match mode {
        Mode::PlainList => {
            if detail.pjson_version != None {
                for cell in row.iter_mut() {
                    cell.style(Attr::ForegroundColor(color::BLUE));
                }
            }
        }
        Mode::DirectDepsList => {
            if let Some(pjson_version) = &detail.pjson_version {
                let version_re = Regex::new(&detail.version).unwrap();
                if version_re.is_match(&pjson_version) {
                    for cell in row.iter_mut() {
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
