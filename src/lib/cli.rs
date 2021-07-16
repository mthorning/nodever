use std::path::PathBuf;
use structopt::StructOpt;
use once_cell::sync::OnceCell;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Cli {
    /// The pattern to filter folders in node_modules.
    #[structopt(default_value = ".*")]
    pub filter: String,

    /// The path to node_modules folder.
    #[structopt(long, parse(from_os_str), default_value = ".")]
    pub path: PathBuf,

    #[structopt(long)]
    pub diff: Option<PathBuf>,

    #[structopt(long)]
    pub required_by: bool,

    /// Search in global dependencies.
    #[structopt(long, short = "g")]
    pub global: bool,

    /// Show dependencies.
    #[structopt(long, short = "D" )] 
    pub dep: bool,

    /// Show devDependencies.
    #[structopt(long, short = "d")]
    pub dev: bool,

    /// Include build metadata in versions
    #[structopt(long, short = "m")]
    pub meta: bool,
}

static INSTANCE: OnceCell<Cli> = OnceCell::new();

impl Cli {
    pub fn get() -> &'static Cli {
        INSTANCE.get_or_init(|| {
            Cli::from_args()
        })
    }
}
