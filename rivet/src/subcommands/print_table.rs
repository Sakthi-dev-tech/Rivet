use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table, presets::UTF8_FULL};
use std::time::Duration;

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

pub fn print_response_table(
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

pub fn print_error_table(method: &str, url: &str, duration: Duration, error: &reqwest::Error) {
    let mut table = Table::new();

    let error_message = if error.is_timeout() {
        "Request timed out. Increase [config] timeout or check the server.".to_string()
    } else {
        error.to_string()
    };

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
        .add_row(vec![Cell::new("Error"), Cell::new(error_message)]);

    println!("{table}");
}
