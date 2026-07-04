use owo_colors::OwoColorize;
use std::{env, fs};

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

pub fn add_function(name: &String, collection: &String) -> Result<(), String> {
    if let Ok(current_path) = env::current_dir() {
        let collection_path = current_path.join(format!(".rivet/collections/{}", collection));

        if let Err(error) = fs::create_dir_all(&collection_path) {
            return Err(format!(
                "Something went wrong when creating collection: {}",
                error.red()
            ));
        };

        if let Err(error) = fs::write(
            collection_path.join(format!("{}.toml", name)),
            DEFAULT_REQUEST_TOML,
        ) {
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
