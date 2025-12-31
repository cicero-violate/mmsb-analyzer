//! Layer utility functions for layer-based dependency analysis
//! This module is at layer 010 to be accessible from all higher layers

use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
pub use crate::cluster_001::{build_file_layers, detect_layer, gather_julia_files, julia_entry_paths};
#[allow(unused_imports)]
pub use crate::cluster_010::contains_tools;

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

pub fn gather_rust_files(root: &Path) -> Vec<PathBuf> {
    use walkdir::WalkDir;

    let src_root = resolve_source_root(root);
    WalkDir::new(&src_root)
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            if !entry.file_type().is_dir() {
                return true;
            }
            allow_analysis_dir(&src_root, entry.path())
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| {
            let rel = e.path().strip_prefix(&src_root).unwrap_or(e.path());
            rel.components().count() == 1 || e.path().starts_with(src_root.join("src"))
        })
        .map(|entry| entry.into_path())
        .collect()
}

// ============================================================================
// CLI Entrypoint (from src/000_cluster_011.rs)
// ============================================================================

#[derive(Parser, Debug)]
#[command(name = "mmsb-analyzer")]
#[command(about = "MMSB Intelligence Substrate Analyzer", long_about = None)]
struct Args {
    /// Root directory to analyze
    #[arg(short, long, default_value = "../..")]
    root: PathBuf,

    /// Output directory for reports
    #[arg(short, long, default_value = "../../docs/analysis")]
    output: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Skip Julia file analysis
    #[arg(long)]
    skip_julia: bool,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    let root_path = std::env::current_dir()?.join(&args.root).canonicalize()?;
    let output_path = std::env::current_dir()?
        .join(&args.output)
        .canonicalize()
        .unwrap_or_else(|_| {
            let p = std::env::current_dir().unwrap().join(&args.output);
            std::fs::create_dir_all(&p).ok();
            p.canonicalize().unwrap_or(p)
        });
    run_analysis(&root_path, &output_path, args.verbose, args.skip_julia)
}

pub fn run_analysis(
    root_path: &Path,
    output_path: &Path,
    verbose: bool,
    skip_julia: bool,
) -> Result<()> {
    use crate::control_flow::ControlFlowAnalyzer;
    use crate::cohesion_analyzer::FunctionCohesionAnalyzer;
    use crate::dependency::LayerGraph;
    use crate::directory_analyzer::DirectoryAnalyzer;
    use crate::dot_exporter::export_program_cfg_to_path;
    use crate::julia_parser::JuliaAnalyzer;
    use crate::report::ReportGenerator;
    use crate::rust_parser::RustAnalyzer;
    use crate::types::{AnalysisResult, FileOrderingResult};

    let julia_script_path = root_path.join("src/000_main.jl");

    println!("MMSB Intelligence Substrate Analyzer");
    println!("=====================================\n");
    println!("Root directory: {:?}", root_path);
    println!("Output directory: {:?}", output_path);
    println!("Julia script: {:?}\n", julia_script_path);

    let rust_analyzer = RustAnalyzer::new(root_path.to_string_lossy().to_string());
    let mut combined_result = AnalysisResult::new();

    println!("Scanning Rust files (dependency-ordered)...");
    let mut rust_count = 0;
    let rust_files = gather_rust_files(root_path);
    let (ordered_rust_files, rust_layer_graph) =
        crate::dependency::order_rust_files_by_dependency(&rust_files, root_path)
            .context("Failed to resolve Rust dependency order")?;
    let rust_file_ordering =
        crate::dependency::analyze_file_ordering(&rust_files, None)
            .context("Failed to analyze Rust file ordering")?;
    let julia_file_ordering = FileOrderingResult {
        ordered_files: Vec::new(),
        violations: Vec::new(),
        layer_violations: Vec::new(),
        ordered_directories: Vec::new(),
        cycles: Vec::new(),
    };

    for path in ordered_rust_files {
        if verbose {
            println!("  Analyzing: {:?}", path);
        }

        match rust_analyzer.analyze_file(&path) {
            Ok(result) => {
                rust_count += 1;
                combined_result.merge(result);
            }
            Err(e) => {
                eprintln!("Warning: Failed to analyze {:?}: {}", path, e);
            }
        }
    }

    println!("  Analyzed {} Rust files\n", rust_count);

    let mut julia_count = 0;
    let mut julia_layer_graph = LayerGraph {
        ordered_layers: Vec::new(),
        edges: Vec::new(),
        cycles: Vec::new(),
        unresolved: Vec::new(),
    };
    if !skip_julia {
        println!("Scanning Julia files (dependency-ordered)...");
        let julia_files = gather_julia_files(root_path);
        let (ordered_julia_files, jlg) =
            crate::dependency::order_julia_files_by_dependency(&julia_files, root_path)
                .context("Failed to resolve Julia dependency order")?;
        julia_layer_graph = jlg;

        if julia_script_path.exists() {
            let julia_analyzer = JuliaAnalyzer::new(
                root_path.to_path_buf(),
                julia_script_path.clone(),
                output_path.join("30_cfg/dots"),
            );

            for path in ordered_julia_files {
                if verbose {
                    println!("  Analyzing: {:?}", path);
                }

                match julia_analyzer.analyze_file(&path) {
                    Ok(result) => {
                        julia_count += 1;
                        combined_result.merge(result);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to analyze {:?}: {}", path, e);
                    }
                }
            }
        } else {
            println!("  Skipping Julia analysis (script not found)");
        }

        println!("  Analyzed {} Julia files\n", julia_count);
    }

    println!("Building call graph...");
    let mut cf_analyzer = ControlFlowAnalyzer::new();
    cf_analyzer.build_call_graph(&combined_result);

    // NEW: Invariant detection
    use crate::invariant_integrator::InvariantDetector;
    println!("Detecting invariants...");
    let invariants_result = {
        let invariant_detector = InvariantDetector::new(
            &combined_result,
            &combined_result.call_graph,
        );
        invariant_detector.detect_all()
    };
    let constraints = {
        let invariant_detector = InvariantDetector::new(
            &combined_result,
            &combined_result.call_graph,
        );
        invariant_detector.generate_constraints(&invariants_result)
    };
    combined_result.invariants = invariants_result;
    combined_result.constraints = constraints;

    println!("Analyzing function cohesion...");
    let cohesion_analyzer = FunctionCohesionAnalyzer::new();
    let placements = cohesion_analyzer.analyze(&combined_result)?;
    let clusters = cohesion_analyzer.detect_clusters(&combined_result)?;

    println!("Analyzing directory structure...");
    let dir_analyzer = DirectoryAnalyzer::new(root_path.to_path_buf());
    let dir_analysis = dir_analyzer.analyze()?;

    println!("\nGenerating reports...");
    let report_gen = ReportGenerator::new(output_path.to_string_lossy().to_string());
    report_gen.generate_all(
        &combined_result,
        &cf_analyzer,
        &rust_layer_graph,
        &julia_layer_graph,
        &rust_file_ordering,
        &julia_file_ordering,
        &placements,
        &clusters,
        &dir_analysis,
        root_path,
    )
    .context("Failed to generate reports")?;

    println!("\nExporting program CFG...");
    export_program_cfg_to_path(&combined_result, &cf_analyzer.call_edges(), output_path)?;

    println!("\nGenerating invariant report...");
    use crate::invariant_reporter;
    invariant_reporter::generate_invariant_report(&combined_result.invariants, output_path)
        .context("Failed to generate invariant report")?;
    invariant_reporter::export_constraints_json(&combined_result.constraints, output_path)
        .context("Failed to export constraints")?;

    println!("\n✓ Analysis complete!");
    println!("  Total elements: {}", combined_result.elements.len());
    println!("  Rust files: {}", rust_count);
    println!("  Julia files: {}", julia_count);
    println!("  Output: {}\n", output_path.display());

    Ok(())
}
