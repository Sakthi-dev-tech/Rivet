use super::print_table::{print_error_table, print_response_table};
use crate::actions::send_action::send_request;

use owo_colors::OwoColorize;
use std::time::Duration;

pub fn get_response_table(path: &str) -> Result<(), String> {
    let response = match send_request(path) {
        Ok(response) => response,
        Err(error) => {
            print_error_table("unknown", "unknown", Duration::default(), &error);
            return Err(error);
        }
    };

    print_response_table(
        &response.method,
        &response.url,
        response.status,
        response.elapsed,
        &response.content_type,
        response.text.len(),
    );

    let formatted_text = match serde_json::from_str::<serde_json::Value>(&response.text) {
        Ok(json) => match serde_json::to_string_pretty(&json) {
            Ok(pretty_json) => pretty_json,
            Err(_) => response.text,
        },
        Err(_) => response.text,
    };

    if response.status.is_success() {
        println!("{}", formatted_text.green());
    } else {
        return Err(formatted_text);
    }

    Ok(())
}
