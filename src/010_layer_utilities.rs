//! Layer utility functions for layer-based dependency analysis
//! This module is at layer 005 to be accessible from all higher layers

use std::path::{Path, PathBuf};

/// Extracts numeric layer prefix from a layer string (e.g., "060_file_ordering" -> 60)
pub fn layer_prefix_value(layer: &str) -> Option<i32> {
    let mut chars = layer.chars();
    let mut digits = String::new();
    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else {
            break;
        }
    }
    if digits.is_empty() {
        None
    } else {
        digits.parse::<i32>().ok()
    }
}

/// Checks if a dependency from one layer to another violates layer ordering
/// Returns true if from_layer > to_layer (violation: higher depends on lower)
pub fn is_layer_violation(from: &str, to: &str) -> bool {
    match (layer_prefix_value(from), layer_prefix_value(to)) {
        (Some(a), Some(b)) => a > b,
        _ => false,
    }
}

/// Detects the layer identifier from a path by finding first digit-prefixed component
pub fn detect_layer(path: &Path) -> String {
    for component in path.components() {
        if let Some(name) = component.as_os_str().to_str() {
            if let Some(first) = name.chars().next() {
                if first.is_ascii_digit() {
                    if let Some(pos) = name.find('_') {
                        if name[..pos].chars().all(|c| c.is_ascii_digit()) {
                            return name.to_string();
                        }
                    }
                }
            }
        }
    }
    "root".to_string()
}

/// Checks if path contains 'tools' directory component
pub fn contains_tools(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == "tools")
}

/// Resolves the source root directory from a given root path
pub fn resolve_source_root(root: &Path) -> PathBuf {
    let src_candidate = root.join("src");
    if src_candidate.exists() && src_candidate.is_dir() {
        src_candidate
    } else {
        root.to_path_buf()
    }
}

/// Checks if a directory should be included in analysis
pub fn allow_analysis_dir(root: &Path, dir: &Path) -> bool {
    let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
    
    if name.starts_with('.') || name == "target" || name == "node_modules" {
        return false;
    }
    
    if let Ok(rel) = dir.strip_prefix(root) {
        if rel.components().any(|c| {
            let s = c.as_os_str().to_str().unwrap_or("");
            s.starts_with('.') || s == "target" || s == "node_modules"
        }) {
            return false;
        }
    }
    
    true
}
