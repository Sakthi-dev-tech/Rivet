use super::print_table::{print_error_table, print_response_table};

use owo_colors::OwoColorize;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, env, fs, time::Duration};

const REQUEST_TIMEOUT_SECS: u64 = 30;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestConfig {
    method: String,
    url: String,
    params: Option<HashMap<String, String>>,
    auth: Option<AuthConfig>,
    headers: Option<HashMap<String, String>>,
    body: Option<RequestBody>,
    config: Option<Config>
}

#[derive(Debug, Deserialize)]
struct RequestBody {
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum AuthConfig {
    Basic {
        username: String,
        password: Option<String>, // Optional if some APIs allow empty passwords
    },
    Bearer {
        token: String,
    },
}

#[derive(Debug, Deserialize)]
struct Config {
    timeout: u64
}

fn resolve_env_placeholders(value: &str) -> Result<String, String> {
    let placeholder_regex =
        Regex::new(r"\{\{\s*([A-Za-z_][A-Za-z0-9_]*)\s*\}\}").map_err(|err| err.to_string())?;

    let mut resolved = String::new();
    let mut last_end_idx = 0;

    for capture in placeholder_regex.captures_iter(value) {
        let placeholder = capture
            .get(0)
            .ok_or_else(|| "Invalid environment placeholder".to_string())?;

        let key = capture
            .get(1)
            .ok_or_else(|| "Invalid environment placeholder".to_string())?
            .as_str();

        let env_value =
            env::var(key).map_err(|_| format!("Missing environment variable: {}", key))?;

        resolved.push_str(&value[last_end_idx..placeholder.start()]);
        resolved.push_str(&env_value);
        last_end_idx = placeholder.end();
    }
    resolved.push_str(&value[last_end_idx..]);

    Ok(resolved)
}

fn resolve_request_placeholders(request_config: &mut RequestConfig) -> Result<(), String> {
    request_config.method = resolve_env_placeholders(&request_config.method)?;
    request_config.url = resolve_env_placeholders(&request_config.url)?;

    if let Some(params) = &mut request_config.params {
        for value in params.values_mut() {
            *value = resolve_env_placeholders(value)?;
        }
    }

    if let Some(auth_config) = &mut request_config.auth {
        match auth_config {
            AuthConfig::Basic { username, password } => {
                *username = resolve_env_placeholders(username)?;

                if let Some(password) = password {
                    *password = resolve_env_placeholders(password)?;
                }
            }

            AuthConfig::Bearer { token } => {
                *token = resolve_env_placeholders(token)?;
            }
        }
    }

    if let Some(headers) = &mut request_config.headers {
        for value in headers.values_mut() {
            *value = resolve_env_placeholders(value)?;
        }
    }

    if let Some(body) = &mut request_config.body {
        body.content = resolve_env_placeholders(&body.content)?;
    }

    Ok(())
}

pub fn send_function(name: &String, collection: &String) -> Result<(), ()> {
    if let Ok(current_path) = env::current_dir() {
        let env_path = current_path.join(".env");

        // Check if we can load the dotenv file if it exists
        if env_path.exists() {
            if let Err(error) = dotenvy::from_path(&env_path) {
                println!("Failed to load .env file: {}", error.red());
                return Err(());
            }
        }

        let collection_path = current_path.join(format!(".rivet/collections/{}", collection));

        if !collection_path.exists() {
            println!("{}", "Collection not found".red());
            return Err(());
        }

        let file_path = collection_path.join(format!("{}.toml", name));

        if !file_path.exists() {
            println!("{}", "TOML file is not found!".red());
            return Err(());
        }

        // Convert file content to raw string
        let file_content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(error) => {
                println!("Failed to read TOML file!: {}", error.red());
                return Err(());
            }
        };

        // Convert the raw string content of file to toml structure for Rust to read
        let mut request_config: RequestConfig = match toml::from_str(&file_content) {
            Ok(config) => config,
            Err(error) => {
                println!("Failed to parse TOML file: {}", error.red());
                return Err(());
            }
        };

        if let Err(error) = resolve_request_placeholders(&mut request_config) {
            println!("{}", error.red());
            return Err(());
        }

        let method = match request_config.method.to_uppercase().as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "PATCH" => reqwest::Method::PATCH,
            "DELETE" => reqwest::Method::DELETE,
            "HEAD" => reqwest::Method::HEAD,
            "OPTIONS" => reqwest::Method::OPTIONS,
            method => {
                println!("Invalid HTTP method: {}", method.red());
                return Err(());
            }
        };

        let mut request_url = request_config.url.clone();

        if let Some(params) = &request_config.params {
            if !params.is_empty() {
                let mut url = match reqwest::Url::parse(&request_config.url) {
                    Ok(url) => url,
                    Err(error) => {
                        println!("Invalid URL: {}", error.red());
                        return Err(());
                    }
                };

                for (key, value) in params {
                    url.query_pairs_mut().append_pair(key, value);
                }

                request_url = url.to_string();
            }
        }

        let timeout_secs = request_config
            .config
            .as_ref()
            .map(|config| u64::from(config.timeout))
            .unwrap_or(REQUEST_TIMEOUT_SECS);

        let client = match reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
        {
            Ok(client) => client,
            Err(error) => {
                println!("Failed to create HTTP client: {}", error.red());
                return Err(());
            }
        };

        let mut request = client.request(method, &request_url);

        if let Some(auth_config) = request_config.auth {
            request = match auth_config {
                AuthConfig::Basic { username, password } => request.basic_auth(username, password),

                AuthConfig::Bearer { token } => request.bearer_auth(token),
            }
        }

        if let Some(headers) = request_config.headers {
            for (key, value) in headers {
                if !value.is_empty() {
                    request = request.header(key, value);
                }
            }
        }

        if let Some(body) = request_config.body {
            if !body.content.trim().is_empty() && request_config.method.to_uppercase() != "GET" {
                request = request.body(body.content);
            }
        }

        let started_at = std::time::Instant::now();
        match request.send() {
            Ok(response) => {
                let status = response.status();
                let final_url = response.url().to_string();
                let headers = response.headers().clone();
                let content_type = headers
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|val| val.to_str().ok())
                    .unwrap_or("unknown")
                    .to_string();

                match response.text() {
                    Ok(text) => {
                        let elapsed = started_at.elapsed();

                        print_response_table(
                            &request_config.method,
                            &final_url,
                            status,
                            elapsed,
                            &content_type,
                            text.len(),
                        );

                        let formatted_text = match serde_json::from_str::<serde_json::Value>(&text)
                        {
                            Ok(json) => match serde_json::to_string_pretty(&json) {
                                Ok(pretty_json) => pretty_json,
                                Err(_) => text,
                            },
                            Err(_) => text,
                        };

                        if status.is_success() {
                            println!("{}", formatted_text.green());
                        } else {
                            println!("Request failed with status code: {}", status.red());
                            println!("{}", formatted_text);
                            return Err(());
                        }
                    }

                    Err(error) => {
                        let elapsed = started_at.elapsed();

                        print_error_table(&request_config.method, &final_url, elapsed, &error);
                        return Err(());
                    }
                }
            }

            Err(error) => {
                let elapsed = started_at.elapsed();

                print_error_table(&request_config.method, &request_url, elapsed, &error);
                return Err(());
            }
        };
    } else {
        println!("{}", "Error getting current directory".red());
        return Err(());
    };

    Ok(())
}
