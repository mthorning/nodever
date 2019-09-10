pub mod cli {
    use structopt::StructOpt;
    use std::path::PathBuf;

    #[derive(Debug, StructOpt)]
    pub struct Cli {
        /// The path to the root of the Nexcenter app.
        #[structopt(short, long, parse(from_os_str), default_value=".")]
        pub path: PathBuf,
        /// The pattern to filter folders in node_modules.
        #[structopt(short, long, default_value=".*")]
        pub filter: String,
        /// Sort the results alphabetically
        #[structopt(short, long)]
        pub sort: bool
    }
}

pub mod detail {
    use std::path::PathBuf;
    use std::io::{Read, Error};
    use serde::{Deserialize, Serialize};
    use std::fs::File;

    #[derive(Deserialize, Serialize)]
    struct PjsonDetails {
        name: String,
        version: String,
    }

    /// Holds the name and version values from the package.json files.
    #[derive(Debug)]
    pub struct Detail {
        pub name: String,
        pub version: String,
        pub path: PathBuf,
    }

    impl Detail {
        /// Returns a new Detail type.
        pub fn from(path: PathBuf) -> Result<Detail, Error> {
            let pjson_string = Detail::get_pjson(&path)?;
            let pjson_details: PjsonDetails = serde_json::from_str(&pjson_string[..])?;

            Ok(Detail {
                name: pjson_details.name,
                version: pjson_details.version,
                path,
            })
        }

        /// Returns the data from the package.json file.
        fn get_pjson(path: &PathBuf) -> Result<String, Error> {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }
    }
}

