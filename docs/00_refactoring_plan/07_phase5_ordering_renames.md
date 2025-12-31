## Phase 5: Ordering & Renames

Action: optional: rename files to match ordering conventions.
Note: update module paths and imports after renames.

- [Rust] `MMSB/tools/mmsb-analyzer/src/050_cluster_010.rs` -> `MMSB/tools/mmsb-analyzer/src/060_cluster_010.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/070_cluster_011.rs` -> `MMSB/tools/mmsb-analyzer/src/080_cluster_011.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/100_structural_detector.rs` -> `MMSB/tools/mmsb-analyzer/src/110_structural_detector.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/120_semantic_detector.rs` -> `MMSB/tools/mmsb-analyzer/src/130_semantic_detector.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/150_invariant_integrator.rs` -> `MMSB/tools/mmsb-analyzer/src/160_invariant_integrator.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/160_layer_utilities.rs` -> `MMSB/tools/mmsb-analyzer/src/170_layer_utilities.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/210_utilities.rs` -> `MMSB/tools/mmsb-analyzer/src/220_utilities.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/211_dead_code_attribute_parser.rs` -> `MMSB/tools/mmsb-analyzer/src/230_dead_code_attribute_parser.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/320_main.rs` -> `MMSB/tools/mmsb-analyzer/src/340_main.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/340_lib.rs` -> `MMSB/tools/mmsb-analyzer/src/360_lib.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/380_dead_code_call_graph.rs` -> `MMSB/tools/mmsb-analyzer/src/390_dead_code_call_graph.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/460_dead_code_report.rs` -> `MMSB/tools/mmsb-analyzer/src/470_dead_code_report.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/520_violation_predictor.rs` -> `MMSB/tools/mmsb-analyzer/src/530_violation_predictor.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/590_quality_delta_calculator.rs` -> `MMSB/tools/mmsb-analyzer/src/600_quality_delta_calculator.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/610_correction_plan_serializer.rs` -> `MMSB/tools/mmsb-analyzer/src/620_correction_plan_serializer.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/020_invariant_types.rs` -> `MMSB/tools/mmsb-analyzer/src/030_invariant_types.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/030_refactor_constraints.rs` -> `MMSB/tools/mmsb-analyzer/src/040_refactor_constraints.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/040_scc_compressor.rs` -> `MMSB/tools/mmsb-analyzer/src/050_scc_compressor.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/060_layer_inference.rs` -> `MMSB/tools/mmsb-analyzer/src/070_layer_inference.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/080_fixpoint_solver.rs` -> `MMSB/tools/mmsb-analyzer/src/090_fixpoint_solver.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/090_dependency.rs` -> `MMSB/tools/mmsb-analyzer/src/100_dependency.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/110_cluster_006.rs` -> `MMSB/tools/mmsb-analyzer/src/120_cluster_006.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/130_layer_core.rs` -> `MMSB/tools/mmsb-analyzer/src/140_layer_core.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/140_path_detector.rs` -> `MMSB/tools/mmsb-analyzer/src/150_path_detector.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/170_invariant_reporter.rs` -> `MMSB/tools/mmsb-analyzer/src/180_invariant_reporter.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/180_conscience_graph.rs` -> `MMSB/tools/mmsb-analyzer/src/190_conscience_graph.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/190_action_validator.rs` -> `MMSB/tools/mmsb-analyzer/src/200_action_validator.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/200_agent_conscience.rs` -> `MMSB/tools/mmsb-analyzer/src/210_agent_conscience.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/220_types.rs` -> `MMSB/tools/mmsb-analyzer/src/240_types.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/230_cohesion_analyzer.rs` -> `MMSB/tools/mmsb-analyzer/src/250_cohesion_analyzer.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/240_directory_analyzer.rs` -> `MMSB/tools/mmsb-analyzer/src/260_directory_analyzer.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/250_control_flow.rs` -> `MMSB/tools/mmsb-analyzer/src/270_control_flow.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/260_file_ordering.rs` -> `MMSB/tools/mmsb-analyzer/src/280_file_ordering.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/270_julia_parser.rs` -> `MMSB/tools/mmsb-analyzer/src/290_julia_parser.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/280_rust_parser.rs` -> `MMSB/tools/mmsb-analyzer/src/300_rust_parser.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/290_dot_exporter.rs` -> `MMSB/tools/mmsb-analyzer/src/310_dot_exporter.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/300_file_gathering.rs` -> `MMSB/tools/mmsb-analyzer/src/320_file_gathering.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/310_report.rs` -> `MMSB/tools/mmsb-analyzer/src/330_report.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/330_agent_cli.rs` -> `MMSB/tools/mmsb-analyzer/src/350_agent_cli.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/350_dead_code_types.rs` -> `MMSB/tools/mmsb-analyzer/src/370_dead_code_types.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/370_dead_code_doc_comment_parser.rs` -> `MMSB/tools/mmsb-analyzer/src/380_dead_code_doc_comment_parser.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/390_dead_code_intent.rs` -> `MMSB/tools/mmsb-analyzer/src/400_dead_code_intent.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/400_dead_code_test_boundaries.rs` -> `MMSB/tools/mmsb-analyzer/src/410_dead_code_test_boundaries.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/410_dead_code_entrypoints.rs` -> `MMSB/tools/mmsb-analyzer/src/420_dead_code_entrypoints.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/420_dead_code_classifier.rs` -> `MMSB/tools/mmsb-analyzer/src/430_dead_code_classifier.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/430_dead_code_confidence.rs` -> `MMSB/tools/mmsb-analyzer/src/440_dead_code_confidence.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/440_dead_code_actions.rs` -> `MMSB/tools/mmsb-analyzer/src/450_dead_code_actions.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/450_correction_plan_types.rs` -> `MMSB/tools/mmsb-analyzer/src/460_correction_plan_types.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/470_dead_code_filter.rs` -> `MMSB/tools/mmsb-analyzer/src/480_dead_code_filter.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/480_verification_policy_types.rs` -> `MMSB/tools/mmsb-analyzer/src/490_verification_policy_types.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/490_dead_code_cli.rs` -> `MMSB/tools/mmsb-analyzer/src/500_dead_code_cli.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/500_quality_delta_types.rs` -> `MMSB/tools/mmsb-analyzer/src/510_quality_delta_types.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/510_dead_code_policy.rs` -> `MMSB/tools/mmsb-analyzer/src/520_dead_code_policy.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/530_dead_code_report_split.rs` -> `MMSB/tools/mmsb-analyzer/src/540_dead_code_report_split.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/540_tier_classifier.rs` -> `MMSB/tools/mmsb-analyzer/src/550_tier_classifier.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/550_confidence_scorer.rs` -> `MMSB/tools/mmsb-analyzer/src/560_confidence_scorer.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/560_correction_plan_generator.rs` -> `MMSB/tools/mmsb-analyzer/src/570_correction_plan_generator.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/570_verification_scope_planner.rs` -> `MMSB/tools/mmsb-analyzer/src/580_verification_scope_planner.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/580_rollback_criteria_builder.rs` -> `MMSB/tools/mmsb-analyzer/src/590_rollback_criteria_builder.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/600_action_impact_estimator.rs` -> `MMSB/tools/mmsb-analyzer/src/610_action_impact_estimator.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/620_verification_policy_emitter.rs` -> `MMSB/tools/mmsb-analyzer/src/630_verification_policy_emitter.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/630_correction_intelligence_report.rs` -> `MMSB/tools/mmsb-analyzer/src/640_correction_intelligence_report.rs`

```bash
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/050_cluster_010.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/060_cluster_010.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/070_cluster_011.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/080_cluster_011.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/100_structural_detector.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/110_structural_detector.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/120_semantic_detector.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/130_semantic_detector.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/150_invariant_integrator.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/160_invariant_integrator.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/160_layer_utilities.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/170_layer_utilities.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/210_utilities.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/220_utilities.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/211_dead_code_attribute_parser.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/230_dead_code_attribute_parser.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/320_main.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/340_main.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/340_lib.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/360_lib.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/380_dead_code_call_graph.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/390_dead_code_call_graph.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/460_dead_code_report.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/470_dead_code_report.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/520_violation_predictor.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/530_violation_predictor.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/590_quality_delta_calculator.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/600_quality_delta_calculator.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/610_correction_plan_serializer.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/620_correction_plan_serializer.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/020_invariant_types.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/030_invariant_types.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/030_refactor_constraints.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/040_refactor_constraints.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/040_scc_compressor.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/050_scc_compressor.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/060_layer_inference.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/070_layer_inference.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/080_fixpoint_solver.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/090_fixpoint_solver.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/090_dependency.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/100_dependency.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/110_cluster_006.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/120_cluster_006.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/130_layer_core.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/140_layer_core.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/140_path_detector.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/150_path_detector.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/170_invariant_reporter.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/180_invariant_reporter.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/180_conscience_graph.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/190_conscience_graph.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/190_action_validator.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/200_action_validator.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/200_agent_conscience.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/210_agent_conscience.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/220_types.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/240_types.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/230_cohesion_analyzer.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/250_cohesion_analyzer.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/240_directory_analyzer.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/260_directory_analyzer.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/250_control_flow.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/270_control_flow.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/260_file_ordering.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/280_file_ordering.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/270_julia_parser.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/290_julia_parser.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/280_rust_parser.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/300_rust_parser.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/290_dot_exporter.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/310_dot_exporter.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/300_file_gathering.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/320_file_gathering.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/310_report.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/330_report.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/330_agent_cli.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/350_agent_cli.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/350_dead_code_types.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/370_dead_code_types.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/370_dead_code_doc_comment_parser.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/380_dead_code_doc_comment_parser.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/390_dead_code_intent.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/400_dead_code_intent.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/400_dead_code_test_boundaries.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/410_dead_code_test_boundaries.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/410_dead_code_entrypoints.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/420_dead_code_entrypoints.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/420_dead_code_classifier.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/430_dead_code_classifier.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/430_dead_code_confidence.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/440_dead_code_confidence.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/440_dead_code_actions.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/450_dead_code_actions.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/450_correction_plan_types.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/460_correction_plan_types.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/470_dead_code_filter.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/480_dead_code_filter.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/480_verification_policy_types.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/490_verification_policy_types.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/490_dead_code_cli.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/500_dead_code_cli.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/500_quality_delta_types.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/510_quality_delta_types.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/510_dead_code_policy.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/520_dead_code_policy.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/530_dead_code_report_split.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/540_dead_code_report_split.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/540_tier_classifier.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/550_tier_classifier.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/550_confidence_scorer.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/560_confidence_scorer.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/560_correction_plan_generator.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/570_correction_plan_generator.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/570_verification_scope_planner.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/580_verification_scope_planner.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/580_rollback_criteria_builder.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/590_rollback_criteria_builder.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/600_action_impact_estimator.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/610_action_impact_estimator.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/620_verification_policy_emitter.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/630_verification_policy_emitter.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/630_correction_intelligence_report.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/640_correction_intelligence_report.rs"
```

