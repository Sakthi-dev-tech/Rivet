use std::time::Duration;

use reqwest::StatusCode;

pub struct ApiResponse {
    pub method: String,
    pub url: String,
    pub status: StatusCode,
    pub elapsed: Duration,
    pub content_type: String,
    pub text: String,
}
