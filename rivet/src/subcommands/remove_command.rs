use std::{env, fs};
use owo_colors::OwoColorize;

pub fn remove_function(name: &String, collection: &String) {
    if let Ok(current_path) = env::current_dir() {
        let collection_path = current_path.join(format!(".rivet/collections/{}", collection));

        if !collection_path.exists() {
            println!("{}", "Collection not found".red());
            return;
        }

        let file_path = collection_path.join(format!("{}.toml", name));
        if let Err(error) = fs::remove_file(file_path) {
            println!("Failed to remove file: {}", error.red());
            return;
        };
    };

    println!("{}", "Successfully deleted your TOML file!".green())
}
