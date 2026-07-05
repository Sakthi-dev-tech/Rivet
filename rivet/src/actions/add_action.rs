use owo_colors::OwoColorize;
use std::{env, fs};

use super::request_path::resolve_request_file_path;

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

pub fn add_function(path: &str) -> Result<(), String> {
    if let Ok(current_path) = env::current_dir() {
        let file_path = resolve_request_file_path(&current_path, path)?;
        let parent_path = file_path
            .parent()
            .ok_or_else(|| "Invalid request path".to_string())?;

        if let Err(error) = fs::create_dir_all(parent_path) {
            return Err(format!(
                "Something went wrong when creating collection: {}",
                error.red()
            ));
        };

        if let Err(error) = fs::write(file_path, DEFAULT_REQUEST_TOML) {
            return Err(format!(
                "Something went wrong when making file: {}",
                error.red()
            ));
        };
    } else {
        return Err(format!("{}", "Error getting current directory".red()));
    };

    println!("{}", "Successfully created your TOML file!".green());
    Ok(())
}
