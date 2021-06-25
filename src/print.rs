use super::types::dependency_detail::{DepDetail, DepKey, DepTuple, DepType};
use super::types::output_schema::{Col, Mode, Schema};
use prettytable::{color, Attr, Cell, Row, Table};
use regex::Regex;
use std::io::{self, Error, Write};

#[derive(Debug, Clone)]
struct Dependency<'a>(pub &'a str, pub Option<&'a DepDetail>, pub Option<DepDetail>);

impl<'a> Dependency<'a> {
    pub fn get_record_str(self: &Self, dep_tuple: &DepTuple) -> &str {
        let (dep_data, dep_key) = match dep_tuple {
            DepTuple::Main(key) => {
                let Dependency(_, data, _) = self;
                (data, key)
            }
            DepTuple::Diff(key) => {
                let Dependency(_, _, data) = self;
                (data, key)
            }
        };

        if let Some(detail) = dep_data {
            match dep_key {
                DepKey::Version => &detail.version,
                DepKey::DepType => match detail.dep_type {
                    DepType::Dependency => "dep",
                    DepType::DevDependency => "dev",
                    DepType::PeerDependency => "peer",
                    DepType::ChildDependency => "child",
                },
                DepKey::PjsonVersion => {
                    if let Some(pjson_version) = &detail.pjson_version {
                        pjson_version
                    } else {
                        ""
                    }
                }
            }
        } else {
            ""
        }
    }
}

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

fn munge_dependencies<'a>(schema: &'a Schema) -> Vec<Dependency<'a>> {
    let Schema {
        app_details, diff, ..
    } = schema;
    let mut munged_deps = app_details
        .dependency_details
        .iter()
        .map(|dep| Dependency(&dep.name, Some(dep.to_owned()), None))
        .collect();

    match diff {
        None => munged_deps,
        Some(diff_app_details) => {
            let mut new_dependencies = Vec::new();
            for diff_app_dep_detail in diff_app_details.dependency_details.iter() {
                let mut found = false;
                for (i, munged_dep) in munged_deps.to_vec().iter().enumerate() {
                    let Dependency(name, app_dep, _) = munged_dep;
                    if let Some(dep) = app_dep {
                        if dep.name == diff_app_dep_detail.name {
                            munged_deps[i] = Dependency(
                                name,
                                Some(dep.to_owned()),
                                Some(diff_app_dep_detail.to_owned()),
                            );
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    new_dependencies.push(Dependency(
                        &diff_app_dep_detail.name,
                        None,
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

    let mut row_vec = vec![Cell::new("Name")];
    for col in cols.iter() {
        let Col(title, _) = col;
        row_vec.push(Cell::new(title));
    }

    let row = Row::new(row_vec);
    table.add_row(row);
}

fn add_table_rows(table: &mut Table, schema: &Schema, dependencies: Vec<Dependency>) {
    let Schema { mode, cols, .. } = schema;
    for dependency in dependencies.iter() {
        let Dependency(name, ..) = dependency;
        let mut row_vec = vec![Cell::new(name)];

        for col in cols.to_vec().iter() {
            let Col(_, record) = col;
            row_vec.push(Cell::new(dependency.get_record_str(record)));
        }
        let mut row = Row::new(row_vec);
        format_row(&mut row, &mode, dependency);
        table.add_row(row);
    }
}

fn format_row(row: &mut Row, mode: &Mode, dependency: &Dependency) {
    match mode {
        Mode::PlainList => {
            let Dependency(_, app_dep_details, _) = dependency;
            if let Some(dep_details) = app_dep_details {
                if dep_details.pjson_version != None {
                    change_row_fg_color(row, color::BLUE);
                }
            }
        }
        Mode::DirectDepsList => {
            let Dependency(_, app_dep_details, _) = dependency;
            if let Some(dep_details) = app_dep_details {
                if let Some(pjson_version) = &dep_details.pjson_version {
                    let version_re = Regex::new(&dep_details.version).unwrap();
                    if !version_re.is_match(&pjson_version) {
                        change_row_fg_color(row, color::BLUE);
                    }
                }
            }
        }
        Mode::DiffList => match dependency {
            Dependency(_, Some(dep_details), Some(diff_dep_details)) => {
                if dep_details.version != "" && dep_details.version != diff_dep_details.version {
                    change_row_fg_color(row, color::BLUE);
                }
            }
            Dependency(_, Some(_), None) => change_row_fg_color(row, color::RED),
            Dependency(_, None, Some(_)) => change_row_fg_color(row, color::GREEN),
            Dependency(_, None, None) => (),
        },
    }
}

fn change_row_fg_color(row: &mut Row, color: color::Color) {
    for cell in row.iter_mut() {
        cell.style(Attr::ForegroundColor(color));
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
