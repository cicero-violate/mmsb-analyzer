//! MMSB Analyzer Library
//!
//! Provides code analysis capabilities for Rust and Julia projects.

#[path = "000_dependency.rs"]
pub mod dependency;
#[path = "010_layer_utilities.rs"]
pub mod layer_utilities;
#[path = "020_file_gathering.rs"]
pub mod file_gathering;
#[path = "030_utilities.rs"]
pub mod utilities;
#[path = "040_types.rs"]
pub mod types;
#[path = "050_cohesion_analyzer.rs"]
pub mod cohesion_analyzer;
#[path = "060_directory_analyzer.rs"]
pub mod directory_analyzer;
#[path = "070_control_flow.rs"]
pub mod control_flow;
#[path = "080_file_ordering.rs"]
pub mod file_ordering;
#[path = "090_julia_parser.rs"]
pub mod julia_parser;
#[path = "100_rust_parser.rs"]
pub mod rust_parser;
#[path = "110_dot_exporter.rs"]
pub mod dot_exporter;
#[path = "120_report.rs"]
pub mod report;

pub use dependency::{
    julia_entry_paths, order_julia_files_by_dependency, order_rust_files_by_dependency, LayerGraph,
};
pub use types::AnalysisResult;
pub use cohesion_analyzer::FunctionCohesionAnalyzer;
pub use directory_analyzer::DirectoryAnalyzer;
pub use control_flow::ControlFlowAnalyzer;
pub use file_ordering::{
    analyze_file_ordering, build_file_dependency_graph, DagCache, parallel_build_file_dag,
};
pub use julia_parser::JuliaAnalyzer;
pub use rust_parser::RustAnalyzer;
pub use dot_exporter::{export_complete_program_dot, export_program_cfg_to_path};
pub use report::ReportGenerator;
