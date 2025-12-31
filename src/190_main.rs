#[path = "000_invariant_types.rs"]
mod invariant_types;
#[path = "005_refactor_constraints.rs"]
mod refactor_constraints;
#[path = "010_scc_compressor.rs"]
mod scc_compressor;
#[path = "020_layer_inference.rs"]
mod layer_inference;
#[path = "030_fixpoint_solver.rs"]
mod fixpoint_solver;
#[path = "040_structural_detector.rs"]
mod structural_detector;
#[path = "050_semantic_detector.rs"]
mod semantic_detector;
#[path = "060_path_detector.rs"]
mod path_detector;
#[path = "070_invariant_integrator.rs"]
mod invariant_integrator;
#[path = "080_invariant_reporter.rs"]
mod invariant_reporter;
#[path = "082_conscience_graph.rs"]
mod conscience_graph;
#[path = "083_action_validator.rs"]
mod action_validator;
#[path = "085_agent_conscience.rs"]
mod agent_conscience;
#[path = "191_agent_cli.rs"]
mod agent_cli;
#[path = "000_cluster_001.rs"]
mod cluster_001;
#[path = "010_cluster_008.rs"]
mod cluster_008;
#[path = "020_cluster_010.rs"]
mod cluster_010;
#[path = "030_cluster_011.rs"]
mod cluster_011;
#[path = "050_cluster_006.rs"]
mod cluster_006;
#[path = "130_control_flow.rs"]
mod control_flow;
#[path = "120_directory_analyzer.rs"]
mod directory_analyzer;
#[path = "060_layer_core.rs"]
mod layer_core;
#[path = "070_layer_utilities.rs"]
mod layer_utilities;
#[path = "040_dependency.rs"]
mod dependency;
#[path = "090_utilities.rs"]
mod utilities;
#[path = "170_dot_exporter.rs"]
mod dot_exporter;
#[path = "110_cohesion_analyzer.rs"]
mod cohesion_analyzer;
#[path = "140_file_ordering.rs"]
mod file_ordering;
#[path = "150_julia_parser.rs"]
mod julia_parser;
#[path = "180_report.rs"]
mod report;
#[path = "160_rust_parser.rs"]
mod rust_parser;
#[path = "100_types.rs"]
mod types;

use anyhow::Result;

fn main() -> Result<()> {
    // Check if running in agent mode
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "agent" {
        // Remove "agent" from args and run agent CLI
        return agent_cli::run_agent_cli();
    }

    // Normal analyzer mode
    crate::layer_utilities::main()
}
