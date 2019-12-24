use crate::types::application_detail::AppDetail;
use crate::types::dependency_detail::{DepKey, DepTuple};

pub enum Schematic<'a> {
    Plain(&'a AppDetail),
    Direct(&'a AppDetail),
    Diff(&'a AppDetail, &'a AppDetail),
}

#[derive(PartialEq)]
pub enum Mode {
    PlainList,
    DirectDepsList,
    DiffList,
}

#[derive(Clone)]
pub struct Col(pub &'static str, pub DepTuple);

pub struct Schema<'a> {
    pub app_details: &'a AppDetail,
    pub mode: Mode,
    pub cols: Vec<Col>,
    pub diff: Option<&'a AppDetail>,
    pub message: &'static str,
}

impl<'a> Schema<'a> {
    pub fn new(schematic: Schematic) -> Schema {
        match schematic {
            Schematic::Direct(app_details) => Schema {
                app_details,
                mode: Mode::DirectDepsList,
                cols: vec![
                    Col("Type", DepTuple::Main(DepKey::DepType)),
                    Col("PJSON", DepTuple::Main(DepKey::PjsonVersion)),
                    Col("Version", DepTuple::Main(DepKey::Version)),
                ],
                diff: None,
                message:
                    "\nDependencies which have resolved to a different version are highlighted in blue.",
            },
            Schematic::Plain(app_details) => Schema {
                app_details,
                mode: Mode::PlainList,
                cols: vec![Col("Version", DepTuple::Main(DepKey::Version))],
                diff: None,
                message:
                    "\nDirect dependencies (listed in the package.json) are highlighted in blue.",
            },
            Schematic::Diff(app_details, diff_app_details) => Schema {
                app_details,
                mode: Mode::DiffList,
                cols: vec![
                    Col("Type 1", DepTuple::Main(DepKey::DepType)),
                    Col("PJSON 1", DepTuple::Main(DepKey::PjsonVersion)),
                    Col("Version 1", DepTuple::Main(DepKey::Version)),
                    Col("Type 2", DepTuple::Diff(DepKey::DepType)),
                    Col("PJSON 2", DepTuple::Diff(DepKey::PjsonVersion)),
                    Col("Version 2", DepTuple::Diff(DepKey::Version)),
                ],
                diff: Some(diff_app_details),
                message: "\nDependencies only found in the main app are highlighted in red.\nDependencies only found in the diff app are highlighted in green.\nDependencies found in both apps which are different versions are highlighted in blue.",
            },
        }
    }
}
