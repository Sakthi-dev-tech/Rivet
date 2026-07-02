use std::{env, fs};
use owo_colors::OwoColorize;

const DEFAULT_REQUEST_TOML: &str = r#"method = ""
url = ""

[headers]
Content-Type = ""
Authorization = ""

[body]
type = "json"
content = ""
"#;

pub fn add_function(name: &String, collection: &String) {
    if let Ok(current_path) = env::current_dir() {
        let collection_path = current_path.join(format!(".rivet/collections/{}", collection));

        if let Err(error) = fs::create_dir_all(&collection_path) {
            println!("Something went wrong when creating collection: {}", error.red());
            return;
        };

        if let Err(error) = fs::write(collection_path.join(format!("{}.toml", name)), DEFAULT_REQUEST_TOML) {
            println!("Something went wrong when making file: {}", error.red());
            return;
        };
    };

    println!("{}", "Successfully created your TOML file!".green())
}
