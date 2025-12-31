# Call Graph Analysis

This document shows the **interprocedural call graph** - which functions call which other functions.

> **Note:** This is NOT a control flow graph (CFG). CFG shows intraprocedural control flow (branches, loops) within individual functions.

## Call Graph Statistics

- Total functions: 188
- Total function calls: 284
- Maximum call depth: 13
- Leaf functions (no outgoing calls): 88

## Call Graph Visualization

```mermaid
graph TD
    src_000_cluster_001_rs__build_directory_entry_map["src/000_cluster_001.rs::build_directory_entry_map"]
    src_000_cluster_001_rs__collect_naming_warnings["src/000_cluster_001.rs::collect_naming_warnings"]
    src_000_cluster_001_rs__layer_constrained_sort["src/000_cluster_001.rs::layer_constrained_sort"]
    src_000_cluster_001_rs__topo_sort_within["src/000_cluster_001.rs::topo_sort_within"]
    src_000_cluster_001_rs__detect_layer["src/000_cluster_001.rs::detect_layer"]
    src_000_cluster_001_rs__rust_entry_paths["src/000_cluster_001.rs::rust_entry_paths"]
    src_000_cluster_001_rs__collect_rust_dependencies["src/000_cluster_001.rs::collect_rust_dependencies"]
    src_000_cluster_001_rs__order_rust_files_by_dependency["src/000_cluster_001.rs::order_rust_files_by_dependency"]
    src_000_cluster_001_rs__collect_julia_dependencies["src/000_cluster_001.rs::collect_julia_dependencies"]
    src_000_cluster_001_rs__julia_entry_paths["src/000_cluster_001.rs::julia_entry_paths"]
    src_000_cluster_001_rs__build_file_layers["src/000_cluster_001.rs::build_file_layers"]
    src_000_cluster_001_rs__gather_julia_files["src/000_cluster_001.rs::gather_julia_files"]
    src_000_cluster_001_rs__topological_sort["src/000_cluster_001.rs::topological_sort"]
    src_000_cluster_001_rs__ordered_by_name["src/000_cluster_001.rs::ordered_by_name"]
    src_000_cluster_001_rs__build_entries["src/000_cluster_001.rs::build_entries"]
    src_000_cluster_001_rs__analyze_file_ordering["src/000_cluster_001.rs::analyze_file_ordering"]
    src_000_cluster_001_rs__naming_score_for_file["src/000_cluster_001.rs::naming_score_for_file"]
    src_000_cluster_001_rs__detect_cycles["src/000_cluster_001.rs::detect_cycles"]
    src_000_cluster_001_rs__detect_violations["src/000_cluster_001.rs::detect_violations"]
    src_000_cluster_001_rs__export_complete_program_dot["src/000_cluster_001.rs::export_complete_program_dot"]
    src_000_cluster_001_rs__order_julia_files_by_dependency["src/000_cluster_001.rs::order_julia_files_by_dependency"]
    src_000_cluster_001_rs__run_analysis["src/000_cluster_001.rs::run_analysis"]
    src_010_cluster_008_rs__build_result["src/010_cluster_008.rs::build_result"]
    src_010_cluster_008_rs__adjacency_from_edges["src/010_cluster_008.rs::adjacency_from_edges"]
    src_010_cluster_008_rs__topo_sort["src/010_cluster_008.rs::topo_sort"]
    src_010_cluster_008_rs__layer_rank_map["src/010_cluster_008.rs::layer_rank_map"]
    src_010_cluster_008_rs__insert_sorted["src/010_cluster_008.rs::insert_sorted"]
    src_010_cluster_008_rs__is_mmsb_main["src/010_cluster_008.rs::is_mmsb_main"]
    src_010_cluster_008_rs__is_layer_violation["src/010_cluster_008.rs::is_layer_violation"]
    src_010_cluster_008_rs__layer_prefix_value["src/010_cluster_008.rs::layer_prefix_value"]
    src_010_cluster_008_rs__compare_dir_layers["src/010_cluster_008.rs::compare_dir_layers"]
    src_010_cluster_008_rs__compare_path_components["src/010_cluster_008.rs::compare_path_components"]
    src_010_cluster_008_rs__layer_adheres["src/010_cluster_008.rs::layer_adheres"]
    src_010_cluster_008_rs__structural_layer_value["src/010_cluster_008.rs::structural_layer_value"]
    src_010_cluster_008_rs__detect_layer_violation["src/010_cluster_008.rs::detect_layer_violation"]
    src_010_cluster_008_rs__parse_cluster_members["src/010_cluster_008.rs::parse_cluster_members"]
    src_010_cluster_008_rs__is_core_module_path["src/010_cluster_008.rs::is_core_module_path"]
    src_010_cluster_008_rs__cluster_target_path["src/010_cluster_008.rs::cluster_target_path"]
    src_010_cluster_008_rs__collect_cluster_plans["src/010_cluster_008.rs::collect_cluster_plans"]
    src_010_cluster_008_rs__node_style["src/010_cluster_008.rs::node_style"]
    src_010_cluster_008_rs__cyclomatic_complexity["src/010_cluster_008.rs::cyclomatic_complexity"]
    src_010_cluster_008_rs__structural_cmp["src/010_cluster_008.rs::structural_cmp"]
    src_010_cluster_008_rs__sort_structural_items["src/010_cluster_008.rs::sort_structural_items"]
    src_020_cluster_010_rs__gather_rust_files["src/020_cluster_010.rs::gather_rust_files"]
    src_030_refactor_constraints_rs__generate_constraints["src/030_refactor_constraints.rs::generate_constraints"]
    src_050_cluster_010_rs__normalize_module_name["src/050_cluster_010.rs::normalize_module_name"]
    src_050_cluster_010_rs__resolve_module["src/050_cluster_010.rs::resolve_module"]
    src_050_cluster_010_rs__contains_tools["src/050_cluster_010.rs::contains_tools"]
    src_050_cluster_010_rs__build_module_root_map["src/050_cluster_010.rs::build_module_root_map"]
    src_050_cluster_010_rs__extract_rust_dependencies["src/050_cluster_010.rs::extract_rust_dependencies"]
    src_050_cluster_010_rs__extract_julia_dependencies["src/050_cluster_010.rs::extract_julia_dependencies"]
    src_050_cluster_010_rs__resolve_module_name["src/050_cluster_010.rs::resolve_module_name"]
    src_050_cluster_010_rs__build_dependency_map["src/050_cluster_010.rs::build_dependency_map"]
    src_050_cluster_010_rs__extract_dependencies["src/050_cluster_010.rs::extract_dependencies"]
    src_070_cluster_011_rs__build_module_map["src/070_cluster_011.rs::build_module_map"]
    src_070_cluster_011_rs__resolve_path["src/070_cluster_011.rs::resolve_path"]
    src_070_cluster_011_rs__build_directory_dag["src/070_cluster_011.rs::build_directory_dag"]
    src_070_cluster_011_rs__build_file_dependency_graph["src/070_cluster_011.rs::build_file_dependency_graph"]
    src_070_cluster_011_rs__export_program_cfg_to_path["src/070_cluster_011.rs::export_program_cfg_to_path"]
    src_070_cluster_011_rs__build_file_dag["src/070_cluster_011.rs::build_file_dag"]
    src_090_dependency_rs__collect_roots["src/090_dependency.rs::collect_roots"]
    src_110_cluster_006_rs__layer_prefix_value["src/110_cluster_006.rs::layer_prefix_value"]
    src_110_cluster_006_rs__order_directories["src/110_cluster_006.rs::order_directories"]
    src_110_cluster_006_rs__common_root["src/110_cluster_006.rs::common_root"]
    src_110_cluster_006_rs__strip_numeric_prefix["src/110_cluster_006.rs::strip_numeric_prefix"]
    src_110_cluster_006_rs__generate_canonical_name["src/110_cluster_006.rs::generate_canonical_name"]
    src_110_cluster_006_rs__collect_directory_moves["src/110_cluster_006.rs::collect_directory_moves"]
    src_110_cluster_006_rs__compute_cohesion_score["src/110_cluster_006.rs::compute_cohesion_score"]
    src_160_layer_utilities_rs__resolve_source_root["src/160_layer_utilities.rs::resolve_source_root"]
    src_160_layer_utilities_rs__allow_analysis_dir["src/160_layer_utilities.rs::allow_analysis_dir"]
    src_160_layer_utilities_rs__main["src/160_layer_utilities.rs::main"]
    src_160_layer_utilities_rs__run_dead_code_pipeline["src/160_layer_utilities.rs::run_dead_code_pipeline"]
    src_170_invariant_reporter_rs__export_json["src/170_invariant_reporter.rs::export_json"]
    src_170_invariant_reporter_rs__export_constraints_json["src/170_invariant_reporter.rs::export_constraints_json"]
    src_180_conscience_graph_rs__generate_conscience_map["src/180_conscience_graph.rs::generate_conscience_map"]
    src_190_action_validator_rs__extract_layer["src/190_action_validator.rs::extract_layer"]
    src_190_action_validator_rs__validate_action["src/190_action_validator.rs::validate_action"]
    src_210_utilities_rs__compress_path["src/210_utilities.rs::compress_path"]
    src_210_utilities_rs__collect_directory_files["src/210_utilities.rs::collect_directory_files"]
    src_210_utilities_rs__path_common_prefix_len["src/210_utilities.rs::path_common_prefix_len"]
    src_210_utilities_rs__resolve_required_layer_path["src/210_utilities.rs::resolve_required_layer_path"]
    src_210_utilities_rs__compute_move_metrics["src/210_utilities.rs::compute_move_metrics"]
    src_210_utilities_rs__collect_move_items["src/210_utilities.rs::collect_move_items"]
    src_210_utilities_rs__write_structural_batches["src/210_utilities.rs::write_structural_batches"]
    src_210_utilities_rs__write_cluster_batches["src/210_utilities.rs::write_cluster_batches"]
    src_211_dead_code_attribute_parser_rs__parse_mmsb_latent_attr["src/211_dead_code_attribute_parser.rs::parse_mmsb_latent_attr"]
    src_211_dead_code_attribute_parser_rs__scan_file_attributes["src/211_dead_code_attribute_parser.rs::scan_file_attributes"]
    src_211_dead_code_attribute_parser_rs__extract_attribute_value["src/211_dead_code_attribute_parser.rs::extract_attribute_value"]
    src_211_dead_code_attribute_parser_rs__collect_latent_attrs["src/211_dead_code_attribute_parser.rs::collect_latent_attrs"]
    src_211_dead_code_attribute_parser_rs__marker_from_str["src/211_dead_code_attribute_parser.rs::marker_from_str"]
    src_211_dead_code_attribute_parser_rs__scan_intent_tags["src/211_dead_code_attribute_parser.rs::scan_intent_tags"]
    src_211_dead_code_attribute_parser_rs__scan_doc_comments["src/211_dead_code_attribute_parser.rs::scan_doc_comments"]
    src_211_dead_code_attribute_parser_rs__detect_intent_signals["src/211_dead_code_attribute_parser.rs::detect_intent_signals"]
    src_211_dead_code_attribute_parser_rs__is_cfg_test_item["src/211_dead_code_attribute_parser.rs::is_cfg_test_item"]
    src_211_dead_code_attribute_parser_rs__detect_test_modules["src/211_dead_code_attribute_parser.rs::detect_test_modules"]
    src_211_dead_code_attribute_parser_rs__detect_test_symbols["src/211_dead_code_attribute_parser.rs::detect_test_symbols"]
    src_260_file_ordering_rs__parallel_build_file_dag["src/260_file_ordering.rs::parallel_build_file_dag"]
    src_320_main_rs__main["src/320_main.rs::main"]
    src_330_agent_cli_rs__run_agent_cli["src/330_agent_cli.rs::run_agent_cli"]
    src_330_agent_cli_rs__check_action["src/330_agent_cli.rs::check_action"]
    src_330_agent_cli_rs__query_function["src/330_agent_cli.rs::query_function"]
    src_330_agent_cli_rs__list_invariants["src/330_agent_cli.rs::list_invariants"]
    src_330_agent_cli_rs__show_stats["src/330_agent_cli.rs::show_stats"]
    src_330_agent_cli_rs__load_invariants["src/330_agent_cli.rs::load_invariants"]
    src_370_dead_code_doc_comment_parser_rs__detect_latent_markers["src/370_dead_code_doc_comment_parser.rs::detect_latent_markers"]
    src_370_dead_code_doc_comment_parser_rs__merge_doc_intent["src/370_dead_code_doc_comment_parser.rs::merge_doc_intent"]
    src_370_dead_code_doc_comment_parser_rs__extract_doc_markers["src/370_dead_code_doc_comment_parser.rs::extract_doc_markers"]
    src_370_dead_code_doc_comment_parser_rs__item_name["src/370_dead_code_doc_comment_parser.rs::item_name"]
    src_370_dead_code_doc_comment_parser_rs__item_attrs["src/370_dead_code_doc_comment_parser.rs::item_attrs"]
    src_380_dead_code_call_graph_rs__build_call_graph["src/380_dead_code_call_graph.rs::build_call_graph"]
    src_380_dead_code_call_graph_rs__build_reverse_call_graph["src/380_dead_code_call_graph.rs::build_reverse_call_graph"]
    src_380_dead_code_call_graph_rs__compute_reachability["src/380_dead_code_call_graph.rs::compute_reachability"]
    src_380_dead_code_call_graph_rs__is_reachable["src/380_dead_code_call_graph.rs::is_reachable"]
    src_380_dead_code_call_graph_rs__classify_symbol["src/380_dead_code_call_graph.rs::classify_symbol"]
    src_380_dead_code_call_graph_rs__is_test_only["src/380_dead_code_call_graph.rs::is_test_only"]
    src_390_dead_code_intent_rs__check_planned_directory["src/390_dead_code_intent.rs::check_planned_directory"]
    src_390_dead_code_intent_rs__merge_intent_sources["src/390_dead_code_intent.rs::merge_intent_sources"]
    src_390_dead_code_intent_rs__planned_directory_intent["src/390_dead_code_intent.rs::planned_directory_intent"]
    src_390_dead_code_intent_rs__collect_symbols["src/390_dead_code_intent.rs::collect_symbols"]
    src_400_dead_code_test_boundaries_rs__find_test_callers["src/400_dead_code_test_boundaries.rs::find_test_callers"]
    src_400_dead_code_test_boundaries_rs__has_test_attr["src/400_dead_code_test_boundaries.rs::has_test_attr"]
    src_400_dead_code_test_boundaries_rs__item_attrs["src/400_dead_code_test_boundaries.rs::item_attrs"]
    src_410_dead_code_entrypoints_rs__collect_entrypoints["src/410_dead_code_entrypoints.rs::collect_entrypoints"]
    src_410_dead_code_entrypoints_rs__collect_exports["src/410_dead_code_entrypoints.rs::collect_exports"]
    src_410_dead_code_entrypoints_rs__is_public_api["src/410_dead_code_entrypoints.rs::is_public_api"]
    src_410_dead_code_entrypoints_rs__collect_use_tree_idents["src/410_dead_code_entrypoints.rs::collect_use_tree_idents"]
    src_410_dead_code_entrypoints_rs__treat_public_as_entrypoint["src/410_dead_code_entrypoints.rs::treat_public_as_entrypoint"]
    src_420_dead_code_classifier_rs__is_reachable["src/420_dead_code_classifier.rs::is_reachable"]
    src_430_dead_code_confidence_rs__assign_confidence["src/430_dead_code_confidence.rs::assign_confidence"]
    src_440_dead_code_actions_rs__recommend_action["src/440_dead_code_actions.rs::recommend_action"]
    src_460_dead_code_report_rs__build_report["src/460_dead_code_report.rs::build_report"]
    src_460_dead_code_report_rs__write_report["src/460_dead_code_report.rs::write_report"]
    src_460_dead_code_report_rs__build_basic_report["src/460_dead_code_report.rs::build_basic_report"]
    src_460_dead_code_report_rs__write_outputs["src/460_dead_code_report.rs::write_outputs"]
    src_470_dead_code_filter_rs__filter_dead_code_elements["src/470_dead_code_filter.rs::filter_dead_code_elements"]
    src_470_dead_code_filter_rs__should_exclude_from_analysis["src/470_dead_code_filter.rs::should_exclude_from_analysis"]
    src_470_dead_code_filter_rs__collect_excluded_symbols["src/470_dead_code_filter.rs::collect_excluded_symbols"]
    src_490_dead_code_cli_rs__merge_intent_map["src/490_dead_code_cli.rs::merge_intent_map"]
    src_490_dead_code_cli_rs__reason_for_category["src/490_dead_code_cli.rs::reason_for_category"]
    src_490_dead_code_cli_rs__is_test_path["src/490_dead_code_cli.rs::is_test_path"]
    src_510_dead_code_policy_rs__load_policy["src/510_dead_code_policy.rs::load_policy"]
    src_510_dead_code_policy_rs__parse_policy["src/510_dead_code_policy.rs::parse_policy"]
    src_510_dead_code_policy_rs__parse_list["src/510_dead_code_policy.rs::parse_list"]
    src_510_dead_code_policy_rs__parse_bool["src/510_dead_code_policy.rs::parse_bool"]
    src_520_violation_predictor_rs__predict_violations["src/520_violation_predictor.rs::predict_violations"]
    src_520_violation_predictor_rs__find_callers["src/520_violation_predictor.rs::find_callers"]
    src_520_violation_predictor_rs__find_reference_files["src/520_violation_predictor.rs::find_reference_files"]
    src_520_violation_predictor_rs__find_element_file["src/520_violation_predictor.rs::find_element_file"]
    src_520_violation_predictor_rs__symbol_exists["src/520_violation_predictor.rs::symbol_exists"]
    src_520_violation_predictor_rs__move_violates_invariant["src/520_violation_predictor.rs::move_violates_invariant"]
    src_520_violation_predictor_rs__generate_intelligence_report["src/520_violation_predictor.rs::generate_intelligence_report"]
    src_530_dead_code_report_split_rs__write_summary_markdown["src/530_dead_code_report_split.rs::write_summary_markdown"]
    src_530_dead_code_report_split_rs__write_plan_markdown["src/530_dead_code_report_split.rs::write_plan_markdown"]
    src_530_dead_code_report_split_rs__top_items["src/530_dead_code_report_split.rs::top_items"]
    src_530_dead_code_report_split_rs__plan_options["src/530_dead_code_report_split.rs::plan_options"]
    src_540_tier_classifier_rs__classify_tier["src/540_tier_classifier.rs::classify_tier"]
    src_550_confidence_scorer_rs__compute_confidence["src/550_confidence_scorer.rs::compute_confidence"]
    src_560_correction_plan_generator_rs__generate_correction_plan["src/560_correction_plan_generator.rs::generate_correction_plan"]
    src_560_correction_plan_generator_rs__average_confidence["src/560_correction_plan_generator.rs::average_confidence"]
    src_560_correction_plan_generator_rs__estimate_fix_time["src/560_correction_plan_generator.rs::estimate_fix_time"]
    src_560_correction_plan_generator_rs__action_symbol["src/560_correction_plan_generator.rs::action_symbol"]
    src_560_correction_plan_generator_rs__action_function["src/560_correction_plan_generator.rs::action_function"]
    src_560_correction_plan_generator_rs__action_module_path["src/560_correction_plan_generator.rs::action_module_path"]
    src_560_correction_plan_generator_rs__action_refs["src/560_correction_plan_generator.rs::action_refs"]
    src_560_correction_plan_generator_rs__action_target_layer["src/560_correction_plan_generator.rs::action_target_layer"]
    src_560_correction_plan_generator_rs__action_visibility["src/560_correction_plan_generator.rs::action_visibility"]
    src_570_verification_scope_planner_rs__plan_verification_scope["src/570_verification_scope_planner.rs::plan_verification_scope"]
    src_570_verification_scope_planner_rs__affected_files["src/570_verification_scope_planner.rs::affected_files"]
    src_570_verification_scope_planner_rs__action_module["src/570_verification_scope_planner.rs::action_module"]
    src_570_verification_scope_planner_rs__estimate_verification_time["src/570_verification_scope_planner.rs::estimate_verification_time"]
    src_580_rollback_criteria_builder_rs__build_rollback_criteria["src/580_rollback_criteria_builder.rs::build_rollback_criteria"]
    src_580_rollback_criteria_builder_rs__extract_critical_tests["src/580_rollback_criteria_builder.rs::extract_critical_tests"]
    src_590_quality_delta_calculator_rs__calculate_quality_delta["src/590_quality_delta_calculator.rs::calculate_quality_delta"]
    src_590_quality_delta_calculator_rs__estimate_impact["src/590_quality_delta_calculator.rs::estimate_impact"]
    src_600_action_impact_estimator_rs__simulate_action["src/600_action_impact_estimator.rs::simulate_action"]
    src_610_correction_plan_serializer_rs__serialize_correction_plan["src/610_correction_plan_serializer.rs::serialize_correction_plan"]
    src_610_correction_plan_serializer_rs__serialize_correction_plans["src/610_correction_plan_serializer.rs::serialize_correction_plans"]
    src_610_correction_plan_serializer_rs__write_intelligence_outputs_at["src/610_correction_plan_serializer.rs::write_intelligence_outputs_at"]
    src_620_verification_policy_emitter_rs__emit_verification_policy["src/620_verification_policy_emitter.rs::emit_verification_policy"]
    src_630_correction_intelligence_report_rs__build_state["src/630_correction_intelligence_report.rs::build_state"]
    src_630_correction_intelligence_report_rs__write_intelligence_outputs["src/630_correction_intelligence_report.rs::write_intelligence_outputs"]
    src_630_correction_intelligence_report_rs__filter_path_coherence_report["src/630_correction_intelligence_report.rs::filter_path_coherence_report"]
    src_630_correction_intelligence_report_rs__filter_visibility_report["src/630_correction_intelligence_report.rs::filter_visibility_report"]
    src_630_correction_intelligence_report_rs__augment_path_coherence_strategies["src/630_correction_intelligence_report.rs::augment_path_coherence_strategies"]
    src_630_correction_intelligence_report_rs__module_name_from_path["src/630_correction_intelligence_report.rs::module_name_from_path"]
    src_630_correction_intelligence_report_rs__compute_summary["src/630_correction_intelligence_report.rs::compute_summary"]
    src_630_correction_intelligence_report_rs__fill_prediction_confidence["src/630_correction_intelligence_report.rs::fill_prediction_confidence"]
    src_630_correction_intelligence_report_rs__default_confidence["src/630_correction_intelligence_report.rs::default_confidence"]
    src_000_cluster_001_rs__build_directory_entry_map --> src_070_cluster_011_rs__build_module_map
    src_000_cluster_001_rs__build_directory_entry_map --> src_050_cluster_010_rs__build_dependency_map
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__build_file_layers
    src_000_cluster_001_rs__build_directory_entry_map --> src_070_cluster_011_rs__build_file_dag
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__detect_cycles
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__layer_constrained_sort
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__topological_sort
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__ordered_by_name
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__ordered_by_name
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__build_entries
    src_000_cluster_001_rs__collect_naming_warnings --> src_000_cluster_001_rs__build_directory_entry_map
    src_000_cluster_001_rs__collect_naming_warnings --> src_000_cluster_001_rs__naming_score_for_file
    src_000_cluster_001_rs__collect_naming_warnings --> src_000_cluster_001_rs__collect_naming_warnings
    src_000_cluster_001_rs__layer_constrained_sort --> src_010_cluster_008_rs__layer_prefix_value
    src_000_cluster_001_rs__layer_constrained_sort --> src_110_cluster_006_rs__layer_prefix_value
    src_000_cluster_001_rs__layer_constrained_sort --> src_000_cluster_001_rs__topo_sort_within
    src_000_cluster_001_rs__rust_entry_paths --> src_160_layer_utilities_rs__resolve_source_root
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_050_cluster_010_rs__build_module_root_map
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_000_cluster_001_rs__rust_entry_paths
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_000_cluster_001_rs__detect_layer
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_000_cluster_001_rs__collect_rust_dependencies
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_010_cluster_008_rs__build_result
    src_000_cluster_001_rs__julia_entry_paths --> src_160_layer_utilities_rs__resolve_source_root
    src_000_cluster_001_rs__build_file_layers --> src_000_cluster_001_rs__detect_layer
    src_000_cluster_001_rs__gather_julia_files --> src_160_layer_utilities_rs__resolve_source_root
    src_000_cluster_001_rs__gather_julia_files --> src_160_layer_utilities_rs__allow_analysis_dir
    src_000_cluster_001_rs__build_entries --> src_110_cluster_006_rs__generate_canonical_name
    src_000_cluster_001_rs__analyze_file_ordering --> src_070_cluster_011_rs__build_module_map
    src_000_cluster_001_rs__analyze_file_ordering --> src_050_cluster_010_rs__build_dependency_map
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__build_file_layers
    src_000_cluster_001_rs__analyze_file_ordering --> src_110_cluster_006_rs__order_directories
    src_000_cluster_001_rs__analyze_file_ordering --> src_070_cluster_011_rs__build_file_dag
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__detect_cycles
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__layer_constrained_sort
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__topological_sort
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__ordered_by_name
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__ordered_by_name
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__build_entries
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__detect_violations
    src_000_cluster_001_rs__export_complete_program_dot --> src_010_cluster_008_rs__cyclomatic_complexity
    src_000_cluster_001_rs__export_complete_program_dot --> src_010_cluster_008_rs__node_style
    src_000_cluster_001_rs__order_julia_files_by_dependency --> src_000_cluster_001_rs__julia_entry_paths
    src_000_cluster_001_rs__order_julia_files_by_dependency --> src_000_cluster_001_rs__detect_layer
    src_000_cluster_001_rs__order_julia_files_by_dependency --> src_000_cluster_001_rs__collect_julia_dependencies
    src_000_cluster_001_rs__order_julia_files_by_dependency --> src_000_cluster_001_rs__detect_layer
    src_000_cluster_001_rs__order_julia_files_by_dependency --> src_050_cluster_010_rs__resolve_module
    src_000_cluster_001_rs__order_julia_files_by_dependency --> src_010_cluster_008_rs__build_result
    src_000_cluster_001_rs__run_analysis --> src_020_cluster_010_rs__gather_rust_files
    src_000_cluster_001_rs__run_analysis --> src_000_cluster_001_rs__order_rust_files_by_dependency
    src_000_cluster_001_rs__run_analysis --> src_000_cluster_001_rs__analyze_file_ordering
    src_000_cluster_001_rs__run_analysis --> src_000_cluster_001_rs__gather_julia_files
    src_000_cluster_001_rs__run_analysis --> src_000_cluster_001_rs__order_julia_files_by_dependency
    src_000_cluster_001_rs__run_analysis --> src_510_dead_code_policy_rs__load_policy
    src_000_cluster_001_rs__run_analysis --> src_160_layer_utilities_rs__run_dead_code_pipeline
    src_000_cluster_001_rs__run_analysis --> src_470_dead_code_filter_rs__filter_dead_code_elements
    src_000_cluster_001_rs__run_analysis --> src_380_dead_code_call_graph_rs__build_call_graph
    src_000_cluster_001_rs__run_analysis --> src_030_refactor_constraints_rs__generate_constraints
    src_000_cluster_001_rs__run_analysis --> src_070_cluster_011_rs__export_program_cfg_to_path
    src_000_cluster_001_rs__run_analysis --> src_170_invariant_reporter_rs__export_constraints_json
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__adjacency_from_edges
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__topo_sort
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__layer_rank_map
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__is_mmsb_main
    src_010_cluster_008_rs__build_result --> src_160_layer_utilities_rs__main
    src_010_cluster_008_rs__build_result --> src_320_main_rs__main
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__is_mmsb_main
    src_010_cluster_008_rs__build_result --> src_160_layer_utilities_rs__main
    src_010_cluster_008_rs__build_result --> src_320_main_rs__main
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__is_layer_violation
    src_010_cluster_008_rs__topo_sort --> src_010_cluster_008_rs__insert_sorted
    src_010_cluster_008_rs__is_layer_violation --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__is_layer_violation --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__is_layer_violation --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__is_layer_violation --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_dir_layers --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_dir_layers --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_dir_layers --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_dir_layers --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_path_components --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_path_components --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_path_components --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_path_components --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__layer_adheres --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__layer_adheres --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__layer_adheres --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__layer_adheres --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__structural_layer_value --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__structural_layer_value --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violation --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violation --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violation --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violation --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__cluster_target_path --> src_010_cluster_008_rs__is_core_module_path
    src_010_cluster_008_rs__cluster_target_path --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__cluster_target_path --> src_110_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__collect_cluster_plans --> src_010_cluster_008_rs__parse_cluster_members
    src_010_cluster_008_rs__collect_cluster_plans --> src_010_cluster_008_rs__cluster_target_path
    src_010_cluster_008_rs__structural_cmp --> src_010_cluster_008_rs__structural_layer_value
    src_010_cluster_008_rs__structural_cmp --> src_010_cluster_008_rs__structural_layer_value
    src_010_cluster_008_rs__structural_cmp --> src_010_cluster_008_rs__structural_layer_value
    src_010_cluster_008_rs__structural_cmp --> src_010_cluster_008_rs__structural_layer_value
    src_010_cluster_008_rs__sort_structural_items --> src_010_cluster_008_rs__structural_layer_value
    src_010_cluster_008_rs__sort_structural_items --> src_010_cluster_008_rs__structural_layer_value
    src_010_cluster_008_rs__sort_structural_items --> src_010_cluster_008_rs__structural_cmp
    src_020_cluster_010_rs__gather_rust_files --> src_160_layer_utilities_rs__resolve_source_root
    src_020_cluster_010_rs__gather_rust_files --> src_160_layer_utilities_rs__allow_analysis_dir
    src_050_cluster_010_rs__resolve_module --> src_050_cluster_010_rs__normalize_module_name
    src_050_cluster_010_rs__resolve_module --> src_070_cluster_011_rs__resolve_path
    src_050_cluster_010_rs__build_module_root_map --> src_050_cluster_010_rs__contains_tools
    src_050_cluster_010_rs__build_module_root_map --> src_050_cluster_010_rs__normalize_module_name
    src_050_cluster_010_rs__build_module_root_map --> src_000_cluster_001_rs__detect_layer
    src_050_cluster_010_rs__extract_rust_dependencies --> src_090_dependency_rs__collect_roots
    src_050_cluster_010_rs__extract_rust_dependencies --> src_050_cluster_010_rs__resolve_module
    src_050_cluster_010_rs__extract_rust_dependencies --> src_050_cluster_010_rs__resolve_module
    src_050_cluster_010_rs__extract_julia_dependencies --> src_050_cluster_010_rs__resolve_module
    src_050_cluster_010_rs__extract_julia_dependencies --> src_070_cluster_011_rs__resolve_path
    src_050_cluster_010_rs__extract_julia_dependencies --> src_050_cluster_010_rs__resolve_module_name
    src_050_cluster_010_rs__extract_julia_dependencies --> src_050_cluster_010_rs__resolve_module_name
    src_050_cluster_010_rs__extract_julia_dependencies --> src_050_cluster_010_rs__resolve_module_name
    src_050_cluster_010_rs__extract_julia_dependencies --> src_050_cluster_010_rs__resolve_module_name
    src_050_cluster_010_rs__resolve_module_name --> src_050_cluster_010_rs__resolve_module
    src_050_cluster_010_rs__build_dependency_map --> src_050_cluster_010_rs__extract_dependencies
    src_050_cluster_010_rs__extract_dependencies --> src_050_cluster_010_rs__extract_rust_dependencies
    src_050_cluster_010_rs__extract_dependencies --> src_050_cluster_010_rs__extract_julia_dependencies
    src_070_cluster_011_rs__build_module_map --> src_050_cluster_010_rs__normalize_module_name
    src_070_cluster_011_rs__build_module_map --> src_050_cluster_010_rs__normalize_module_name
    src_070_cluster_011_rs__resolve_path --> src_050_cluster_010_rs__normalize_module_name
    src_070_cluster_011_rs__build_directory_dag --> src_070_cluster_011_rs__build_module_map
    src_070_cluster_011_rs__build_directory_dag --> src_050_cluster_010_rs__build_dependency_map
    src_070_cluster_011_rs__build_directory_dag --> src_070_cluster_011_rs__build_file_dag
    src_070_cluster_011_rs__build_file_dependency_graph --> src_070_cluster_011_rs__build_module_map
    src_070_cluster_011_rs__build_file_dependency_graph --> src_050_cluster_010_rs__build_dependency_map
    src_070_cluster_011_rs__build_file_dependency_graph --> src_070_cluster_011_rs__build_file_dag
    src_070_cluster_011_rs__export_program_cfg_to_path --> src_000_cluster_001_rs__export_complete_program_dot
    src_090_dependency_rs__collect_roots --> src_090_dependency_rs__collect_roots
    src_090_dependency_rs__collect_roots --> src_090_dependency_rs__collect_roots
    src_110_cluster_006_rs__order_directories --> src_110_cluster_006_rs__common_root
    src_110_cluster_006_rs__order_directories --> src_010_cluster_008_rs__compare_path_components
    src_110_cluster_006_rs__generate_canonical_name --> src_110_cluster_006_rs__strip_numeric_prefix
    src_110_cluster_006_rs__collect_directory_moves --> src_010_cluster_008_rs__compare_dir_layers
    src_110_cluster_006_rs__collect_directory_moves --> src_110_cluster_006_rs__strip_numeric_prefix
    src_110_cluster_006_rs__compute_cohesion_score --> src_010_cluster_008_rs__layer_adheres
    src_160_layer_utilities_rs__main --> src_000_cluster_001_rs__run_analysis
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_020_cluster_010_rs__gather_rust_files
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_211_dead_code_attribute_parser_rs__detect_intent_signals
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_490_dead_code_cli_rs__merge_intent_map
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_211_dead_code_attribute_parser_rs__detect_test_modules
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_211_dead_code_attribute_parser_rs__detect_test_symbols
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_490_dead_code_cli_rs__is_test_path
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_380_dead_code_call_graph_rs__build_call_graph
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_410_dead_code_entrypoints_rs__collect_entrypoints
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_410_dead_code_entrypoints_rs__collect_exports
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_380_dead_code_call_graph_rs__classify_symbol
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_380_dead_code_call_graph_rs__is_reachable
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_420_dead_code_classifier_rs__is_reachable
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_490_dead_code_cli_rs__reason_for_category
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_430_dead_code_confidence_rs__assign_confidence
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_410_dead_code_entrypoints_rs__is_public_api
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_440_dead_code_actions_rs__recommend_action
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_460_dead_code_report_rs__build_report
    src_160_layer_utilities_rs__run_dead_code_pipeline --> src_460_dead_code_report_rs__write_outputs
    src_190_action_validator_rs__validate_action --> src_190_action_validator_rs__extract_layer
    src_190_action_validator_rs__validate_action --> src_190_action_validator_rs__extract_layer
    src_210_utilities_rs__collect_directory_files --> src_210_utilities_rs__collect_directory_files
    src_210_utilities_rs__resolve_required_layer_path --> src_210_utilities_rs__collect_directory_files
    src_210_utilities_rs__resolve_required_layer_path --> src_210_utilities_rs__path_common_prefix_len
    src_210_utilities_rs__collect_move_items --> src_210_utilities_rs__compute_move_metrics
    src_210_utilities_rs__collect_move_items --> src_210_utilities_rs__compress_path
    src_210_utilities_rs__collect_move_items --> src_210_utilities_rs__resolve_required_layer_path
    src_210_utilities_rs__collect_move_items --> src_210_utilities_rs__compress_path
    src_210_utilities_rs__collect_move_items --> src_210_utilities_rs__compute_move_metrics
    src_210_utilities_rs__write_structural_batches --> src_210_utilities_rs__compress_path
    src_210_utilities_rs__write_structural_batches --> src_210_utilities_rs__compress_path
    src_210_utilities_rs__write_cluster_batches --> src_210_utilities_rs__compress_path
    src_211_dead_code_attribute_parser_rs__parse_mmsb_latent_attr --> src_370_dead_code_doc_comment_parser_rs__item_name
    src_211_dead_code_attribute_parser_rs__parse_mmsb_latent_attr --> src_211_dead_code_attribute_parser_rs__collect_latent_attrs
    src_211_dead_code_attribute_parser_rs__parse_mmsb_latent_attr --> src_370_dead_code_doc_comment_parser_rs__item_attrs
    src_211_dead_code_attribute_parser_rs__parse_mmsb_latent_attr --> src_400_dead_code_test_boundaries_rs__item_attrs
    src_211_dead_code_attribute_parser_rs__scan_file_attributes --> src_370_dead_code_doc_comment_parser_rs__item_name
    src_211_dead_code_attribute_parser_rs__scan_file_attributes --> src_211_dead_code_attribute_parser_rs__collect_latent_attrs
    src_211_dead_code_attribute_parser_rs__scan_file_attributes --> src_370_dead_code_doc_comment_parser_rs__item_attrs
    src_211_dead_code_attribute_parser_rs__scan_file_attributes --> src_400_dead_code_test_boundaries_rs__item_attrs
    src_211_dead_code_attribute_parser_rs__collect_latent_attrs --> src_211_dead_code_attribute_parser_rs__marker_from_str
    src_211_dead_code_attribute_parser_rs__scan_intent_tags --> src_211_dead_code_attribute_parser_rs__parse_mmsb_latent_attr
    src_211_dead_code_attribute_parser_rs__scan_intent_tags --> src_211_dead_code_attribute_parser_rs__scan_doc_comments
    src_211_dead_code_attribute_parser_rs__scan_intent_tags --> src_390_dead_code_intent_rs__check_planned_directory
    src_211_dead_code_attribute_parser_rs__scan_intent_tags --> src_390_dead_code_intent_rs__collect_symbols
    src_211_dead_code_attribute_parser_rs__scan_doc_comments --> src_370_dead_code_doc_comment_parser_rs__item_name
    src_211_dead_code_attribute_parser_rs__scan_doc_comments --> src_370_dead_code_doc_comment_parser_rs__extract_doc_markers
    src_211_dead_code_attribute_parser_rs__scan_doc_comments --> src_370_dead_code_doc_comment_parser_rs__item_attrs
    src_211_dead_code_attribute_parser_rs__scan_doc_comments --> src_400_dead_code_test_boundaries_rs__item_attrs
    src_211_dead_code_attribute_parser_rs__detect_intent_signals --> src_211_dead_code_attribute_parser_rs__parse_mmsb_latent_attr
    src_211_dead_code_attribute_parser_rs__detect_intent_signals --> src_211_dead_code_attribute_parser_rs__scan_doc_comments
    src_211_dead_code_attribute_parser_rs__detect_intent_signals --> src_370_dead_code_doc_comment_parser_rs__merge_doc_intent
    src_211_dead_code_attribute_parser_rs__detect_intent_signals --> src_390_dead_code_intent_rs__planned_directory_intent
    src_211_dead_code_attribute_parser_rs__detect_intent_signals --> src_390_dead_code_intent_rs__merge_intent_sources
    src_211_dead_code_attribute_parser_rs__is_cfg_test_item --> src_370_dead_code_doc_comment_parser_rs__item_attrs
    src_211_dead_code_attribute_parser_rs__is_cfg_test_item --> src_400_dead_code_test_boundaries_rs__item_attrs
    src_211_dead_code_attribute_parser_rs__detect_test_modules --> src_211_dead_code_attribute_parser_rs__is_cfg_test_item
    src_211_dead_code_attribute_parser_rs__detect_test_symbols --> src_400_dead_code_test_boundaries_rs__has_test_attr
    src_211_dead_code_attribute_parser_rs__detect_test_symbols --> src_211_dead_code_attribute_parser_rs__is_cfg_test_item
    src_260_file_ordering_rs__parallel_build_file_dag --> src_070_cluster_011_rs__build_directory_dag
    src_320_main_rs__main --> src_330_agent_cli_rs__run_agent_cli
    src_320_main_rs__main --> src_160_layer_utilities_rs__main
    src_320_main_rs__main --> src_320_main_rs__main
    src_330_agent_cli_rs__run_agent_cli --> src_330_agent_cli_rs__check_action
    src_330_agent_cli_rs__run_agent_cli --> src_330_agent_cli_rs__query_function
    src_330_agent_cli_rs__run_agent_cli --> src_330_agent_cli_rs__list_invariants
    src_330_agent_cli_rs__run_agent_cli --> src_330_agent_cli_rs__show_stats
    src_330_agent_cli_rs__check_action --> src_330_agent_cli_rs__load_invariants
    src_330_agent_cli_rs__check_action --> src_330_agent_cli_rs__check_action
    src_330_agent_cli_rs__query_function --> src_330_agent_cli_rs__load_invariants
    src_330_agent_cli_rs__list_invariants --> src_330_agent_cli_rs__load_invariants
    src_330_agent_cli_rs__show_stats --> src_330_agent_cli_rs__load_invariants
    src_370_dead_code_doc_comment_parser_rs__extract_doc_markers --> src_370_dead_code_doc_comment_parser_rs__detect_latent_markers
    src_380_dead_code_call_graph_rs__is_reachable --> src_380_dead_code_call_graph_rs__compute_reachability
    src_380_dead_code_call_graph_rs__classify_symbol --> src_380_dead_code_call_graph_rs__is_test_only
    src_380_dead_code_call_graph_rs__classify_symbol --> src_380_dead_code_call_graph_rs__is_reachable
    src_380_dead_code_call_graph_rs__classify_symbol --> src_420_dead_code_classifier_rs__is_reachable
    src_380_dead_code_call_graph_rs__is_test_only --> src_380_dead_code_call_graph_rs__build_reverse_call_graph
    src_390_dead_code_intent_rs__planned_directory_intent --> src_390_dead_code_intent_rs__check_planned_directory
    src_390_dead_code_intent_rs__planned_directory_intent --> src_390_dead_code_intent_rs__collect_symbols
    src_400_dead_code_test_boundaries_rs__find_test_callers --> src_380_dead_code_call_graph_rs__build_reverse_call_graph
    src_410_dead_code_entrypoints_rs__collect_entrypoints --> src_410_dead_code_entrypoints_rs__treat_public_as_entrypoint
    src_410_dead_code_entrypoints_rs__collect_exports --> src_410_dead_code_entrypoints_rs__collect_use_tree_idents
    src_410_dead_code_entrypoints_rs__collect_use_tree_idents --> src_410_dead_code_entrypoints_rs__collect_use_tree_idents
    src_410_dead_code_entrypoints_rs__collect_use_tree_idents --> src_410_dead_code_entrypoints_rs__collect_use_tree_idents
    src_420_dead_code_classifier_rs__is_reachable --> src_380_dead_code_call_graph_rs__compute_reachability
    src_460_dead_code_report_rs__write_outputs --> src_460_dead_code_report_rs__write_report
    src_460_dead_code_report_rs__write_outputs --> src_530_dead_code_report_split_rs__write_summary_markdown
    src_460_dead_code_report_rs__write_outputs --> src_530_dead_code_report_split_rs__write_plan_markdown
    src_470_dead_code_filter_rs__filter_dead_code_elements --> src_470_dead_code_filter_rs__collect_excluded_symbols
    src_470_dead_code_filter_rs__collect_excluded_symbols --> src_470_dead_code_filter_rs__should_exclude_from_analysis
    src_510_dead_code_policy_rs__load_policy --> src_510_dead_code_policy_rs__parse_policy
    src_510_dead_code_policy_rs__parse_policy --> src_510_dead_code_policy_rs__parse_list
    src_510_dead_code_policy_rs__parse_policy --> src_510_dead_code_policy_rs__parse_list
    src_510_dead_code_policy_rs__parse_policy --> src_510_dead_code_policy_rs__parse_list
    src_510_dead_code_policy_rs__parse_policy --> src_510_dead_code_policy_rs__parse_bool
    src_520_violation_predictor_rs__predict_violations --> src_520_violation_predictor_rs__find_callers
    src_520_violation_predictor_rs__predict_violations --> src_520_violation_predictor_rs__move_violates_invariant
    src_520_violation_predictor_rs__predict_violations --> src_520_violation_predictor_rs__symbol_exists
    src_520_violation_predictor_rs__predict_violations --> src_520_violation_predictor_rs__find_reference_files
    src_520_violation_predictor_rs__find_callers --> src_520_violation_predictor_rs__find_element_file
    src_520_violation_predictor_rs__find_reference_files --> src_520_violation_predictor_rs__find_element_file
    src_520_violation_predictor_rs__generate_intelligence_report --> src_520_violation_predictor_rs__predict_violations
    src_520_violation_predictor_rs__generate_intelligence_report --> src_630_correction_intelligence_report_rs__fill_prediction_confidence
    src_520_violation_predictor_rs__generate_intelligence_report --> src_560_correction_plan_generator_rs__generate_correction_plan
    src_520_violation_predictor_rs__generate_intelligence_report --> src_630_correction_intelligence_report_rs__augment_path_coherence_strategies
    src_520_violation_predictor_rs__generate_intelligence_report --> src_570_verification_scope_planner_rs__plan_verification_scope
    src_520_violation_predictor_rs__generate_intelligence_report --> src_580_rollback_criteria_builder_rs__build_rollback_criteria
    src_520_violation_predictor_rs__generate_intelligence_report --> src_590_quality_delta_calculator_rs__estimate_impact
    src_520_violation_predictor_rs__generate_intelligence_report --> src_630_correction_intelligence_report_rs__compute_summary
    src_530_dead_code_report_split_rs__write_summary_markdown --> src_530_dead_code_report_split_rs__top_items
    src_530_dead_code_report_split_rs__write_plan_markdown --> src_530_dead_code_report_split_rs__top_items
    src_530_dead_code_report_split_rs__write_plan_markdown --> src_530_dead_code_report_split_rs__plan_options
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_symbol
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_module_path
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_refs
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_refs
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_symbol
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_target_layer
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_function
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_function
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_target_layer
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__action_visibility
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__average_confidence
    src_560_correction_plan_generator_rs__generate_correction_plan --> src_560_correction_plan_generator_rs__estimate_fix_time
    src_570_verification_scope_planner_rs__plan_verification_scope --> src_570_verification_scope_planner_rs__affected_files
    src_570_verification_scope_planner_rs__plan_verification_scope --> src_570_verification_scope_planner_rs__action_module
    src_570_verification_scope_planner_rs__plan_verification_scope --> src_570_verification_scope_planner_rs__estimate_verification_time
    src_580_rollback_criteria_builder_rs__build_rollback_criteria --> src_580_rollback_criteria_builder_rs__extract_critical_tests
    src_590_quality_delta_calculator_rs__estimate_impact --> src_600_action_impact_estimator_rs__simulate_action
    src_590_quality_delta_calculator_rs__estimate_impact --> src_590_quality_delta_calculator_rs__calculate_quality_delta
    src_610_correction_plan_serializer_rs__serialize_correction_plans --> src_610_correction_plan_serializer_rs__serialize_correction_plan
    src_610_correction_plan_serializer_rs__write_intelligence_outputs_at --> src_610_correction_plan_serializer_rs__serialize_correction_plans
    src_610_correction_plan_serializer_rs__write_intelligence_outputs_at --> src_620_verification_policy_emitter_rs__emit_verification_policy
    src_630_correction_intelligence_report_rs__write_intelligence_outputs --> src_610_correction_plan_serializer_rs__write_intelligence_outputs_at
    src_630_correction_intelligence_report_rs__filter_path_coherence_report --> src_630_correction_intelligence_report_rs__compute_summary
    src_630_correction_intelligence_report_rs__filter_visibility_report --> src_630_correction_intelligence_report_rs__compute_summary
    src_630_correction_intelligence_report_rs__augment_path_coherence_strategies --> src_630_correction_intelligence_report_rs__module_name_from_path
    src_630_correction_intelligence_report_rs__augment_path_coherence_strategies --> src_630_correction_intelligence_report_rs__module_name_from_path
    src_630_correction_intelligence_report_rs__augment_path_coherence_strategies --> src_020_cluster_010_rs__gather_rust_files
    src_630_correction_intelligence_report_rs__module_name_from_path --> src_050_cluster_010_rs__normalize_module_name
    src_630_correction_intelligence_report_rs__fill_prediction_confidence --> src_630_correction_intelligence_report_rs__default_confidence
```
