use std::{fs, io, path::Path};

use termtree::Tree;


pub fn build_request_tree(collections_path: &Path) -> io::Result<Tree<String>> {
    let mut root = Tree::new("Your Collection".to_string());

    let mut entries = fs::read_dir(collections_path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        if let Some(tree) = build_tree(
            &entry.path(),
            entry.file_name().to_string_lossy().to_string(),
        )? {
            root.push(tree);
        }
    }

    Ok(root)
}


fn build_tree(path: &Path, name: String) -> io::Result<Option<Tree<String>>> {
    let file_type = fs::metadata(path)?.file_type();

    if file_type.is_file() {
        if path.extension().is_some_and(|ext| ext == "toml") {
            let name = path
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
                .unwrap_or(name);
            return Ok(Some(Tree::new(format!("{}.toml", name))));
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
        if let Some(child_tree) = build_tree(
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
