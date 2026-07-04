use super::print_table::{print_error_table, print_response_table};
use crate::types::request_type::{AuthConfig, RequestConfig};
use crate::actions::send_action::{resolve_request_placeholders};

use owo_colors::OwoColorize;
use std::{env, fs, time::Duration};

const REQUEST_TIMEOUT_SECS: u64 = 30;

pub fn send_function(name: &String, collection: &String) -> Result<(), String> {
    if let Ok(current_path) = env::current_dir() {
        let env_path = current_path.join(".env");

        // Check if we can load the dotenv file if it exists
        if env_path.exists() {
            if let Err(error) = dotenvy::from_path(&env_path) {
                return Err(format!("Failed to load .env file: {}", error.red()));
            }
        }

        let collection_path = current_path.join(format!(".rivet/collections/{}", collection));

        if !collection_path.exists() {
            return Err(format!("{}", "Collection not found".red()));
        }

        let file_path = collection_path.join(format!("{}.toml", name));

        if !file_path.exists() {
            return Err(format!("{}", "TOML file is not found!".red()));
        }

        // Convert file content to raw string
        let file_content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(error) => {
                return Err(format!("Failed to read TOML file!: {}", error.red()));
            }
        };

        // Convert the raw string content of file to toml structure for Rust to read
        let mut request_config: RequestConfig = match toml::from_str(&file_content) {
            Ok(config) => config,
            Err(error) => {
                return Err(format!("Failed to parse TOML file: {}", error.red()));
            }
        };

        if let Err(error) = resolve_request_placeholders(&mut request_config) {
            return Err(format!("{}", error.red()));
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
                return Err(format!("Invalid HTTP method: {}", method.red()));
            }
        };

        let mut request_url = request_config.url.clone();

        if let Some(params) = &request_config.params {
            if !params.is_empty() {
                let mut url = match reqwest::Url::parse(&request_config.url) {
                    Ok(url) => url,
                    Err(error) => {
                        return Err(format!("Invalid URL: {}", error.red()));
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
                return Err(format!("Failed to create HTTP client: {}", error.red()));
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
                            return Err(format!("{}", formatted_text));
                        }
                    }

                    Err(error) => {
                        let elapsed = started_at.elapsed();

                        print_error_table(&request_config.method, &final_url, elapsed, &error);
                        return Err(error.to_string());
                    }
                }
            }

            Err(error) => {
                let elapsed = started_at.elapsed();

                print_error_table(&request_config.method, &request_url, elapsed, &error);
                return Err(error.to_string());
            }
        };
    } else {
        return Err(format!("{}", "Error getting current directory".red()));
    };

    Ok(())
}
