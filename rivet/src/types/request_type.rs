use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestConfig {
    pub method: String,
    pub url: String,
    pub params: Option<HashMap<String, String>>,
    pub auth: Option<AuthConfig>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<RequestBody>,
    pub config: Option<Config>,
}

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthConfig {
    Basic {
        username: String,
        password: Option<String>, // Optional if some APIs allow empty passwords
    },
    Bearer {
        token: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub timeout: u64,
}
