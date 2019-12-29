use crate::types::application_detail::AppDetail;
use crate::types::dependency_detail::DepValue;

pub enum Schematic<'a> {
    Plain(&'a AppDetail),
    Direct(&'a AppDetail),
    // Diff,
}

#[derive(PartialEq)]
pub enum Mode {
    PlainList,
    DirectDepsList,
}

#[derive(Clone)]
pub struct Col(pub &'static str, pub DepValue);

pub struct DiffCols<'a>(Vec<Col>, &'a AppDetail);

pub struct Schema<'a> {
    pub app_details: &'a AppDetail,
    pub mode: Mode,
    pub cols: Vec<Col>,
    pub diff: Option<&'a DiffCols<'a>>,
    pub message: &'static str,
}

impl<'a> Schema<'a> {
    pub fn new(schematic: Schematic) -> Schema {
        match schematic {
            Schematic::Direct(app_details) => Schema {
                app_details,
                mode: Mode::DirectDepsList,
                cols: vec![
                    Col("Package", DepValue::Name),
                    Col("Type", DepValue::DepType),
                    Col("PJSON", DepValue::PjsonVersion),
                    Col("Version", DepValue::Version),
                ],
                diff: None,
                message:
                    "Packages which have resolved to a different version are highlighted in blue.",
            },
            Schematic::Plain(app_details) => Schema {
                app_details,
                mode: Mode::PlainList,
                cols: vec![
                    Col("Package", DepValue::Name),
                    Col("Version", DepValue::Version),
                ],
                diff: None,
                message:
                    "Direct dependencies (listed in the package.json) are highlighted in blue.",
            },
        }
    }
}
