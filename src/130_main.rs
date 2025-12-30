#[path = "070_control_flow.rs"]
mod control_flow;
#[path = "060_directory_analyzer.rs"]
mod directory_analyzer;
#[path = "010_layer_utilities.rs"]
mod layer_utilities;
#[path = "020_file_gathering.rs"]
mod file_gathering;
#[path = "000_dependency.rs"]
mod dependency;
#[path = "030_utilities.rs"]
mod utilities;
#[path = "110_dot_exporter.rs"]
mod dot_exporter;
#[path = "050_cohesion_analyzer.rs"]
mod cohesion_analyzer;
#[path = "080_file_ordering.rs"]
mod file_ordering;
#[path = "090_julia_parser.rs"]
mod julia_parser;
#[path = "120_report.rs"]
mod report;
#[path = "100_rust_parser.rs"]
mod rust_parser;
#[path = "040_types.rs"]
mod types;

use crate::control_flow::ControlFlowAnalyzer;
use crate::directory_analyzer::DirectoryAnalyzer;
use crate::dependency::{
    order_julia_files_by_dependency, order_rust_files_by_dependency,
};
use crate::file_gathering::{gather_julia_files, gather_rust_files};
use crate::dot_exporter::export_program_cfg_to_path;
use crate::cohesion_analyzer::FunctionCohesionAnalyzer;
use crate::file_ordering::analyze_file_ordering;
use crate::types::FileOrderingResult;
use crate::dependency::LayerGraph;
use crate::julia_parser::JuliaAnalyzer;
use crate::report::ReportGenerator;
use crate::rust_parser::RustAnalyzer;
use crate::types::AnalysisResult;
use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::path::PathBuf;

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

fn main() -> Result<()> {
    let args = Args::parse();

    // Resolve to absolute paths
    let root_path = env::current_dir()?.join(&args.root).canonicalize()?;
    let output_path = env::current_dir()?
        .join(&args.output)
        .canonicalize()
        .unwrap_or_else(|_| {
            let p = env::current_dir().unwrap().join(&args.output);
            std::fs::create_dir_all(&p).ok();
            p.canonicalize().unwrap_or(p)
        });
    let julia_script_path = root_path.join("src/000_main.jl");

    println!("MMSB Intelligence Substrate Analyzer");
    println!("=====================================\n");
    println!("Root directory: {:?}", root_path);
    println!("Output directory: {:?}", output_path);
    println!("Julia script: {:?}\n", julia_script_path);

    let rust_analyzer = RustAnalyzer::new(root_path.to_string_lossy().to_string());
    let mut combined_result = AnalysisResult::new();

    // Scan for Rust files
    println!("Scanning Rust files (dependency-ordered)...");
    let mut rust_count = 0;
    let rust_files = gather_rust_files(&root_path);
    let (ordered_rust_files, rust_layer_graph) =
        order_rust_files_by_dependency(&rust_files, &root_path)
            .context("Failed to resolve Rust dependency order")?;
    let rust_file_ordering =
        analyze_file_ordering(&rust_files, None).context("Failed to analyze Rust file ordering")?;
    let julia_file_ordering = FileOrderingResult {
        ordered_files: Vec::new(),
        violations: Vec::new(),
        layer_violations: Vec::new(),
        ordered_directories: Vec::new(),
        cycles: Vec::new(),
    };

    for path in ordered_rust_files {
        if args.verbose {
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

    // Julia analysis
    let mut julia_count = 0;
    let mut julia_layer_graph = LayerGraph {
        ordered_layers: Vec::new(),
        edges: Vec::new(),
        cycles: Vec::new(),
        unresolved: Vec::new(),
    };
    if !args.skip_julia {
        println!("Scanning Julia files (dependency-ordered)...");
        let julia_files = gather_julia_files(&root_path);
        let (ordered_julia_files, jlg) =
            order_julia_files_by_dependency(&julia_files, &root_path)
                .context("Failed to resolve Julia dependency order")?;
        julia_layer_graph = jlg;

        if julia_script_path.exists() {
            let julia_analyzer = JuliaAnalyzer::new(
                root_path.clone(),
                julia_script_path.clone(),
                output_path.join("cfg/dots"),
            );

            for path in ordered_julia_files {
                if args.verbose {
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

    // Build control flow graph
    println!("Building call graph...");
    let mut cf_analyzer = ControlFlowAnalyzer::new();
    cf_analyzer.build_call_graph(&combined_result);

    // Function cohesion analysis
    println!("Analyzing function cohesion...");
    let cohesion_analyzer = FunctionCohesionAnalyzer::new();
    let placements = cohesion_analyzer.analyze(&combined_result)?;
    let clusters = cohesion_analyzer.detect_clusters(&combined_result)?;

    // Directory analysis
    println!("Analyzing directory structure...");
    let dir_analyzer = DirectoryAnalyzer::new(root_path.clone());
    let dir_analysis = dir_analyzer.analyze()?;

    // Generate reports
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
        &root_path,
    )
        .context("Failed to generate reports")?;

    println!("\nExporting program CFG...");
    export_program_cfg_to_path(&combined_result, &cf_analyzer.call_edges(), &output_path)?;

    println!("\n✓ Analysis complete!");
    println!("  Total elements: {}", combined_result.elements.len());
    println!("  Rust files: {}", rust_count);
    println!("  Julia files: {}", julia_count);
    println!("  Output: {}\n", output_path.display());

    Ok(())
}
