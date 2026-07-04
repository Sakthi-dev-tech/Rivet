use std::{env, fs};
use owo_colors::OwoColorize;

const DEFAULT_REQUEST_TOML: &str = r#"method = ""
url = ""

[params]
# query = "10"
# sort = "desc"
# filter = "active"

# Choose one authentication method below (uncomment to use):

# --- Bearer Auth ---
# [auth]
# type = "bearer"
# token = "your_token_here"

# --- Basic Auth ---
# [auth]
# type = "basic"
# username = "admin"
# password = "{{AUTH_PASSWORD}}" # this will be your environment variable in .env file

[headers]
# Content-Type = ""
# Authorization = ""
# X-API-Key = ""

[body]
content = """{}"""

[config]
timeout = 30
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
