use super::types::application_detail::AppDetail;
use super::types::dependency_detail::{DepDetail, DepValue};
use super::types::output_schema::{Col, DiffCols, Mode, Schema};
use prettytable::{color, Attr, Cell, Row, Table};
use regex::Regex;
use std::io::{self, Error, Write};

pub fn print_details(schema: Schema) -> Result<(), Error> {
    print_table(&schema);
    print_completion_message(&schema)?;
    Ok(())
}

fn print_table(schema: &Schema) {
    let Schema {
        app_details,
        mode,
        cols,
        diff,
        ..
    } = schema;
    if !app_details.dependency_details.is_empty() {
        let mut table = Table::new();
        add_table_headers(&mut table, schema);
        add_table_rows(app_details, &mut table, mode, cols, diff);
        table.printstd();
    }
}

fn add_table_headers(table: &mut Table, schema: &Schema) {
    let Schema { cols, .. } = schema;

    let mut row_vec = Vec::new();
    for col in cols.iter() {
        let Col(title, _) = col;
        row_vec.push(Cell::new(title).style_spec("BcFw"));
    }

    let row = Row::new(row_vec);
    table.add_row(row);
}

fn add_table_rows(
    app_details: &AppDetail,
    table: &mut Table,
    mode: &Mode,
    cols: &Vec<Col>,
    diff: &Option<&DiffCols>,
) {
    for detail in app_details.dependency_details.iter() {
        let mut row_vec = Vec::new();
        for col in cols.to_vec().iter() {
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
                if !version_re.is_match(&pjson_version) {
                    for cell in row.iter_mut() {
                        cell.style(Attr::ForegroundColor(color::BLUE));
                    }
                }
            }
        }
    }
}

fn print_completion_message(schema: &Schema) -> Result<(), Error> {
    let Schema {
        message,
        app_details,
        ..
    } = schema;
    let mut buffer = Vec::new();
    writeln!(
        &mut buffer,
        "\n{} matches found in version {} of {}.\n{}\n",
        app_details.dependency_details.len(),
        app_details.version,
        app_details.name,
        message
    )?;
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(buffer.as_mut_slice())?;
    Ok(())
}
