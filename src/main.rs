use exitfailure::ExitFailure;

use library::{Cli, PjsonDetail, GlobalModule, StandardModule, DiffedPair};

fn main() -> Result<(), ExitFailure> {
    let cli = Cli::get();


    if cli.global {
        let base_path = library::get_node_modules_path(&library::get_global_path());
        let mut dependencies = Vec::<GlobalModule>::new();
        library::collect_dependencies(&base_path, &mut dependencies, None)?;
        library::print_table(&dependencies);
    } else {
        let base_path = library::get_node_modules_path(&cli.path);
        let app_pjson = PjsonDetail::from(&cli.path)?;
            let mut dependencies = Vec::<StandardModule>::new();
            library::collect_dependencies(&base_path, &mut dependencies, Some(&app_pjson))?;
        
        if let Some(path) = &cli.diff {
            let diff_path = library::get_node_modules_path(&path);
            let diff_pjson = PjsonDetail::from(&path)?;
            let mut diff_dependencies = Vec::<StandardModule>::new();
            library::collect_dependencies(&diff_path, &mut diff_dependencies, Some(&diff_pjson))?;
            let diffed_pairs = DiffedPair::get_pairs(&dependencies, &diff_dependencies);
            library::print_table(&diffed_pairs);
        } else {
            library::print_table(&dependencies);
            library::print_completion_message(format!(
                "\n{} matches found in version {} of {}.\n",
                dependencies.len(),
                app_pjson.version,
                app_pjson.name,
            ))?;
        }
    }

    Ok(())
}
