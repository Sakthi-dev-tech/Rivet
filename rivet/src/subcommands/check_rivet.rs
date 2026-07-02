use std::{env, ops::ControlFlow, path::Path};

use owo_colors::OwoColorize;

fn is_dir(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

fn is_file(path: &Path) -> bool {
    path.exists() && path.is_file()
}

pub fn check_rivet_folder() -> ControlFlow<()> {
    let current_path = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            println!("Error getting current directory: {}", err.red());
            return ControlFlow::Break(());
        }
    };

    let rivet_path = current_path.join(".rivet");

    if !is_dir(&rivet_path) {
        println!(".rivet not found! Run the {} first!", "init command".red());
        return ControlFlow::Break(());
    }

    let required_dirs = [rivet_path.join("collections"), rivet_path.join("history")];
    let required_files = [rivet_path.join("config.toml")];

    let has_missing_dir = required_dirs.iter().any(|path| !is_dir(path));
    let has_missing_file = required_files.iter().any(|path| !is_file(path));

    if has_missing_dir || has_missing_file {
        println!(".rivet is corrupted! Run {} again!", "init command".red());
        return ControlFlow::Break(());
    }

    ControlFlow::Continue(())
}
