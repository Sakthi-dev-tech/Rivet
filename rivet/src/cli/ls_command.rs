use std::env ;

use owo_colors::OwoColorize;

use crate::actions::ls_action::{build_request_tree};

pub fn ls_function() -> Result<(), ()> {
    if let Ok(current_path) = env::current_dir() {
        let rivet_path = current_path.join(".rivet");
        let collections_path = rivet_path.join("collections");

        match build_request_tree(&collections_path) {
            Ok(tree) => {
                println!("{}", tree);
            }
            Err(error) => {
                println!("Error reading collections: {}", error.red());
                return Err(());
            }
        }
    } else {
        println!("{}", "Error getting current directory".red());
        return Err(());
    };

    Ok(())
}
