use crate::types::dependency_detail::DepDetail;
use crate::types::pjson_detail::PjsonDetail;
use regex::Regex;
use std::io::Error;
use std::path::PathBuf;

pub struct Args {
    pub filter: String,
    pub path: PathBuf,
    pub direct_deps: bool,
    pub global: bool,
}

/// Holds the name and version values from the package.json files.
pub struct AppDetail {
    pub pjson: Option<PjsonDetail>,
    pub dependency_details: Vec<DepDetail>,
    pub args: Args,
}

impl AppDetail {
    /// Returns a new AppDetail type.
    pub fn new(args: Args) -> Result<AppDetail, Error> {
        let pjson_details = match args.global {
            true => None,
            false => Some(PjsonDetail::new(&args.path)?),
        };
        let dependency_details = Vec::new();

        let mut new_app = AppDetail {
            pjson: pjson_details,
            dependency_details,
            args,
        };
        let mut base_path = PathBuf::from(&new_app.args.path);
        base_path.push("node_modules");

        new_app.collect_dependencies(&base_path)?;

        Ok(new_app)
    }

    /// Loops through the node_modules directory and pushes the details into a Vec.
    fn collect_dependencies(&mut self, base_path: &PathBuf) -> Result<(), Error> {
        let node_modules = base_path.read_dir()?;
        let name_filter = Regex::new(&self.args.filter).unwrap();

        for entry in node_modules {
            if let Ok(entry) = entry {
                let folder_name = entry.file_name().into_string().unwrap();

                if folder_name.starts_with('.') {
                    continue;
                }
                // let mut dep_path = PathBuf::from(&base_path);
                let mut dep_path = base_path.clone();
                dep_path.push(&folder_name);

                if folder_name.starts_with('@') {
                    AppDetail::collect_dependencies(self, &dep_path)?;
                } else {
                    match DepDetail::new(&dep_path, self) {
                        Ok(detail) => {
                            if self.filter_by_name(&detail, &name_filter) && self.filter_by_flags(&detail) {
                                self.dependency_details.push(detail);
                            }
                        }
                        Err(err) => println!("Error getting {:?}: {}", dep_path, err),
                    }
                }
            }
        }

        self.dependency_details.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(())
    }

    fn filter_by_name(&self, detail: &DepDetail, name_filter: &Regex) -> bool {
        name_filter.is_match(&detail.name)
    }

    fn filter_by_flags(&self, detail: &DepDetail) -> bool {
        let Args { direct_deps, .. } = self.args;

        if direct_deps && detail.pjson_version.is_none() {
            return false;
        }

        true
    }
}
