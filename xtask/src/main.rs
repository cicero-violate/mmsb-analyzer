use std::env;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must live in a subdirectory");
    let (input_dir, output_dir, extra_args) = parse_args(root);

    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(root)
        .status()
        .expect("failed to run cargo build");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }

    let status = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--")
        .arg("--root")
        .arg(input_dir)
        .arg("--output")
        .arg(output_dir)
        .args(extra_args)
        .current_dir(root)
        .status()
        .expect("failed to run analyzer");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

fn parse_args(root: &Path) -> (PathBuf, PathBuf, Vec<String>) {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let input = if args.is_empty() {
        root.to_path_buf()
    } else {
        resolve_path(root, args.remove(0))
    };
    let output = if args.is_empty() {
        root.join("docs")
    } else {
        resolve_path(root, args.remove(0))
    };
    (input, output, args)
}

fn resolve_path(root: &Path, raw: String) -> PathBuf {
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        path
    } else {
        root.join(path)
    }
}
