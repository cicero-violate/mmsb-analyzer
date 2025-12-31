//! MMSB Analyzer Library
//!
//! Provides code analysis capabilities for Rust and Julia projects.

#[path = "000_invariant_types.rs"]
pub mod invariant_types;
#[path = "005_refactor_constraints.rs"]
pub mod refactor_constraints;
#[path = "010_scc_compressor.rs"]
pub mod scc_compressor;
#[path = "020_layer_inference.rs"]
pub mod layer_inference;
#[path = "030_fixpoint_solver.rs"]
pub mod fixpoint_solver;
#[path = "040_structural_detector.rs"]
pub mod structural_detector;
#[path = "050_semantic_detector.rs"]
pub mod semantic_detector;
#[path = "060_path_detector.rs"]
pub mod path_detector;
#[path = "070_invariant_integrator.rs"]
pub mod invariant_integrator;
#[path = "080_invariant_reporter.rs"]
pub mod invariant_reporter;
#[path = "082_conscience_graph.rs"]
pub mod conscience_graph;
#[path = "083_action_validator.rs"]
pub mod action_validator;
#[path = "085_agent_conscience.rs"]
pub mod agent_conscience;
#[path = "000_cluster_001.rs"]
pub mod cluster_001;
#[path = "010_cluster_008.rs"]
pub mod cluster_008;
#[path = "020_cluster_010.rs"]
pub mod cluster_010;
#[path = "030_cluster_011.rs"]
pub mod cluster_011;
#[path = "050_cluster_006.rs"]
pub mod cluster_006;
#[path = "040_dependency.rs"]
pub mod dependency;
#[path = "060_layer_core.rs"]
pub mod layer_core;
#[path = "070_layer_utilities.rs"]
pub mod layer_utilities;
#[path = "090_utilities.rs"]
pub mod utilities;
#[path = "100_types.rs"]
pub mod types;
#[path = "110_cohesion_analyzer.rs"]
pub mod cohesion_analyzer;
#[path = "120_directory_analyzer.rs"]
pub mod directory_analyzer;
#[path = "130_control_flow.rs"]
pub mod control_flow;
#[path = "140_file_ordering.rs"]
pub mod file_ordering;
#[path = "150_julia_parser.rs"]
pub mod julia_parser;
#[path = "160_rust_parser.rs"]
pub mod rust_parser;
#[path = "170_dot_exporter.rs"]
pub mod dot_exporter;
#[path = "180_report.rs"]
pub mod report;

pub use invariant_types::*;
pub use refactor_constraints::*;
pub use action_validator::{AgentAction, ConstraintViolation, ViolationSeverity};
pub use agent_conscience::{ActionPermission, AgentConscience};
pub use conscience_graph::{generate_conscience_map, generate_conscience_stats};
pub use dependency::{
    julia_entry_paths, order_julia_files_by_dependency, order_rust_files_by_dependency, LayerGraph,
    build_file_dependency_graph, analyze_file_ordering,
};
pub use types::AnalysisResult;
pub use cohesion_analyzer::FunctionCohesionAnalyzer;
pub use directory_analyzer::DirectoryAnalyzer;
pub use control_flow::ControlFlowAnalyzer;
pub use file_ordering::{DagCache, parallel_build_file_dag};
pub use julia_parser::JuliaAnalyzer;
pub use rust_parser::RustAnalyzer;
pub use dot_exporter::{export_complete_program_dot, export_program_cfg_to_path};
pub use report::ReportGenerator;
