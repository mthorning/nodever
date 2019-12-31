use super::types::application_detail::AppDetail;
use super::types::dependency_detail::{DepDetail, DepType, Dependency};
use super::types::output_schema::{Col, Mode, Schema};
use prettytable::{color, Attr, Cell, Row, Table};
use regex::Regex;
use std::io::{self, Error, Write};

pub fn print_details(schema: Schema) -> Result<(), Error> {
    print_table(&schema);
    print_completion_message(&schema)?;
    Ok(())
}

fn print_table(schema: &Schema) {
    if !schema.app_details.dependency_details.is_empty() {
        let mut table = Table::new();
        let dependencies = munge_dependencies(schema);
        add_table_headers(&mut table, schema);
        add_table_rows(&mut table, schema, dependencies);
        table.printstd();
    }
}

fn munge_dependencies(schema: &Schema) -> Vec<Dependency> {
    let Schema {
        app_details, diff, ..
    } = schema;
    let mut munged_deps = app_details
        .dependency_details
        .to_vec()
        .into_iter()
        .map(|dep| Dependency(Some(dep), None))
        .collect();

    match diff {
        None => munged_deps,
        Some(diff_app_details) => {
            let mut new_dependencies = Vec::new();
            for diff_app_dep_detail in diff_app_details.dependency_details.iter() {
                let mut found = false;
                for (i, munged_dep) in munged_deps.to_vec().iter().enumerate() {
                    let Dependency(app_dep, _) = munged_dep;
                    if let Some(dep) = app_dep {
                        if dep.name == diff_app_dep_detail.name {
                            munged_deps[i] = Dependency(
                                Some(dep.to_owned()),
                                Some(diff_app_dep_detail.to_owned()),
                            );
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    let name_only = DepDetail {
                        name: diff_app_dep_detail.name.clone(),
                        version: String::new(),
                        dep_type: DepType::None,
                        pjson_version: None,
                    };
                    new_dependencies.push(Dependency(
                        Some(name_only),
                        Some(diff_app_dep_detail.to_owned()),
                    ));
                }
            }
            [&munged_deps[..], &new_dependencies[..]].concat()
        }
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

fn add_table_rows(table: &mut Table, schema: &Schema, dependencies: Vec<Dependency>) {
    let Schema {
        mode, cols, diff, ..
    } = schema;
    for dependency in dependencies.iter() {
        let mut row_vec = Vec::new();
        let Dependency(app_dep_details, diff_dep_details) = dependency;

        if let Some(dep_details) = app_dep_details {
            for col in cols.to_vec().iter() {
                let Col(_, record) = col;
                row_vec.push(Cell::new(dependency.get_record_str(record)));
            }
            let mut row = Row::new(row_vec);
            format_row(&mut row, &mode, dep_details, diff_dep_details);
            table.add_row(row);
        }
    }
}

fn format_row(
    row: &mut Row,
    mode: &Mode,
    dep_details: &DepDetail,
    diff_dep_details: &Option<DepDetail>,
) {
    match mode {
        Mode::PlainList => {
            if dep_details.pjson_version != None {
                for cell in row.iter_mut() {
                    cell.style(Attr::ForegroundColor(color::BLUE));
                }
            }
        }
        Mode::DirectDepsList => {
            if let Some(pjson_version) = &dep_details.pjson_version {
                let version_re = Regex::new(&dep_details.version).unwrap();
                if !version_re.is_match(&pjson_version) {
                    for cell in row.iter_mut() {
                        cell.style(Attr::ForegroundColor(color::BLUE));
                    }
                }
            }
        }
        Mode::DiffList => {
            if let Some(diff_deps) = diff_dep_details {
                if dep_details.version != "" && dep_details.version != diff_deps.version {
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
