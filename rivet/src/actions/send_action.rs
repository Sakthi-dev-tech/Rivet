use crate::types::request_type::{AuthConfig, RequestConfig};
use std::env;
use regex::Regex;

// Given a string value, it finds environment placeholders and replace it with the proper value
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

// Given a request config, it goes through each key and calls the resolve_env_placeholders on each
// value
pub fn resolve_request_placeholders(request_config: &mut RequestConfig) -> Result<(), String> {
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
