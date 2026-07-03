use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table, presets::UTF8_FULL};
use owo_colors::OwoColorize;
use serde::Deserialize;
use std::time::Duration;
use std::{collections::HashMap, env, fs};

#[derive(Debug, Deserialize)]
struct RequestConfig {
    method: String,
    url: String,
    params: Option<HashMap<String, String>>,
    auth: Option<AuthConfig>,
    headers: Option<HashMap<String, String>>,
    body: Option<RequestBody>,
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

fn format_duration(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.2} s", duration.as_secs_f64())
    } else {
        format!("{} ms", duration.as_millis())
    }
}

fn format_body_size(size: usize) -> String {
    if size >= 1024 * 1024 {
        format!("{:.2} MB", size as f64 / 1024.0 / 1024.0)
    } else if size >= 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else {
        format!("{} bytes", size)
    }
}

fn print_response_table(
    method: &str,
    url: &str,
    status: reqwest::StatusCode,
    duration: Duration,
    content_type: &str,
    body_size: usize,
) {
    let status_text = format!(
        "{} {}",
        status.as_u16(),
        status.canonical_reason().unwrap_or("Unknown")
    );

    let status_cell = if status.is_success() {
        Cell::new(status_text).fg(Color::Green)
    } else if status.is_client_error() || status.is_server_error() {
        Cell::new(status_text).fg(Color::Red)
    } else {
        Cell::new(status_text).fg(Color::Yellow)
    };

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Metric").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![Cell::new("Method"), Cell::new(method)])
        .add_row(vec![Cell::new("URL"), Cell::new(url)])
        .add_row(vec![Cell::new("Status"), status_cell])
        .add_row(vec![
            Cell::new("Duration"),
            Cell::new(format_duration(duration)),
        ])
        .add_row(vec![Cell::new("Content-Type"), Cell::new(content_type)])
        .add_row(vec![
            Cell::new("Body Size"),
            Cell::new(format_body_size(body_size)),
        ]);

    println!("{table}");
}

fn print_error_table(method: &str, url: &str, duration: Duration, error: &reqwest::Error) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Metric").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![Cell::new("Method"), Cell::new(method)])
        .add_row(vec![Cell::new("URL"), Cell::new(url)])
        .add_row(vec![
            Cell::new("Status"),
            Cell::new("Request Error").fg(Color::Red),
        ])
        .add_row(vec![
            Cell::new("Duration"),
            Cell::new(format_duration(duration)),
        ])
        .add_row(vec![Cell::new("Error"), Cell::new(error.to_string())]);

    println!("{table}");
}

pub fn send_function(name: &String, collection: &String) {
    if let Ok(current_path) = env::current_dir() {
        let collection_path = current_path.join(format!(".rivet/collections/{}", collection));

        if !collection_path.exists() {
            println!("{}", "Collection not found".red());
            return;
        }

        let file_path = collection_path.join(format!("{}.toml", name));

        if !file_path.exists() {
            println!("{}", "TOML file is not found!".red());
            return;
        }

        // Convert file content to raw string
        let file_content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(error) => {
                println!("Failed to read TOML file!: {}", error.red());
                return;
            }
        };

        // Convert the raw string content of file to toml structure for Rust to read
        let request_config: RequestConfig = match toml::from_str(&file_content) {
            Ok(config) => config,
            Err(error) => {
                println!("Failed to parse TOML file: {}", error.red());
                return;
            }
        };

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
                return;
            }
        };

        let mut request_url = request_config.url.clone();

        if let Some(params) = &request_config.params {
            if !params.is_empty() {
                let mut url = match reqwest::Url::parse(&request_config.url) {
                    Ok(url) => url,
                    Err(error) => {
                        println!("Invalid URL: {}", error.red());
                        return;
                    }
                };

                for (key, value) in params {
                    url.query_pairs_mut().append_pair(key, value);
                }

                request_url = url.to_string();
            }
        }

        let client = reqwest::blocking::Client::new();
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
                        }
                    }

                    Err(error) => {
                        let elapsed = started_at.elapsed();

                        print_error_table(&request_config.method, &final_url, elapsed, &error);
                    }
                }
            }

            Err(error) => {
                let elapsed = started_at.elapsed();

                print_error_table(&request_config.method, &request_url, elapsed, &error);
            }
        };
    };
}
