use owo_colors::OwoColorize;
use std::{env, fs};

pub fn init_function() -> Result<(), String> {
    let current_path = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            return Err(format!("Error getting current directory: {}", err.red()));
        }
    };

    let rivet_path = current_path.join(".rivet");

    let dirs = [rivet_path.join("collections/"), rivet_path.join("history/")];

    let files = [rivet_path.join("config.toml")];

    for dir in dirs {
        if let Err(err) = fs::create_dir_all(&dir) {
            return Err(format!(
                "Error creating {}: {}",
                dir.display(),
                err.bright_red()
            ));
        }
    }

    for file in files {
        if !file.exists() {
            if let Err(err) = fs::write(&file, "") {
                return Err(format!("Error creating {}: {}", file.display(), err.red()));
            }
        }
    }

    println!("{}", "Initialised successfully!".green());
    Ok(())
}
