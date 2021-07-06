use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    let cli = library::Cli::get();

    if cli.global {
        library::run_global()?;
    } else if let Some(path) = &cli.diff {
        library::run_diff(&path)?;
    } else {
        library::run_standard()?;
    }

    Ok(())
}
