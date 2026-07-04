use std::{env, fs, path::Path};

use owo_colors::OwoColorize;
use termtree::Tree;

fn print_collections(collections_path: &Path) -> std::io::Result<()> {
    let mut root = Tree::new("Your Collection".to_string());

    // Collect all folders within the collections folder into a vector
    // and sort them by their name
    let mut folders = fs::read_dir(collections_path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .collect::<Vec<_>>();

    folders.sort_by_key(|entry| entry.file_name());

    // for each folder, we extract all the toml files and push it onto the tree
    for folder in folders {
        let mut folder_tree = Tree::new(folder.file_name().to_string_lossy().to_string());

        let mut toml_files = fs::read_dir(folder.path())?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.file_type().map(|t| t.is_file()).unwrap_or(false)
                    && entry.path().extension().is_some_and(|ext| ext == "toml")
            })
            .collect::<Vec<_>>();

        toml_files.sort_by_key(|entry| entry.file_name());

        for file in toml_files {
            folder_tree.push(file.file_name().to_string_lossy().to_string());
        }

        if !folder_tree.leaves.is_empty() {
            root.push(folder_tree);
        }
    }

    println!("{root}");
    Ok(())
}

pub fn ls_function() -> Result<(), ()> {
    if let Ok(current_path) = env::current_dir() {
        let rivet_path = current_path.join(".rivet");
        let collections_path = rivet_path.join("collections");

        if let Err(err) = print_collections(&collections_path) {
            println!("Error reading collections: {}", err.red());
            return Err(());
        }
    } else {
        println!("{}", "Error getting current directory".red());
        return Err(());
    };

    Ok(())
}
