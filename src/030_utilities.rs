//! Utility functions shared across modules

/// Compress absolute paths to MMSB-relative format
pub fn compress_path(path: &str) -> String {
    // Find MMSB in the path and return everything from there
    if let Some(idx) = path.find("/MMSB/") {
        return format!("MMSB{}", &path[idx + 5..]);
    }
    // If already starts with MMSB/, return as-is
    if path.starts_with("MMSB/") {
        return path.to_string();
    }
    // Fallback: try to find src/ or other common markers
    if let Some(idx) = path.rfind("/src/") {
        return format!("MMSB/src{}", &path[idx + 4..]);
    }
    // Last resort: return original
    path.to_string()
}

pub fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        "_old"
            | "target"
            | "examples"
            | "tools"
            | "docs"
            | "xtask"
            | ".git"
            | ".julia"
            | "test"
            | "tests"
            | "benches"
    )
}

// Re-export layer utility functions for backward compatibility
pub use crate::layer_utilities::{allow_analysis_dir, detect_layer, resolve_source_root};
