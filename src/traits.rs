use std::path::PathBuf;
use crate::types::pjson_detail::PjsonDetail;
use crate::types::cli::Cli;
use std::io::Error;

pub trait NodeModule {
    // fn get_comparison_field(&self) -> String;
    fn print(&self) -> String;
    fn populate(&mut self, base_path: &PathBuf, app_pjson: &PjsonDetail, cli: &Cli) -> Result<(), Error>;
}

