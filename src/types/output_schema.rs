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
                    Col("Package", DepTuple::Main(DepKey::Name)),
                    Col("Type", DepTuple::Main(DepKey::DepType)),
                    Col("PJSON", DepTuple::Main(DepKey::PjsonVersion)),
                    Col("Version", DepTuple::Main(DepKey::Version)),
                ],
                diff: None,
                message:
                    "Packages which have resolved to a different version are highlighted in blue.",
            },
            Schematic::Plain(app_details) => Schema {
                app_details,
                mode: Mode::PlainList,
                cols: vec![
                    Col("Package", DepTuple::Main(DepKey::Name)),
                    Col("Version", DepTuple::Main(DepKey::Version)),
                ],
                diff: None,
                message:
                    "Direct dependencies (listed in the package.json) are highlighted in blue.",
            },
            Schematic::Diff(app_details, diff_app_details) => Schema {
                app_details,
                mode: Mode::PlainList,
                cols: vec![
                    Col("Package", DepTuple::Main(DepKey::Name)),
                    Col("Type", DepTuple::Main(DepKey::DepType)),
                    Col("PJSON", DepTuple::Main(DepKey::PjsonVersion)),
                    Col("Version", DepTuple::Main(DepKey::Version)),
                    Col("Type", DepTuple::Diff(DepKey::DepType)),
                    Col("PJSON", DepTuple::Diff(DepKey::PjsonVersion)),
                    Col("Version", DepTuple::Diff(DepKey::Version)),
                ],
                diff: Some(diff_app_details),
                message: "This needs to say something",
            },
        }
    }
}
