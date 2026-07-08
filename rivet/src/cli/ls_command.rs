use std::env;

use owo_colors::OwoColorize;
use termtree::Tree;

use crate::actions::ls_action::{ApiCollectionItem, list_collections_from_path};

fn collection_item_to_tree(item: ApiCollectionItem) -> Tree<String> {
    match item {
        ApiCollectionItem::Folder { name, children } => {
            let mut tree = Tree::new(name);

            for child in children {
                tree.push(collection_item_to_tree(child));
            }

            tree
        }
        ApiCollectionItem::Request { name, method, path } => {
            let method = method
                .map(|method| format!("{:?}", method))
                .unwrap_or_else(|| "Unknown".to_string());

            Tree::new(format!("{} {} ({})", method, name, path))
        }
    }
}

pub fn ls_function() -> Result<(), ()> {
    if let Ok(current_path) = env::current_dir() {
        let rivet_path = current_path.join(".rivet");
        let collections_path = rivet_path.join("collections");

        match list_collections_from_path(&collections_path) {
            Ok(collections) => {
                let mut tree = Tree::new("Your Collections".to_string());

                for collection in collections {
                    tree.push(collection_item_to_tree(collection));
                }

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
