## Phase 2: Cluster Extraction

Action: create the listed cluster files and move the grouped functions.
Note: use the batches below to keep changes small.

- Create cluster file `src/560_correction_plan_generator.rs` with 23 functions (cohesion 1.00)
- Create cluster file `src/610_correction_plan_serializer.rs` with 2 functions (cohesion 1.00)
- Create cluster file `src/410_dead_code_entrypoints.rs` with 15 functions (cohesion 0.73)
- Create cluster file `src/380_dead_code_call_graph.rs` with 4 functions (cohesion 0.67)
- Create cluster file `src/380_dead_code_call_graph.rs` with 5 functions (cohesion 0.60)
- Create cluster file `src/211_dead_code_attribute_parser.rs` with 4 functions (cohesion 0.56)
- Create cluster file `src/211_dead_code_attribute_parser.rs` with 3 functions (cohesion 0.45)
- Create cluster file `src/390_dead_code_intent.rs` with 2 functions (cohesion 0.40)
- Create cluster file `src/010_cluster_008.rs` with 2 functions (cohesion 0.07)

```bash
touch "src/560_correction_plan_generator.rs"
touch "src/610_correction_plan_serializer.rs"
touch "src/410_dead_code_entrypoints.rs"
touch "src/380_dead_code_call_graph.rs"
touch "src/380_dead_code_call_graph.rs"
touch "src/211_dead_code_attribute_parser.rs"
touch "src/211_dead_code_attribute_parser.rs"
touch "src/390_dead_code_intent.rs"
touch "src/010_cluster_008.rs"
```

### Phase 2 Tips

Action: apply these guidelines while executing Phase 2 batches.
Note: these are advisory, not checklist items.

- Extract clusters as a unit; avoid splitting a cluster across files.
- Prefer creating new files before moving functions to keep diffs small.
- After each batch, update imports and run tests to lock in behavior.

### Phase 2 Batches

Action: execute batches in order and verify after each batch.
Note: each batch creates or fills a cluster file.

#### Batch 1: target `src/560_correction_plan_generator.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 1.00, 23 functions
- Move `predict_violations` from `src/520_violation_predictor.rs`
- Move `find_callers` from `src/520_violation_predictor.rs`
- Move `find_reference_files` from `src/520_violation_predictor.rs`
- Move `find_element_file` from `src/520_violation_predictor.rs`
- Move `symbol_exists` from `src/520_violation_predictor.rs`
- Move `move_violates_invariant` from `src/520_violation_predictor.rs`
- Move `generate_intelligence_report` from `src/520_violation_predictor.rs`
- Move `plan_verification_scope` from `src/570_verification_scope_planner.rs`
- Move `affected_files` from `src/570_verification_scope_planner.rs`
- Move `action_module` from `src/570_verification_scope_planner.rs`
- Move `estimate_verification_time` from `src/570_verification_scope_planner.rs`
- Move `build_rollback_criteria` from `src/580_rollback_criteria_builder.rs`
- Move `extract_critical_tests` from `src/580_rollback_criteria_builder.rs`
- Move `calculate_quality_delta` from `src/590_quality_delta_calculator.rs`
- Move `estimate_impact` from `src/590_quality_delta_calculator.rs`
- Move `simulate_action` from `src/600_action_impact_estimator.rs`
- Move `filter_path_coherence_report` from `src/630_correction_intelligence_report.rs`
- Move `filter_visibility_report` from `src/630_correction_intelligence_report.rs`
- Move `augment_path_coherence_strategies` from `src/630_correction_intelligence_report.rs`
- Move `module_name_from_path` from `src/630_correction_intelligence_report.rs`
- Move `compute_summary` from `src/630_correction_intelligence_report.rs`
- Move `fill_prediction_confidence` from `src/630_correction_intelligence_report.rs`
- Move `default_confidence` from `src/630_correction_intelligence_report.rs`
- Verification gate: `cargo test`

```bash
rg -n "predict_violations" "src/520_violation_predictor.rs"
rg -n "predict_violations" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "find_callers" "src/520_violation_predictor.rs"
rg -n "find_callers" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "find_reference_files" "src/520_violation_predictor.rs"
rg -n "find_reference_files" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "find_element_file" "src/520_violation_predictor.rs"
rg -n "find_element_file" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "symbol_exists" "src/520_violation_predictor.rs"
rg -n "symbol_exists" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "move_violates_invariant" "src/520_violation_predictor.rs"
rg -n "move_violates_invariant" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "generate_intelligence_report" "src/520_violation_predictor.rs"
rg -n "generate_intelligence_report" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "plan_verification_scope" "src/570_verification_scope_planner.rs"
rg -n "plan_verification_scope" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "affected_files" "src/570_verification_scope_planner.rs"
rg -n "affected_files" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "action_module" "src/570_verification_scope_planner.rs"
rg -n "action_module" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "estimate_verification_time" "src/570_verification_scope_planner.rs"
rg -n "estimate_verification_time" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "build_rollback_criteria" "src/580_rollback_criteria_builder.rs"
rg -n "build_rollback_criteria" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "extract_critical_tests" "src/580_rollback_criteria_builder.rs"
rg -n "extract_critical_tests" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "calculate_quality_delta" "src/590_quality_delta_calculator.rs"
rg -n "calculate_quality_delta" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "estimate_impact" "src/590_quality_delta_calculator.rs"
rg -n "estimate_impact" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "simulate_action" "src/600_action_impact_estimator.rs"
rg -n "simulate_action" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "filter_path_coherence_report" "src/630_correction_intelligence_report.rs"
rg -n "filter_path_coherence_report" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "filter_visibility_report" "src/630_correction_intelligence_report.rs"
rg -n "filter_visibility_report" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "augment_path_coherence_strategies" "src/630_correction_intelligence_report.rs"
rg -n "augment_path_coherence_strategies" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "module_name_from_path" "src/630_correction_intelligence_report.rs"
rg -n "module_name_from_path" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "compute_summary" "src/630_correction_intelligence_report.rs"
rg -n "compute_summary" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "fill_prediction_confidence" "src/630_correction_intelligence_report.rs"
rg -n "fill_prediction_confidence" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "default_confidence" "src/630_correction_intelligence_report.rs"
rg -n "default_confidence" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 2: target `src/610_correction_plan_serializer.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 1.00, 2 functions
- Move `emit_verification_policy` from `src/620_verification_policy_emitter.rs`
- Move `write_intelligence_outputs` from `src/630_correction_intelligence_report.rs`
- Verification gate: `cargo test`

```bash
rg -n "emit_verification_policy" "src/620_verification_policy_emitter.rs"
rg -n "emit_verification_policy" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "write_intelligence_outputs" "src/630_correction_intelligence_report.rs"
rg -n "write_intelligence_outputs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 3: target `src/410_dead_code_entrypoints.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.73, 15 functions
- Move `gather_julia_files` from `src/000_cluster_001.rs`
- Move `run_dead_code_pipeline` from `src/160_layer_utilities.rs`
- Move `detect_test_modules` from `src/211_dead_code_attribute_parser.rs`
- Move `detect_test_symbols` from `src/211_dead_code_attribute_parser.rs`
- Move `is_reachable` from `src/380_dead_code_call_graph.rs`
- Move `classify_symbol` from `src/380_dead_code_call_graph.rs`
- Move `is_test_only` from `src/380_dead_code_call_graph.rs`
- Move `has_test_attr` from `src/400_dead_code_test_boundaries.rs`
- Move `is_reachable` from `src/420_dead_code_classifier.rs`
- Move `assign_confidence` from `src/430_dead_code_confidence.rs`
- Move `recommend_action` from `src/440_dead_code_actions.rs`
- Move `build_report` from `src/460_dead_code_report.rs`
- Move `merge_intent_map` from `src/490_dead_code_cli.rs`
- Move `reason_for_category` from `src/490_dead_code_cli.rs`
- Move `is_test_path` from `src/490_dead_code_cli.rs`
- Verification gate: `cargo test`

```bash
rg -n "gather_julia_files" "src/000_cluster_001.rs"
rg -n "gather_julia_files" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "run_dead_code_pipeline" "src/160_layer_utilities.rs"
rg -n "run_dead_code_pipeline" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "detect_test_modules" "src/211_dead_code_attribute_parser.rs"
rg -n "detect_test_modules" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "detect_test_symbols" "src/211_dead_code_attribute_parser.rs"
rg -n "detect_test_symbols" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "is_reachable" "src/380_dead_code_call_graph.rs"
rg -n "is_reachable" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "classify_symbol" "src/380_dead_code_call_graph.rs"
rg -n "classify_symbol" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "is_test_only" "src/380_dead_code_call_graph.rs"
rg -n "is_test_only" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "has_test_attr" "src/400_dead_code_test_boundaries.rs"
rg -n "has_test_attr" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "is_reachable" "src/420_dead_code_classifier.rs"
rg -n "is_reachable" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "assign_confidence" "src/430_dead_code_confidence.rs"
rg -n "assign_confidence" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "recommend_action" "src/440_dead_code_actions.rs"
rg -n "recommend_action" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "build_report" "src/460_dead_code_report.rs"
rg -n "build_report" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "merge_intent_map" "src/490_dead_code_cli.rs"
rg -n "merge_intent_map" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "reason_for_category" "src/490_dead_code_cli.rs"
rg -n "reason_for_category" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "is_test_path" "src/490_dead_code_cli.rs"
rg -n "is_test_path" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 4: target `src/380_dead_code_call_graph.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.67, 4 functions
- Move `run_analysis` from `src/000_cluster_001.rs`
- Move `generate_constraints` from `src/030_refactor_constraints.rs`
- Move `export_program_cfg_to_path` from `src/070_cluster_011.rs`
- Move `main` from `src/160_layer_utilities.rs`
- Verification gate: `cargo test`

```bash
rg -n "run_analysis" "src/000_cluster_001.rs"
rg -n "run_analysis" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "generate_constraints" "src/030_refactor_constraints.rs"
rg -n "generate_constraints" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "export_program_cfg_to_path" "src/070_cluster_011.rs"
rg -n "export_program_cfg_to_path" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "main" "src/160_layer_utilities.rs"
rg -n "main" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 5: target `src/380_dead_code_call_graph.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.60, 5 functions
- Move `gather_rust_files` from `src/020_cluster_010.rs`
- Move `resolve_source_root` from `src/160_layer_utilities.rs`
- Move `allow_analysis_dir` from `src/160_layer_utilities.rs`
- Move `is_cfg_test_item` from `src/211_dead_code_attribute_parser.rs`
- Move `find_test_callers` from `src/400_dead_code_test_boundaries.rs`
- Verification gate: `cargo test`

```bash
rg -n "gather_rust_files" "src/020_cluster_010.rs"
rg -n "gather_rust_files" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "resolve_source_root" "src/160_layer_utilities.rs"
rg -n "resolve_source_root" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "allow_analysis_dir" "src/160_layer_utilities.rs"
rg -n "allow_analysis_dir" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "is_cfg_test_item" "src/211_dead_code_attribute_parser.rs"
rg -n "is_cfg_test_item" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "find_test_callers" "src/400_dead_code_test_boundaries.rs"
rg -n "find_test_callers" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 6: target `src/211_dead_code_attribute_parser.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.56, 4 functions
- Move `extract_doc_markers` from `src/370_dead_code_doc_comment_parser.rs`
- Move `item_name` from `src/370_dead_code_doc_comment_parser.rs`
- Move `collect_symbols` from `src/390_dead_code_intent.rs`
- Move `item_attrs` from `src/400_dead_code_test_boundaries.rs`
- Verification gate: `cargo test`

```bash
rg -n "extract_doc_markers" "src/370_dead_code_doc_comment_parser.rs"
rg -n "extract_doc_markers" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "item_name" "src/370_dead_code_doc_comment_parser.rs"
rg -n "item_name" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "collect_symbols" "src/390_dead_code_intent.rs"
rg -n "collect_symbols" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "item_attrs" "src/400_dead_code_test_boundaries.rs"
rg -n "item_attrs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 7: target `src/211_dead_code_attribute_parser.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.45, 3 functions
- Move `detect_latent_markers` from `src/370_dead_code_doc_comment_parser.rs`
- Move `item_attrs` from `src/370_dead_code_doc_comment_parser.rs`
- Move `planned_directory_intent` from `src/390_dead_code_intent.rs`
- Verification gate: `cargo test`

```bash
rg -n "detect_latent_markers" "src/370_dead_code_doc_comment_parser.rs"
rg -n "detect_latent_markers" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "item_attrs" "src/370_dead_code_doc_comment_parser.rs"
rg -n "item_attrs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "planned_directory_intent" "src/390_dead_code_intent.rs"
rg -n "planned_directory_intent" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 8: target `src/390_dead_code_intent.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.40, 2 functions
- Move `detect_intent_signals` from `src/211_dead_code_attribute_parser.rs`
- Move `merge_doc_intent` from `src/370_dead_code_doc_comment_parser.rs`
- Verification gate: `cargo test`

```bash
rg -n "detect_intent_signals" "src/211_dead_code_attribute_parser.rs"
rg -n "detect_intent_signals" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "merge_doc_intent" "src/370_dead_code_doc_comment_parser.rs"
rg -n "merge_doc_intent" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 9: target `src/010_cluster_008.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.07, 2 functions
- Move `layer_constrained_sort` from `src/000_cluster_001.rs`
- Move `topo_sort_within` from `src/000_cluster_001.rs`
- Verification gate: `cargo test`

```bash
rg -n "layer_constrained_sort" "src/000_cluster_001.rs"
rg -n "layer_constrained_sort" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "topo_sort_within" "src/000_cluster_001.rs"
rg -n "topo_sort_within" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

