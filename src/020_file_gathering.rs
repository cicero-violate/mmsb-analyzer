//! File gathering utilities for traversing and collecting source files

use crate::layer_utilities::allow_analysis_dir;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Gather all Rust source files from a root directory
pub fn gather_rust_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            if !entry.file_type().is_dir() {
                return true;
            }
            allow_analysis_dir(root, entry.path())
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| {
            let rel = e.path().strip_prefix(root).unwrap_or(e.path());
            rel.components().count() == 1 || e.path().starts_with(root.join("src"))
        })
        .map(|entry| entry.into_path())
        .collect()
}

/// Gather all Julia source files from a root directory
pub fn gather_julia_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            if !entry.file_type().is_dir() {
                return true;
            }
            allow_analysis_dir(root, entry.path())
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "jl"))
        .filter(|e| {
            let rel = e.path().strip_prefix(root).unwrap_or(e.path());
            rel.components().count() == 1 || e.path().starts_with(root.join("src"))
        })
        .map(|entry| entry.into_path())
        .collect()
}
