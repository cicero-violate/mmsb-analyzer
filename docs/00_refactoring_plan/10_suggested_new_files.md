## Suggested New Files (Clusters)

Action: consider creating these files to improve cohesion.
Note: suggestions are heuristic and should be validated.

- cohesion 0.67, suggested `src/330_agent_cli.rs`
  - src/330_agent_cli.rs::run_agent_cli, src/330_agent_cli.rs::query_function, src/330_agent_cli.rs::list_invariants, src/330_agent_cli.rs::load_invariants
- cohesion 1.00, suggested `src/470_dead_code_filter.rs`
  - src/470_dead_code_filter.rs::filter_dead_code_elements, src/470_dead_code_filter.rs::should_exclude_from_analysis, src/470_dead_code_filter.rs::collect_excluded_symbols
- cohesion 1.00, suggested `src/510_dead_code_policy.rs`
  - src/510_dead_code_policy.rs::load_policy, src/510_dead_code_policy.rs::parse_policy, src/510_dead_code_policy.rs::parse_list, src/510_dead_code_policy.rs::parse_bool
- cohesion 1.00, suggested `src/070_cluster_011.rs`
  - src/070_cluster_011.rs::build_module_map, src/070_cluster_011.rs::build_file_dependency_graph, src/070_cluster_011.rs::build_file_dag
- cohesion 0.41, suggested `src/000_cluster_001.rs`
  - src/000_cluster_001.rs::build_directory_entry_map, src/000_cluster_001.rs::collect_naming_warnings, src/000_cluster_001.rs::detect_layer, src/000_cluster_001.rs::order_rust_files_by_dependency, src/000_cluster_001.rs::build_entries, src/000_cluster_001.rs::analyze_file_ordering, src/000_cluster_001.rs::naming_score_for_file, src/000_cluster_001.rs::detect_cycles, src/000_cluster_001.rs::detect_violations
- cohesion 1.00, suggested `src/210_utilities.rs`
  - src/210_utilities.rs::compress_path, src/210_utilities.rs::collect_directory_files, src/210_utilities.rs::path_common_prefix_len, src/210_utilities.rs::resolve_required_layer_path, src/210_utilities.rs::compute_move_metrics, src/210_utilities.rs::collect_move_items, src/210_utilities.rs::write_structural_batches, src/210_utilities.rs::write_cluster_batches
- cohesion 0.60, suggested `src/380_dead_code_call_graph.rs`
  - src/020_cluster_010.rs::gather_rust_files, src/160_layer_utilities.rs::resolve_source_root, src/160_layer_utilities.rs::allow_analysis_dir, src/211_dead_code_attribute_parser.rs::is_cfg_test_item, src/380_dead_code_call_graph.rs::build_reverse_call_graph, src/380_dead_code_call_graph.rs::compute_reachability, src/400_dead_code_test_boundaries.rs::find_test_callers
- cohesion 0.00, suggested `src/050_cluster_010.rs`
  - src/000_cluster_001.rs::collect_julia_dependencies, src/050_cluster_010.rs::contains_tools, src/050_cluster_010.rs::extract_rust_dependencies, src/050_cluster_010.rs::build_dependency_map
- cohesion 0.40, suggested `src/390_dead_code_intent.rs`
  - src/211_dead_code_attribute_parser.rs::detect_intent_signals, src/370_dead_code_doc_comment_parser.rs::merge_doc_intent, src/390_dead_code_intent.rs::check_planned_directory, src/390_dead_code_intent.rs::merge_intent_sources
- cohesion 0.56, suggested `src/211_dead_code_attribute_parser.rs`
  - src/211_dead_code_attribute_parser.rs::scan_intent_tags, src/211_dead_code_attribute_parser.rs::scan_doc_comments, src/370_dead_code_doc_comment_parser.rs::extract_doc_markers, src/370_dead_code_doc_comment_parser.rs::item_name, src/390_dead_code_intent.rs::collect_symbols, src/400_dead_code_test_boundaries.rs::item_attrs
- cohesion 1.00, suggested `src/560_correction_plan_generator.rs`
  - src/520_violation_predictor.rs::predict_violations, src/520_violation_predictor.rs::find_callers, src/520_violation_predictor.rs::find_reference_files, src/520_violation_predictor.rs::find_element_file, src/520_violation_predictor.rs::symbol_exists, src/520_violation_predictor.rs::move_violates_invariant, src/520_violation_predictor.rs::generate_intelligence_report, src/560_correction_plan_generator.rs::generate_correction_plan, src/560_correction_plan_generator.rs::average_confidence, src/560_correction_plan_generator.rs::estimate_fix_time, src/560_correction_plan_generator.rs::action_symbol, src/560_correction_plan_generator.rs::action_function, src/560_correction_plan_generator.rs::action_module_path, src/560_correction_plan_generator.rs::action_refs, src/560_correction_plan_generator.rs::action_target_layer, src/560_correction_plan_generator.rs::action_visibility, src/570_verification_scope_planner.rs::plan_verification_scope, src/570_verification_scope_planner.rs::affected_files, src/570_verification_scope_planner.rs::action_module, src/570_verification_scope_planner.rs::estimate_verification_time, src/580_rollback_criteria_builder.rs::build_rollback_criteria, src/580_rollback_criteria_builder.rs::extract_critical_tests, src/590_quality_delta_calculator.rs::calculate_quality_delta, src/590_quality_delta_calculator.rs::estimate_impact, src/600_action_impact_estimator.rs::simulate_action, src/630_correction_intelligence_report.rs::filter_path_coherence_report, src/630_correction_intelligence_report.rs::filter_visibility_report, src/630_correction_intelligence_report.rs::augment_path_coherence_strategies, src/630_correction_intelligence_report.rs::module_name_from_path, src/630_correction_intelligence_report.rs::compute_summary, src/630_correction_intelligence_report.rs::fill_prediction_confidence, src/630_correction_intelligence_report.rs::default_confidence
- cohesion 0.67, suggested `src/380_dead_code_call_graph.rs`
  - src/000_cluster_001.rs::run_analysis, src/030_refactor_constraints.rs::generate_constraints, src/070_cluster_011.rs::export_program_cfg_to_path, src/160_layer_utilities.rs::main, src/380_dead_code_call_graph.rs::build_call_graph
- cohesion 0.07, suggested `src/010_cluster_008.rs`
  - src/000_cluster_001.rs::layer_constrained_sort, src/000_cluster_001.rs::topo_sort_within, src/010_cluster_008.rs::is_layer_violation, src/010_cluster_008.rs::compare_dir_layers, src/010_cluster_008.rs::layer_adheres, src/010_cluster_008.rs::parse_cluster_members, src/010_cluster_008.rs::is_core_module_path
- cohesion 0.60, suggested `src/530_dead_code_report_split.rs`
  - src/460_dead_code_report.rs::write_outputs, src/530_dead_code_report_split.rs::write_plan_markdown, src/530_dead_code_report_split.rs::top_items, src/530_dead_code_report_split.rs::plan_options
- cohesion 0.00, suggested `src/000_cluster_001.rs`
  - src/000_cluster_001.rs::rust_entry_paths, src/000_cluster_001.rs::collect_rust_dependencies, src/000_cluster_001.rs::build_file_layers, src/000_cluster_001.rs::topological_sort, src/000_cluster_001.rs::ordered_by_name
- cohesion 0.18, suggested `src/050_cluster_010.rs`
  - src/000_cluster_001.rs::order_julia_files_by_dependency, src/050_cluster_010.rs::normalize_module_name, src/050_cluster_010.rs::build_module_root_map, src/050_cluster_010.rs::extract_julia_dependencies, src/050_cluster_010.rs::extract_dependencies
- cohesion 0.85, suggested `src/010_cluster_008.rs`
  - src/010_cluster_008.rs::build_result, src/010_cluster_008.rs::adjacency_from_edges, src/010_cluster_008.rs::topo_sort, src/010_cluster_008.rs::layer_rank_map, src/010_cluster_008.rs::insert_sorted, src/010_cluster_008.rs::is_mmsb_main, src/010_cluster_008.rs::layer_prefix_value, src/010_cluster_008.rs::compare_path_components, src/010_cluster_008.rs::detect_layer_violation, src/010_cluster_008.rs::cluster_target_path, src/010_cluster_008.rs::collect_cluster_plans, src/110_cluster_006.rs::layer_prefix_value
- cohesion 0.73, suggested `src/410_dead_code_entrypoints.rs`
  - src/000_cluster_001.rs::gather_julia_files, src/160_layer_utilities.rs::run_dead_code_pipeline, src/211_dead_code_attribute_parser.rs::detect_test_modules, src/211_dead_code_attribute_parser.rs::detect_test_symbols, src/380_dead_code_call_graph.rs::is_reachable, src/380_dead_code_call_graph.rs::classify_symbol, src/380_dead_code_call_graph.rs::is_test_only, src/400_dead_code_test_boundaries.rs::has_test_attr, src/410_dead_code_entrypoints.rs::collect_entrypoints, src/410_dead_code_entrypoints.rs::collect_exports, src/410_dead_code_entrypoints.rs::is_public_api, src/410_dead_code_entrypoints.rs::collect_use_tree_idents, src/410_dead_code_entrypoints.rs::treat_public_as_entrypoint, src/420_dead_code_classifier.rs::is_reachable, src/430_dead_code_confidence.rs::assign_confidence, src/440_dead_code_actions.rs::recommend_action, src/460_dead_code_report.rs::build_report, src/490_dead_code_cli.rs::merge_intent_map, src/490_dead_code_cli.rs::reason_for_category, src/490_dead_code_cli.rs::is_test_path
- cohesion 1.00, suggested `src/110_cluster_006.rs`
  - src/110_cluster_006.rs::strip_numeric_prefix, src/110_cluster_006.rs::generate_canonical_name, src/110_cluster_006.rs::collect_directory_moves
- cohesion 0.45, suggested `src/211_dead_code_attribute_parser.rs`
  - src/211_dead_code_attribute_parser.rs::parse_mmsb_latent_attr, src/211_dead_code_attribute_parser.rs::scan_file_attributes, src/211_dead_code_attribute_parser.rs::collect_latent_attrs, src/211_dead_code_attribute_parser.rs::marker_from_str, src/370_dead_code_doc_comment_parser.rs::detect_latent_markers, src/370_dead_code_doc_comment_parser.rs::item_attrs, src/390_dead_code_intent.rs::planned_directory_intent
- cohesion 1.00, suggested `src/610_correction_plan_serializer.rs`
  - src/610_correction_plan_serializer.rs::serialize_correction_plan, src/610_correction_plan_serializer.rs::serialize_correction_plans, src/610_correction_plan_serializer.rs::write_intelligence_outputs_at, src/620_verification_policy_emitter.rs::emit_verification_policy, src/630_correction_intelligence_report.rs::write_intelligence_outputs

