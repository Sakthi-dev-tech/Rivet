use std::{env, fs, path::Path};

use owo_colors::OwoColorize;
use termtree::Tree;

fn print_collections(collections_path: &Path) -> std::io::Result<()> {
    let mut root = Tree::new("Your Collection".to_string());

    let mut entries = fs::read_dir(collections_path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        if let Some(tree) = build_collection_tree(
            &entry.path(),
            entry.file_name().to_string_lossy().to_string(),
        )? {
            root.push(tree);
        }
    }

    println!("{root}");
    Ok(())
}

fn build_collection_tree(path: &Path, name: String) -> std::io::Result<Option<Tree<String>>> {
    let file_type = fs::metadata(path)?.file_type();

    if file_type.is_file() {
        if path.extension().is_some_and(|ext| ext == "toml") {
            let name = path
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
                .unwrap_or(name);
            return Ok(Some(Tree::new(name)));
        }

        return Ok(None);
    }

    if !file_type.is_dir() {
        return Ok(None);
    }

    let mut tree = Tree::new(name);
    let mut entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        if let Some(child_tree) = build_collection_tree(
            &entry.path(),
            entry.file_name().to_string_lossy().to_string(),
        )? {
            tree.push(child_tree);
        }
    }

    if tree.leaves.is_empty() {
        Ok(None)
    } else {
        Ok(Some(tree))
    }
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
