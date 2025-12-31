## Phase 4: Cohesion Improvements

Action: optional: improve cohesion by moving functions to better-fit modules.
Note: safe to defer unless you are actively refactoring.

- `scan_doc_comments` from `src/211_dead_code_attribute_parser.rs` to `-`: cohesion 0.10 below threshold 0.60 (impact 0.50)
- `detect_intent_signals` from `src/211_dead_code_attribute_parser.rs` to `-`: cohesion 0.30 below threshold 0.60 (impact 0.30)
- `estimate_impact` from `src/590_quality_delta_calculator.rs` to `-`: cohesion 0.35 below threshold 0.60 (impact 0.25)
- `generate_intelligence_report` from `src/520_violation_predictor.rs` to `-`: cohesion 0.16 below threshold 0.60 (impact 0.44)
- `parse_mmsb_latent_attr` from `src/211_dead_code_attribute_parser.rs` to `-`: cohesion 0.23 below threshold 0.60 (impact 0.37)
- `run_analysis` from `src/000_cluster_001.rs` to `-`: cohesion 0.20 below threshold 0.60 (impact 0.40)
- `scan_file_attributes` from `src/211_dead_code_attribute_parser.rs` to `-`: cohesion 0.23 below threshold 0.60 (impact 0.37)
- `scan_intent_tags` from `src/211_dead_code_attribute_parser.rs` to `-`: cohesion 0.35 below threshold 0.60 (impact 0.25)
- `write_intelligence_outputs_at` from `src/610_correction_plan_serializer.rs` to `-`: cohesion 0.35 below threshold 0.60 (impact 0.25)
- `build_directory_entry_map` from `src/000_cluster_001.rs` to `-`: cohesion 0.49 below threshold 0.60 (impact 0.11)
- `classify_symbol` from `src/380_dead_code_call_graph.rs` to `-`: cohesion 0.43 below threshold 0.60 (impact 0.17)
- `cluster_target_path` from `src/010_cluster_008.rs` to `-`: cohesion 0.43 below threshold 0.60 (impact 0.17)
- `is_cfg_test_item` from `src/211_dead_code_attribute_parser.rs` to `-`: cohesion 0.40 below threshold 0.60 (impact 0.20)
- `order_julia_files_by_dependency` from `src/000_cluster_001.rs` to `-`: cohesion 0.43 below threshold 0.60 (impact 0.17)
- `write_outputs` from `src/460_dead_code_report.rs` to `-`: cohesion 0.42 below threshold 0.60 (impact 0.18)
- `layer_constrained_sort` from `src/000_cluster_001.rs` to `-`: cohesion 0.57 below threshold 0.60 (impact 0.03)

