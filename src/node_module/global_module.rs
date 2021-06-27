use std::io::Error;
use std::path::PathBuf;

use regex::Regex;

use crate::pjson_detail::PjsonDetail;
use crate::cli::Cli;
use crate::node_module::*;

pub struct GlobalModule {
    pub name: String,
    pub version: String,
}

impl NodeModule for GlobalModule {
    fn filter_by_regex(&self, re: &Regex) -> bool {
        re.is_match(&self.name)
    }

    fn populate(&mut self, path: &PathBuf, _cli: &Cli, _app_pjson: Option<&PjsonDetail>) -> Result<(), Error> {

        let PjsonDetail { name, version, .. } = match PjsonDetail::new(path) {
            Ok(pjson_details) => pjson_details,
            Err(err) => {
                println!("Failed to find a package.json in {:?}", path);
                return Err(err);
            },
        };

        self.name = name;
        self.version = version;

        Ok(())
    }

    fn order(&self, to_compare: &GlobalModule) -> Ordering {
        self.name.cmp(&to_compare.name)
    }

    fn table_row(&self, _row_type: RowType) -> Row {
        row![self.name, self.version]
    }
}

impl Default for GlobalModule {
    fn default() -> Self {
        GlobalModule {
            name: String::new(),
            version: String::new(),
        }
    }
}
