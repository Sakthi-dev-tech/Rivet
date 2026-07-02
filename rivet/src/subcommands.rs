use std::{env, fs};
use owo_colors::OwoColorize;

pub fn init_function() {
    let current_path = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            panic!("Error getting current directory: {}", err.red());
        }
    };

    let rivet_path = current_path.join(".rivet");

    let dirs = [
        rivet_path.join("requests/"),
        rivet_path.join("history/"),
    ];

    let files = [
        rivet_path.join("config.toml")
    ];

    for dir in dirs {
        if let Err(err) = fs::create_dir_all(&dir) {
            panic!("Error creating {}: {}", dir.display(), err.bright_red());
        }
    }

    for file in files {
        if !file.exists() {
            fs::write(&file, "")
                .expect("Error creating file!");
        }
    }

    println!("{}", "Initialised successfully!".green())
}
