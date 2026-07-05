use owo_colors::OwoColorize;
use std::{env, fs};

use super::request_path::resolve_request_file_path;

pub fn remove_function(path: &str) -> Result<(), String> {
    if let Ok(current_path) = env::current_dir() {
        let file_path = resolve_request_file_path(&current_path, path)?;

        if !file_path.exists() {
            return Err(format!("{} request config file not found!", path.yellow()));
        }

        if let Err(error) = fs::remove_file(file_path) {
            return Err(format!("Failed to remove file: {}", error.red()));
        };
    } else {
        return Err(format!("{}", "Error getting current directory".red()));
    };

    println!("{}", "Successfully deleted your TOML file!".green());
    Ok(())
}
