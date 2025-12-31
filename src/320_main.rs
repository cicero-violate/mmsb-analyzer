#[path = "020_invariant_types.rs"]
mod invariant_types;
#[path = "030_refactor_constraints.rs"]
mod refactor_constraints;
#[path = "040_scc_compressor.rs"]
mod scc_compressor;
#[path = "060_layer_inference.rs"]
mod layer_inference;
#[path = "080_fixpoint_solver.rs"]
mod fixpoint_solver;
#[path = "100_structural_detector.rs"]
mod structural_detector;
#[path = "120_semantic_detector.rs"]
mod semantic_detector;
#[path = "140_path_detector.rs"]
mod path_detector;
#[path = "150_invariant_integrator.rs"]
mod invariant_integrator;
#[path = "170_invariant_reporter.rs"]
mod invariant_reporter;
#[path = "180_conscience_graph.rs"]
mod conscience_graph;
#[path = "190_action_validator.rs"]
mod action_validator;
#[path = "200_agent_conscience.rs"]
mod agent_conscience;
#[path = "330_agent_cli.rs"]
mod agent_cli;
#[path = "000_cluster_001.rs"]
mod cluster_001;
#[path = "010_cluster_008.rs"]
mod cluster_008;
#[path = "050_cluster_010.rs"]
mod cluster_010;
#[path = "070_cluster_011.rs"]
mod cluster_011;
#[path = "110_cluster_006.rs"]
mod cluster_006;
#[path = "250_control_flow.rs"]
mod control_flow;
#[path = "240_directory_analyzer.rs"]
mod directory_analyzer;
#[path = "130_layer_core.rs"]
mod layer_core;
#[path = "160_layer_utilities.rs"]
mod layer_utilities;
#[path = "090_dependency.rs"]
mod dependency;
#[path = "210_utilities.rs"]
mod utilities;
#[path = "290_dot_exporter.rs"]
mod dot_exporter;
#[path = "230_cohesion_analyzer.rs"]
mod cohesion_analyzer;
#[path = "260_file_ordering.rs"]
mod file_ordering;
#[path = "270_julia_parser.rs"]
mod julia_parser;
#[path = "310_report.rs"]
mod report;
#[path = "350_dead_code_types.rs"]
mod dead_code_types;
#[path = "211_dead_code_attribute_parser.rs"]
mod dead_code_attribute_parser;
#[path = "370_dead_code_doc_comment_parser.rs"]
mod dead_code_doc_comment_parser;
#[path = "380_dead_code_call_graph.rs"]
mod dead_code_call_graph;
#[path = "390_dead_code_intent.rs"]
mod dead_code_intent;
#[path = "400_dead_code_test_boundaries.rs"]
mod dead_code_test_boundaries;
#[path = "410_dead_code_entrypoints.rs"]
mod dead_code_entrypoints;
#[path = "420_dead_code_classifier.rs"]
mod dead_code_classifier;
#[path = "430_dead_code_confidence.rs"]
mod dead_code_confidence;
#[path = "440_dead_code_actions.rs"]
mod dead_code_actions;
#[path = "460_dead_code_report.rs"]
mod dead_code_report;
#[path = "470_dead_code_filter.rs"]
mod dead_code_filter;
#[path = "490_dead_code_cli.rs"]
mod dead_code_cli;
#[path = "510_dead_code_policy.rs"]
mod dead_code_policy;
#[path = "530_dead_code_report_split.rs"]
mod dead_code_report_split;
#[path = "450_correction_plan_types.rs"]
mod correction_plan_types;
#[path = "480_verification_policy_types.rs"]
mod verification_policy_types;
#[path = "500_quality_delta_types.rs"]
mod quality_delta_types;
#[path = "520_violation_predictor.rs"]
mod violation_predictor;
#[path = "540_tier_classifier.rs"]
mod tier_classifier;
#[path = "550_confidence_scorer.rs"]
mod confidence_scorer;
#[path = "560_correction_plan_generator.rs"]
mod correction_plan_generator;
#[path = "570_verification_scope_planner.rs"]
mod verification_scope_planner;
#[path = "580_rollback_criteria_builder.rs"]
mod rollback_criteria_builder;
#[path = "590_quality_delta_calculator.rs"]
mod quality_delta_calculator;
#[path = "600_action_impact_estimator.rs"]
mod action_impact_estimator;
#[path = "610_correction_plan_serializer.rs"]
mod correction_plan_serializer;
#[path = "620_verification_policy_emitter.rs"]
mod verification_policy_emitter;
#[path = "630_correction_intelligence_report.rs"]
mod correction_intelligence_report;
#[path = "280_rust_parser.rs"]
mod rust_parser;
#[path = "220_types.rs"]
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
