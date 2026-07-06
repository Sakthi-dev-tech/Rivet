use std::{fs, io, path::Path};

pub enum ApiCollectionItem {
    Folder {
        name: String,
        children: Vec<ApiCollectionItem>,
    },

    Request {
        name: String,
        method: String,
        path: String,
    },
}

fn request_path_from_file(collections_path: &Path, path: &Path) -> String {
    let path_without_extension = path.with_extension("");
    let relative_path = path_without_extension
        .strip_prefix(collections_path)
        .unwrap_or(&path_without_extension);

    relative_path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn request_method_from_file(path: &Path) -> String {
    let file_content = fs::read_to_string(path).unwrap_or_default();

    toml::from_str::<toml::Value>(&file_content)
        .ok()
        .and_then(|request| {
            request
                .get("method")
                .and_then(|method| method.as_str())
                .map(str::to_string)
        })
        .filter(|method| !method.trim().is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn build_collection_item(
    collections_path: &Path,
    path: &Path,
    name: String,
) -> io::Result<Option<ApiCollectionItem>> {
    let file_type = fs::metadata(path)?.file_type();

    if file_type.is_file() {
        if path.extension().is_some_and(|ext| ext == "toml") {
            let name = path
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
                .unwrap_or(name);

            return Ok(Some(ApiCollectionItem::Request {
                name,
                method: request_method_from_file(path),
                path: request_path_from_file(collections_path, path),
            }));
        }

        return Ok(None);
    }

    if !file_type.is_dir() {
        return Ok(None);
    }

    let mut children = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(path)?.filter_map(Result::ok).collect();
    entries.sort_by_key(|ent| ent.file_name());

    for entry in entries {
        if let Some(child_item) = build_collection_item(
            collections_path,
            &entry.path(),
            entry.file_name().to_string_lossy().to_string(),
        )? {
            children.push(child_item);
        }
    }

    if children.is_empty() {
        Ok(None)
    } else {
        Ok(Some(ApiCollectionItem::Folder { name, children }))
    }
}

pub fn list_collections_from_path(collections_path: &Path) -> io::Result<Vec<ApiCollectionItem>> {
    let mut items = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(collections_path)?
        .filter_map(Result::ok)
        .collect();

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        if let Some(item) = build_collection_item(
            collections_path,
            &entry.path(),
            entry.file_name().to_string_lossy().to_string(),
        )? {
            items.push(item);
        }
    }

    Ok(items)
}
