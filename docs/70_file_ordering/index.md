# File Ordering Report

Generated: 2025-12-31 06:34:48

## Rust File Ordering

### Metrics

- Total files: 65
- Rename suggestions: 62
- Ordering violations: 0
- Layer violations: 112
- Directories: 1

### Cycles Detected
- MMSB/tools/mmsb-analyzer/src/610_correction_plan_serializer.rs, MMSB/tools/mmsb-analyzer/src/630_correction_intelligence_report.rs, MMSB/tools/mmsb-analyzer/src/520_violation_predictor.rs
- MMSB/tools/mmsb-analyzer/src/230_cohesion_analyzer.rs, MMSB/tools/mmsb-analyzer/src/250_control_flow.rs, MMSB/tools/mmsb-analyzer/src/210_utilities.rs, MMSB/tools/mmsb-analyzer/src/310_report.rs, MMSB/tools/mmsb-analyzer/src/130_layer_core.rs, MMSB/tools/mmsb-analyzer/src/110_cluster_006.rs, MMSB/tools/mmsb-analyzer/src/010_cluster_008.rs, MMSB/tools/mmsb-analyzer/src/260_file_ordering.rs, MMSB/tools/mmsb-analyzer/src/090_dependency.rs, MMSB/tools/mmsb-analyzer/src/290_dot_exporter.rs, MMSB/tools/mmsb-analyzer/src/000_cluster_001.rs, MMSB/tools/mmsb-analyzer/src/240_directory_analyzer.rs, MMSB/tools/mmsb-analyzer/src/020_cluster_010.rs, MMSB/tools/mmsb-analyzer/src/160_layer_utilities.rs, MMSB/tools/mmsb-analyzer/src/530_dead_code_report_split.rs, MMSB/tools/mmsb-analyzer/src/460_dead_code_report.rs, MMSB/tools/mmsb-analyzer/src/490_dead_code_cli.rs
- MMSB/tools/mmsb-analyzer/src/590_quality_delta_calculator.rs, MMSB/tools/mmsb-analyzer/src/600_action_impact_estimator.rs
- MMSB/tools/mmsb-analyzer/src/211_dead_code_attribute_parser.rs, MMSB/tools/mmsb-analyzer/src/400_dead_code_test_boundaries.rs, MMSB/tools/mmsb-analyzer/src/380_dead_code_call_graph.rs, MMSB/tools/mmsb-analyzer/src/390_dead_code_intent.rs

### Canonical Order

| Order | Current | Suggested | Rename |
| --- | --- | --- | --- |
| 0 | `src/000_cluster_001.rs` | `000_cluster_001.rs` | no |
| 10 | `src/010_cluster_008.rs` | `010_cluster_008.rs` | no |
| 20 | `src/020_cluster_010.rs` | `020_cluster_010.rs` | no |
| 30 | `src/020_invariant_types.rs` | `030_invariant_types.rs` | yes |
| 40 | `src/030_refactor_constraints.rs` | `040_refactor_constraints.rs` | yes |
| 50 | `src/040_scc_compressor.rs` | `050_scc_compressor.rs` | yes |
| 60 | `src/050_cluster_010.rs` | `060_cluster_010.rs` | yes |
| 70 | `src/060_layer_inference.rs` | `070_layer_inference.rs` | yes |
| 80 | `src/070_cluster_011.rs` | `080_cluster_011.rs` | yes |
| 90 | `src/080_fixpoint_solver.rs` | `090_fixpoint_solver.rs` | yes |
| 100 | `src/090_dependency.rs` | `100_dependency.rs` | yes |
| 110 | `src/100_structural_detector.rs` | `110_structural_detector.rs` | yes |
| 120 | `src/110_cluster_006.rs` | `120_cluster_006.rs` | yes |
| 130 | `src/120_semantic_detector.rs` | `130_semantic_detector.rs` | yes |
| 140 | `src/130_layer_core.rs` | `140_layer_core.rs` | yes |
| 150 | `src/140_path_detector.rs` | `150_path_detector.rs` | yes |
| 160 | `src/150_invariant_integrator.rs` | `160_invariant_integrator.rs` | yes |
| 170 | `src/160_layer_utilities.rs` | `170_layer_utilities.rs` | yes |
| 180 | `src/170_invariant_reporter.rs` | `180_invariant_reporter.rs` | yes |
| 190 | `src/180_conscience_graph.rs` | `190_conscience_graph.rs` | yes |
| 200 | `src/190_action_validator.rs` | `200_action_validator.rs` | yes |
| 210 | `src/200_agent_conscience.rs` | `210_agent_conscience.rs` | yes |
| 220 | `src/210_utilities.rs` | `220_utilities.rs` | yes |
| 230 | `src/211_dead_code_attribute_parser.rs` | `230_dead_code_attribute_parser.rs` | yes |
| 240 | `src/220_types.rs` | `240_types.rs` | yes |
| 250 | `src/230_cohesion_analyzer.rs` | `250_cohesion_analyzer.rs` | yes |
| 260 | `src/240_directory_analyzer.rs` | `260_directory_analyzer.rs` | yes |
| 270 | `src/250_control_flow.rs` | `270_control_flow.rs` | yes |
| 280 | `src/260_file_ordering.rs` | `280_file_ordering.rs` | yes |
| 290 | `src/270_julia_parser.rs` | `290_julia_parser.rs` | yes |
| 300 | `src/280_rust_parser.rs` | `300_rust_parser.rs` | yes |
| 310 | `src/290_dot_exporter.rs` | `310_dot_exporter.rs` | yes |
| 320 | `src/300_file_gathering.rs` | `320_file_gathering.rs` | yes |
| 330 | `src/310_report.rs` | `330_report.rs` | yes |
| 340 | `src/320_main.rs` | `340_main.rs` | yes |
| 350 | `src/330_agent_cli.rs` | `350_agent_cli.rs` | yes |
| 360 | `src/340_lib.rs` | `360_lib.rs` | yes |
| 370 | `src/350_dead_code_types.rs` | `370_dead_code_types.rs` | yes |
| 380 | `src/370_dead_code_doc_comment_parser.rs` | `380_dead_code_doc_comment_parser.rs` | yes |
| 390 | `src/380_dead_code_call_graph.rs` | `390_dead_code_call_graph.rs` | yes |
| 400 | `src/390_dead_code_intent.rs` | `400_dead_code_intent.rs` | yes |
| 410 | `src/400_dead_code_test_boundaries.rs` | `410_dead_code_test_boundaries.rs` | yes |
| 420 | `src/410_dead_code_entrypoints.rs` | `420_dead_code_entrypoints.rs` | yes |
| 430 | `src/420_dead_code_classifier.rs` | `430_dead_code_classifier.rs` | yes |
| 440 | `src/430_dead_code_confidence.rs` | `440_dead_code_confidence.rs` | yes |
| 450 | `src/440_dead_code_actions.rs` | `450_dead_code_actions.rs` | yes |
| 460 | `src/450_correction_plan_types.rs` | `460_correction_plan_types.rs` | yes |
| 470 | `src/460_dead_code_report.rs` | `470_dead_code_report.rs` | yes |
| 480 | `src/470_dead_code_filter.rs` | `480_dead_code_filter.rs` | yes |
| 490 | `src/480_verification_policy_types.rs` | `490_verification_policy_types.rs` | yes |
| 500 | `src/490_dead_code_cli.rs` | `500_dead_code_cli.rs` | yes |
| 510 | `src/500_quality_delta_types.rs` | `510_quality_delta_types.rs` | yes |
| 520 | `src/510_dead_code_policy.rs` | `520_dead_code_policy.rs` | yes |
| 530 | `src/520_violation_predictor.rs` | `530_violation_predictor.rs` | yes |
| 540 | `src/530_dead_code_report_split.rs` | `540_dead_code_report_split.rs` | yes |
| 550 | `src/540_tier_classifier.rs` | `550_tier_classifier.rs` | yes |
| 560 | `src/550_confidence_scorer.rs` | `560_confidence_scorer.rs` | yes |
| 570 | `src/560_correction_plan_generator.rs` | `570_correction_plan_generator.rs` | yes |
| 580 | `src/570_verification_scope_planner.rs` | `580_verification_scope_planner.rs` | yes |
| 590 | `src/580_rollback_criteria_builder.rs` | `590_rollback_criteria_builder.rs` | yes |
| 600 | `src/590_quality_delta_calculator.rs` | `600_quality_delta_calculator.rs` | yes |
| 610 | `src/600_action_impact_estimator.rs` | `610_action_impact_estimator.rs` | yes |
| 620 | `src/610_correction_plan_serializer.rs` | `620_correction_plan_serializer.rs` | yes |
| 630 | `src/620_verification_policy_emitter.rs` | `630_verification_policy_emitter.rs` | yes |
| 640 | `src/630_correction_intelligence_report.rs` | `640_correction_intelligence_report.rs` | yes |

### Ordering Violations
- None detected.

### Layer Violations
- `src/100_structural_detector.rs` (100_structural_detector.rs) depends on `src/220_types.rs` (220_types.rs)
- `src/150_invariant_integrator.rs` (150_invariant_integrator.rs) depends on `src/220_types.rs` (220_types.rs)
- `src/070_cluster_011.rs` (070_cluster_011.rs) depends on `src/220_types.rs` (220_types.rs)
- `src/210_utilities.rs` (210_utilities.rs) depends on `src/310_report.rs` (310_report.rs)
- `src/210_utilities.rs` (210_utilities.rs) depends on `src/220_types.rs` (220_types.rs)
- `src/380_dead_code_call_graph.rs` (380_dead_code_call_graph.rs) depends on `src/390_dead_code_intent.rs` (390_dead_code_intent.rs)
- `src/380_dead_code_call_graph.rs` (380_dead_code_call_graph.rs) depends on `src/400_dead_code_test_boundaries.rs` (400_dead_code_test_boundaries.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/600_action_impact_estimator.rs` (600_action_impact_estimator.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/330_agent_cli.rs` (330_agent_cli.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/550_confidence_scorer.rs` (550_confidence_scorer.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/630_correction_intelligence_report.rs` (630_correction_intelligence_report.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/560_correction_plan_generator.rs` (560_correction_plan_generator.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/610_correction_plan_serializer.rs` (610_correction_plan_serializer.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/450_correction_plan_types.rs` (450_correction_plan_types.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/440_dead_code_actions.rs` (440_dead_code_actions.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/380_dead_code_call_graph.rs` (380_dead_code_call_graph.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/420_dead_code_classifier.rs` (420_dead_code_classifier.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/490_dead_code_cli.rs` (490_dead_code_cli.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/430_dead_code_confidence.rs` (430_dead_code_confidence.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/370_dead_code_doc_comment_parser.rs` (370_dead_code_doc_comment_parser.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/410_dead_code_entrypoints.rs` (410_dead_code_entrypoints.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/470_dead_code_filter.rs` (470_dead_code_filter.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/390_dead_code_intent.rs` (390_dead_code_intent.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/510_dead_code_policy.rs` (510_dead_code_policy.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/460_dead_code_report.rs` (460_dead_code_report.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/530_dead_code_report_split.rs` (530_dead_code_report_split.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/400_dead_code_test_boundaries.rs` (400_dead_code_test_boundaries.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/350_dead_code_types.rs` (350_dead_code_types.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/590_quality_delta_calculator.rs` (590_quality_delta_calculator.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/500_quality_delta_types.rs` (500_quality_delta_types.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/580_rollback_criteria_builder.rs` (580_rollback_criteria_builder.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/540_tier_classifier.rs` (540_tier_classifier.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/620_verification_policy_emitter.rs` (620_verification_policy_emitter.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/480_verification_policy_types.rs` (480_verification_policy_types.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/570_verification_scope_planner.rs` (570_verification_scope_planner.rs)
- `src/320_main.rs` (320_main.rs) depends on `src/520_violation_predictor.rs` (520_violation_predictor.rs)
- `src/460_dead_code_report.rs` (460_dead_code_report.rs) depends on `src/490_dead_code_cli.rs` (490_dead_code_cli.rs)
- `src/460_dead_code_report.rs` (460_dead_code_report.rs) depends on `src/530_dead_code_report_split.rs` (530_dead_code_report_split.rs)
- `src/211_dead_code_attribute_parser.rs` (211_dead_code_attribute_parser.rs) depends on `src/370_dead_code_doc_comment_parser.rs` (370_dead_code_doc_comment_parser.rs)
- `src/211_dead_code_attribute_parser.rs` (211_dead_code_attribute_parser.rs) depends on `src/390_dead_code_intent.rs` (390_dead_code_intent.rs)
- `src/211_dead_code_attribute_parser.rs` (211_dead_code_attribute_parser.rs) depends on `src/400_dead_code_test_boundaries.rs` (400_dead_code_test_boundaries.rs)
- `src/211_dead_code_attribute_parser.rs` (211_dead_code_attribute_parser.rs) depends on `src/350_dead_code_types.rs` (350_dead_code_types.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/110_cluster_006.rs` (110_cluster_006.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/020_cluster_010.rs` (020_cluster_010.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/230_cohesion_analyzer.rs` (230_cohesion_analyzer.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/250_control_flow.rs` (250_control_flow.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/090_dependency.rs` (090_dependency.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/240_directory_analyzer.rs` (240_directory_analyzer.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/290_dot_exporter.rs` (290_dot_exporter.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/260_file_ordering.rs` (260_file_ordering.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/150_invariant_integrator.rs` (150_invariant_integrator.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/170_invariant_reporter.rs` (170_invariant_reporter.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/270_julia_parser.rs` (270_julia_parser.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/130_layer_core.rs` (130_layer_core.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/160_layer_utilities.rs` (160_layer_utilities.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/310_report.rs` (310_report.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/280_rust_parser.rs` (280_rust_parser.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/220_types.rs` (220_types.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/210_utilities.rs` (210_utilities.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/600_action_impact_estimator.rs` (600_action_impact_estimator.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/550_confidence_scorer.rs` (550_confidence_scorer.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/630_correction_intelligence_report.rs` (630_correction_intelligence_report.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/560_correction_plan_generator.rs` (560_correction_plan_generator.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/610_correction_plan_serializer.rs` (610_correction_plan_serializer.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/450_correction_plan_types.rs` (450_correction_plan_types.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/440_dead_code_actions.rs` (440_dead_code_actions.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/380_dead_code_call_graph.rs` (380_dead_code_call_graph.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/420_dead_code_classifier.rs` (420_dead_code_classifier.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/490_dead_code_cli.rs` (490_dead_code_cli.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/430_dead_code_confidence.rs` (430_dead_code_confidence.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/370_dead_code_doc_comment_parser.rs` (370_dead_code_doc_comment_parser.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/410_dead_code_entrypoints.rs` (410_dead_code_entrypoints.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/470_dead_code_filter.rs` (470_dead_code_filter.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/390_dead_code_intent.rs` (390_dead_code_intent.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/510_dead_code_policy.rs` (510_dead_code_policy.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/460_dead_code_report.rs` (460_dead_code_report.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/530_dead_code_report_split.rs` (530_dead_code_report_split.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/400_dead_code_test_boundaries.rs` (400_dead_code_test_boundaries.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/350_dead_code_types.rs` (350_dead_code_types.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/590_quality_delta_calculator.rs` (590_quality_delta_calculator.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/500_quality_delta_types.rs` (500_quality_delta_types.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/580_rollback_criteria_builder.rs` (580_rollback_criteria_builder.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/540_tier_classifier.rs` (540_tier_classifier.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/620_verification_policy_emitter.rs` (620_verification_policy_emitter.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/480_verification_policy_types.rs` (480_verification_policy_types.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/570_verification_scope_planner.rs` (570_verification_scope_planner.rs)
- `src/340_lib.rs` (340_lib.rs) depends on `src/520_violation_predictor.rs` (520_violation_predictor.rs)
- `src/590_quality_delta_calculator.rs` (590_quality_delta_calculator.rs) depends on `src/600_action_impact_estimator.rs` (600_action_impact_estimator.rs)
- `src/610_correction_plan_serializer.rs` (610_correction_plan_serializer.rs) depends on `src/630_correction_intelligence_report.rs` (630_correction_intelligence_report.rs)
- `src/610_correction_plan_serializer.rs` (610_correction_plan_serializer.rs) depends on `src/620_verification_policy_emitter.rs` (620_verification_policy_emitter.rs)
- `src/020_cluster_010.rs` (020_cluster_010.rs) depends on `src/160_layer_utilities.rs` (160_layer_utilities.rs)
- `src/120_semantic_detector.rs` (120_semantic_detector.rs) depends on `src/220_types.rs` (220_types.rs)
- `src/520_violation_predictor.rs` (520_violation_predictor.rs) depends on `src/600_action_impact_estimator.rs` (600_action_impact_estimator.rs)
- `src/520_violation_predictor.rs` (520_violation_predictor.rs) depends on `src/630_correction_intelligence_report.rs` (630_correction_intelligence_report.rs)
- `src/520_violation_predictor.rs` (520_violation_predictor.rs) depends on `src/560_correction_plan_generator.rs` (560_correction_plan_generator.rs)
- `src/520_violation_predictor.rs` (520_violation_predictor.rs) depends on `src/580_rollback_criteria_builder.rs` (580_rollback_criteria_builder.rs)
- `src/520_violation_predictor.rs` (520_violation_predictor.rs) depends on `src/570_verification_scope_planner.rs` (570_verification_scope_planner.rs)
- `src/050_cluster_010.rs` (050_cluster_010.rs) depends on `src/090_dependency.rs` (090_dependency.rs)
- `src/050_cluster_010.rs` (050_cluster_010.rs) depends on `src/160_layer_utilities.rs` (160_layer_utilities.rs)
- `src/010_cluster_008.rs` (010_cluster_008.rs) depends on `src/090_dependency.rs` (090_dependency.rs)
- `src/010_cluster_008.rs` (010_cluster_008.rs) depends on `src/220_types.rs` (220_types.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/440_dead_code_actions.rs` (440_dead_code_actions.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/211_dead_code_attribute_parser.rs` (211_dead_code_attribute_parser.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/380_dead_code_call_graph.rs` (380_dead_code_call_graph.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/490_dead_code_cli.rs` (490_dead_code_cli.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/430_dead_code_confidence.rs` (430_dead_code_confidence.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/410_dead_code_entrypoints.rs` (410_dead_code_entrypoints.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/390_dead_code_intent.rs` (390_dead_code_intent.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/460_dead_code_report.rs` (460_dead_code_report.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/400_dead_code_test_boundaries.rs` (400_dead_code_test_boundaries.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/350_dead_code_types.rs` (350_dead_code_types.rs)
- `src/160_layer_utilities.rs` (160_layer_utilities.rs) depends on `src/220_types.rs` (220_types.rs)

### Directory Order
- `src`

## Julia File Ordering

No files analyzed.

