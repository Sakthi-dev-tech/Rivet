use std::path::{Path, PathBuf};

pub fn resolve_request_file_path(
    current_path: &Path,
    request_path: &str,
) -> Result<PathBuf, String> {
    let request_path = request_path.trim();

    if request_path.is_empty() {
        return Err("Request path cannot be empty".to_string());
    }

    if request_path.ends_with(".toml") {
        return Err("Request path should not include the .toml extension".to_string());
    }

    if request_path.contains('\\') {
        return Err("Request path must use / as the separator".to_string());
    }

    let segments = request_path.split('/').collect::<Vec<_>>();

    if segments.len() < 2 {
        return Err("Request path must use collection/name format".to_string());
    }

    if segments
        .iter()
        .any(|segment| segment.is_empty() || *segment == "." || *segment == "..")
    {
        return Err("Request path contains an invalid segment".to_string());
    }

    let mut file_path = current_path.join(".rivet").join("collections");

    for segment in &segments[..segments.len() - 1] {
        file_path = file_path.join(segment);
    }

    let name = segments
        .last()
        .ok_or_else(|| "Request path must use collection/name format".to_string())?;
    file_path = file_path.join(format!("{name}.toml"));

    Ok(file_path)
}
