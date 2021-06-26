// use crate::types::application_detail::AppDetail;
use crate::types::pjson_detail::PjsonDetail;
use crate::types::cli::Cli;
use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum DepType {
    Dependency(String),
    DevDependency(String),
    PeerDependency(String),
    ChildDependency,
}

// #[derive(Clone)]
// pub enum DepKey {
//     Version,
//     DepType,
//     PjsonVersion,
// }
// #[derive(Clone)]
// pub enum DepTuple {
//     Main(DepKey),
//     Diff(DepKey),
// }

pub trait NodeModule {
    // fn get_comparison_field(&self) -> String;
    fn print(&self) -> String;
    fn populate(&mut self, base_path: &PathBuf, app_pjson: &PjsonDetail, cli: &Cli) -> Result<(), Error>;
}

// pub struct GlobalModule {
//     pub name: String,
//     pub version: String,
// }
// 
// impl NodeModule for GlobalModule {}
// 
// impl GlobalModule {
//     pub fn new(path: &PathBuf, app_pjson: &PjsonDetail) -> Result<GlobalModule, Error> {
//         let PjsonDetail { name, version, .. } = PjsonDetail::new(path)?;
// 
//         let new_dep_detail = GlobalModule {
//             name,
//             version,
//         };
// 
//         Ok(new_dep_detail)
//     }
// }

pub struct StandardModule {
    pub name: String,
    pub version: String,
    pub dep_type: DepType,
}

impl NodeModule for StandardModule {
    fn print(&self) -> String {
        String::from("Hello")
    }

    fn populate(&mut self, path: &PathBuf, app_pjson: &PjsonDetail, _cli: &Cli) -> Result<(), Error> {
        let PjsonDetail { name, version, .. } = PjsonDetail::new(path)?;

        self.dep_type = get_dep_type(&name, app_pjson);

        self.name = name;
        self.version = version;

        Ok(())
    }
}

impl Default for StandardModule {
    fn default() -> Self {
        StandardModule {
            name: String::new(),
            version: String::new(),
            dep_type: DepType::ChildDependency, 
        }
    }
}

fn get_dep_type(name: &str, app_pjson: &PjsonDetail) -> DepType {
    match get_pjson_details(name, &app_pjson.dependencies) {
        Some(required_version) => return DepType::Dependency(required_version),
        None => match get_pjson_details(name, &app_pjson.dev_dependencies) {
            Some(required_version) => return DepType::DevDependency(required_version),
            None => match get_pjson_details(name, &app_pjson.peer_dependencies) {
                Some(required_version) => return DepType::PeerDependency(required_version),
                None => DepType::ChildDependency
            }
        }
    }
}
fn get_pjson_details(
    dep_name: &str,
    required_dependencies: &Option<HashMap<String, String>>,
) -> Option<String> {
    match required_dependencies {
        Some(deps) => match deps.get(dep_name) {
            Some(required_version) => Some(required_version.to_string()),
            None => None,
        },
        None => None,
    }
}
// fn get_comparison_field(&self) -> String {
//    self.name 
// }
