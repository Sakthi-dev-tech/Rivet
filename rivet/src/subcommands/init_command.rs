use std::{env, fs};
use owo_colors::OwoColorize;

pub fn init_function() -> Result<(), ()> {
    let current_path = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            println!("Error getting current directory: {}", err.red());
            return Err(());
        }
    };

    let rivet_path = current_path.join(".rivet");

    let dirs = [
        rivet_path.join("collections/"),
        rivet_path.join("history/"),
    ];

    let files = [
        rivet_path.join("config.toml")
    ];

    for dir in dirs {
        if let Err(err) = fs::create_dir_all(&dir) {
            println!("Error creating {}: {}", dir.display(), err.bright_red());
            return Err(());
        }
    }

    for file in files {
        if !file.exists() {
            if let Err(err) = fs::write(&file, "") {
                println!("Error creating {}: {}", file.display(), err.red());
                return Err(());
            }
        }
    }

    println!("{}", "Initialised successfully!".green());
    Ok(())
}
