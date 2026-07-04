use owo_colors::OwoColorize;
use std::{env, fs};

pub fn remove_function(name: &String, collection: &String) -> Result<(), String> {
    if let Ok(current_path) = env::current_dir() {
        let collection_path = current_path.join(format!(".rivet/collections/{}", collection));

        if !collection_path.exists() {
            return Err(format!("{}", "Collection not found".red()));
        }

        let file_path = collection_path.join(format!("{}.toml", name));
        if let Err(error) = fs::remove_file(file_path) {
            return Err(format!("Failed to remove file: {}", error.red()));
        };
    } else {
        return Err(format!("{}", "Error getting current directory".red()));
    };

    println!("{}", "Successfully deleted your TOML file!".green());
    Ok(())
}
