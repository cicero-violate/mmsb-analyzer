# Call Graph Analysis

This document shows the **interprocedural call graph** - which functions call which other functions.

> **Note:** This is NOT a control flow graph (CFG). CFG shows intraprocedural control flow (branches, loops) within individual functions.

## Call Graph Statistics

- Total functions: 236
- Total function calls: 254
- Maximum call depth: 13
- Leaf functions (no outgoing calls): 132

## Call Graph Visualization

```mermaid
graph TD
    src_000_cluster_001_rs__build_directory_entry_map["src/000_cluster_001.rs::build_directory_entry_map"]
    src_000_cluster_001_rs__collect_naming_warnings["src/000_cluster_001.rs::collect_naming_warnings"]
    src_000_cluster_001_rs__temp_dir["src/000_cluster_001.rs::temp_dir"]
    src_000_cluster_001_rs__detects_cycles["src/000_cluster_001.rs::detects_cycles"]
    src_000_cluster_001_rs__generates_canonical_names_and_violations["src/000_cluster_001.rs::generates_canonical_names_and_violations"]
    src_000_cluster_001_rs__topo_sort_orders_dependencies["src/000_cluster_001.rs::topo_sort_orders_dependencies"]
    src_000_cluster_001_rs__layer_constrained_sort["src/000_cluster_001.rs::layer_constrained_sort"]
    src_000_cluster_001_rs__topo_sort_within["src/000_cluster_001.rs::topo_sort_within"]
    src_000_cluster_001_rs__detect_layer["src/000_cluster_001.rs::detect_layer"]
    src_000_cluster_001_rs__rust_entry_paths["src/000_cluster_001.rs::rust_entry_paths"]
    src_000_cluster_001_rs__collect_rust_dependencies["src/000_cluster_001.rs::collect_rust_dependencies"]
    src_000_cluster_001_rs__collect_roots_from_crate["src/000_cluster_001.rs::collect_roots_from_crate"]
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
    src_000_cluster_001_rs__escape_dot["src/000_cluster_001.rs::escape_dot"]
    src_000_cluster_001_rs__test_detects_cycles["src/000_cluster_001.rs::test_detects_cycles"]
    src_000_cluster_001_rs__test_generates_canonical_names_and_violations["src/000_cluster_001.rs::test_generates_canonical_names_and_violations"]
    src_000_invariant_types_rs__test_confidence_from_strength["src/000_invariant_types.rs::test_confidence_from_strength"]
    src_000_invariant_types_rs__test_is_blocking["src/000_invariant_types.rs::test_is_blocking"]
    src_000_invariant_types_rs__test_stats_calculation["src/000_invariant_types.rs::test_stats_calculation"]
    src_005_refactor_constraints_rs__from_invariant["src/005_refactor_constraints.rs::from_invariant"]
    src_005_refactor_constraints_rs__check_move_allowed["src/005_refactor_constraints.rs::check_move_allowed"]
    src_005_refactor_constraints_rs__generate_constraints["src/005_refactor_constraints.rs::generate_constraints"]
    src_005_refactor_constraints_rs__test_from_invariant_layer_fixed["src/005_refactor_constraints.rs::test_from_invariant_layer_fixed"]
    src_005_refactor_constraints_rs__test_check_move_allowed_blocking["src/005_refactor_constraints.rs::test_check_move_allowed_blocking"]
    src_005_refactor_constraints_rs__test_check_move_allowed_non_blocking["src/005_refactor_constraints.rs::test_check_move_allowed_non_blocking"]
    src_005_refactor_constraints_rs__test_constraint_is_blocking["src/005_refactor_constraints.rs::test_constraint_is_blocking"]
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
    src_010_cluster_008_rs__detect_layer_violations["src/010_cluster_008.rs::detect_layer_violations"]
    src_010_cluster_008_rs__detect_layer_violation["src/010_cluster_008.rs::detect_layer_violation"]
    src_010_cluster_008_rs__parse_cluster_members["src/010_cluster_008.rs::parse_cluster_members"]
    src_010_cluster_008_rs__is_core_module_path["src/010_cluster_008.rs::is_core_module_path"]
    src_010_cluster_008_rs__cluster_target_path["src/010_cluster_008.rs::cluster_target_path"]
    src_010_cluster_008_rs__collect_cluster_plans["src/010_cluster_008.rs::collect_cluster_plans"]
    src_010_cluster_008_rs__node_style["src/010_cluster_008.rs::node_style"]
    src_010_cluster_008_rs__cyclomatic_complexity["src/010_cluster_008.rs::cyclomatic_complexity"]
    src_010_scc_compressor_rs__test_scc_compression_dag["src/010_scc_compressor.rs::test_scc_compression_dag"]
    src_010_scc_compressor_rs__test_scc_compression_cycle["src/010_scc_compressor.rs::test_scc_compression_cycle"]
    src_010_scc_compressor_rs__test_scc_compression_mixed["src/010_scc_compressor.rs::test_scc_compression_mixed"]
    src_020_cluster_010_rs__normalize_module_name["src/020_cluster_010.rs::normalize_module_name"]
    src_020_cluster_010_rs__resolve_module["src/020_cluster_010.rs::resolve_module"]
    src_020_cluster_010_rs__contains_tools["src/020_cluster_010.rs::contains_tools"]
    src_020_cluster_010_rs__build_module_root_map["src/020_cluster_010.rs::build_module_root_map"]
    src_020_cluster_010_rs__resolve_source_root["src/020_cluster_010.rs::resolve_source_root"]
    src_020_cluster_010_rs__order_julia_files_by_dependency["src/020_cluster_010.rs::order_julia_files_by_dependency"]
    src_020_cluster_010_rs__extract_rust_dependencies["src/020_cluster_010.rs::extract_rust_dependencies"]
    src_020_cluster_010_rs__extract_julia_dependencies["src/020_cluster_010.rs::extract_julia_dependencies"]
    src_020_cluster_010_rs__resolve_module_name["src/020_cluster_010.rs::resolve_module_name"]
    src_020_cluster_010_rs__build_dependency_map["src/020_cluster_010.rs::build_dependency_map"]
    src_020_cluster_010_rs__extract_dependencies["src/020_cluster_010.rs::extract_dependencies"]
    src_020_layer_inference_rs__infer_layers["src/020_layer_inference.rs::infer_layers"]
    src_020_layer_inference_rs__detect_layer_violations["src/020_layer_inference.rs::detect_layer_violations"]
    src_020_layer_inference_rs__test_layer_inference_simple_dag["src/020_layer_inference.rs::test_layer_inference_simple_dag"]
    src_020_layer_inference_rs__test_layer_inference_diamond["src/020_layer_inference.rs::test_layer_inference_diamond"]
    src_020_layer_inference_rs__test_detect_layer_violations_none["src/020_layer_inference.rs::test_detect_layer_violations_none"]
    src_030_cluster_011_rs__build_module_map["src/030_cluster_011.rs::build_module_map"]
    src_030_cluster_011_rs__resolve_path["src/030_cluster_011.rs::resolve_path"]
    src_030_cluster_011_rs__build_directory_dag["src/030_cluster_011.rs::build_directory_dag"]
    src_030_cluster_011_rs__build_file_dependency_graph["src/030_cluster_011.rs::build_file_dependency_graph"]
    src_030_cluster_011_rs__export_program_cfg_to_path["src/030_cluster_011.rs::export_program_cfg_to_path"]
    src_030_cluster_011_rs__build_file_dag["src/030_cluster_011.rs::build_file_dag"]
    src_030_fixpoint_solver_rs__propagate_to_fixpoint["src/030_fixpoint_solver.rs::propagate_to_fixpoint"]
    src_030_fixpoint_solver_rs__test_symbolic_abstraction_merge["src/030_fixpoint_solver.rs::test_symbolic_abstraction_merge"]
    src_030_fixpoint_solver_rs__test_fixpoint_simple["src/030_fixpoint_solver.rs::test_fixpoint_simple"]
    src_030_fixpoint_solver_rs__test_fixpoint_convergence["src/030_fixpoint_solver.rs::test_fixpoint_convergence"]
    src_040_dependency_rs__collect_roots["src/040_dependency.rs::collect_roots"]
    src_040_structural_detector_rs__test_detect_leaf_root["src/040_structural_detector.rs::test_detect_leaf_root"]
    src_040_structural_detector_rs__test_detect_degree_stable["src/040_structural_detector.rs::test_detect_degree_stable"]
    src_040_structural_detector_rs__test_all_structural_invariants_proven["src/040_structural_detector.rs::test_all_structural_invariants_proven"]
    src_050_cluster_006_rs__layer_prefix_value["src/050_cluster_006.rs::layer_prefix_value"]
    src_050_cluster_006_rs__order_directories["src/050_cluster_006.rs::order_directories"]
    src_050_cluster_006_rs__common_root["src/050_cluster_006.rs::common_root"]
    src_050_cluster_006_rs__strip_numeric_prefix["src/050_cluster_006.rs::strip_numeric_prefix"]
    src_050_cluster_006_rs__generate_canonical_name["src/050_cluster_006.rs::generate_canonical_name"]
    src_050_cluster_006_rs__collect_directory_moves["src/050_cluster_006.rs::collect_directory_moves"]
    src_050_cluster_006_rs__compute_cohesion_score["src/050_cluster_006.rs::compute_cohesion_score"]
    src_050_semantic_detector_rs__make_function["src/050_semantic_detector.rs::make_function"]
    src_050_semantic_detector_rs__test_detect_type_stable["src/050_semantic_detector.rs::test_detect_type_stable"]
    src_050_semantic_detector_rs__test_detect_pure_function_heuristic["src/050_semantic_detector.rs::test_detect_pure_function_heuristic"]
    src_050_semantic_detector_rs__test_detect_idempotent_heuristic["src/050_semantic_detector.rs::test_detect_idempotent_heuristic"]
    src_050_semantic_detector_rs__test_no_pure_for_mutable["src/050_semantic_detector.rs::test_no_pure_for_mutable"]
    src_060_layer_core_rs__structural_cmp["src/060_layer_core.rs::structural_cmp"]
    src_060_layer_core_rs__sort_structural_items["src/060_layer_core.rs::sort_structural_items"]
    src_060_path_detector_rs__test_path_detector_simple["src/060_path_detector.rs::test_path_detector_simple"]
    src_060_path_detector_rs__test_path_detector_diamond["src/060_path_detector.rs::test_path_detector_diamond"]
    src_060_path_detector_rs__test_extract_facts_from_path["src/060_path_detector.rs::test_extract_facts_from_path"]
    src_060_path_detector_rs__test_path_stats["src/060_path_detector.rs::test_path_stats"]
    src_070_invariant_integrator_rs__make_simple_analysis["src/070_invariant_integrator.rs::make_simple_analysis"]
    src_070_invariant_integrator_rs__test_invariant_detector_creation["src/070_invariant_integrator.rs::test_invariant_detector_creation"]
    src_070_invariant_integrator_rs__test_detect_all["src/070_invariant_integrator.rs::test_detect_all"]
    src_070_layer_utilities_rs__resolve_source_root["src/070_layer_utilities.rs::resolve_source_root"]
    src_070_layer_utilities_rs__allow_analysis_dir["src/070_layer_utilities.rs::allow_analysis_dir"]
    src_070_layer_utilities_rs__gather_rust_files["src/070_layer_utilities.rs::gather_rust_files"]
    src_070_layer_utilities_rs__main["src/070_layer_utilities.rs::main"]
    src_070_layer_utilities_rs__run_analysis["src/070_layer_utilities.rs::run_analysis"]
    src_080_invariant_reporter_rs__generate_invariant_report["src/080_invariant_reporter.rs::generate_invariant_report"]
    src_080_invariant_reporter_rs__export_json["src/080_invariant_reporter.rs::export_json"]
    src_080_invariant_reporter_rs__export_constraints_json["src/080_invariant_reporter.rs::export_constraints_json"]
    src_080_invariant_reporter_rs__test_generate_report["src/080_invariant_reporter.rs::test_generate_report"]
    src_082_conscience_graph_rs__generate_conscience_map["src/082_conscience_graph.rs::generate_conscience_map"]
    src_082_conscience_graph_rs__strength_emoji["src/082_conscience_graph.rs::strength_emoji"]
    src_082_conscience_graph_rs__kind_name["src/082_conscience_graph.rs::kind_name"]
    src_082_conscience_graph_rs__generate_conscience_stats["src/082_conscience_graph.rs::generate_conscience_stats"]
    src_082_conscience_graph_rs__make_test_invariant["src/082_conscience_graph.rs::make_test_invariant"]
    src_082_conscience_graph_rs__test_generate_stats["src/082_conscience_graph.rs::test_generate_stats"]
    src_082_conscience_graph_rs__test_strength_emoji["src/082_conscience_graph.rs::test_strength_emoji"]
    src_083_action_validator_rs__extract_layer["src/083_action_validator.rs::extract_layer"]
    src_083_action_validator_rs__matches_function["src/083_action_validator.rs::matches_function"]
    src_083_action_validator_rs__validate_action["src/083_action_validator.rs::validate_action"]
    src_083_action_validator_rs__check_move_allowed["src/083_action_validator.rs::check_move_allowed"]
    src_083_action_validator_rs__test_extract_layer["src/083_action_validator.rs::test_extract_layer"]
    src_083_action_validator_rs__test_validate_no_move_constraint["src/083_action_validator.rs::test_validate_no_move_constraint"]
    src_083_action_validator_rs__test_validate_layer_fixed_constraint["src/083_action_validator.rs::test_validate_layer_fixed_constraint"]
    src_083_action_validator_rs__test_validate_preserve_signature["src/083_action_validator.rs::test_validate_preserve_signature"]
    src_083_action_validator_rs__test_validate_allowed_action["src/083_action_validator.rs::test_validate_allowed_action"]
    src_083_action_validator_rs__test_check_move_allowed["src/083_action_validator.rs::test_check_move_allowed"]
    src_085_agent_conscience_rs__make_test_invariant["src/085_agent_conscience.rs::make_test_invariant"]
    src_085_agent_conscience_rs__test_conscience_blocks_invalid_move["src/085_agent_conscience.rs::test_conscience_blocks_invalid_move"]
    src_085_agent_conscience_rs__test_conscience_allows_valid_action["src/085_agent_conscience.rs::test_conscience_allows_valid_action"]
    src_085_agent_conscience_rs__test_conscience_stats["src/085_agent_conscience.rs::test_conscience_stats"]
    src_085_agent_conscience_rs__test_query_allowed_actions["src/085_agent_conscience.rs::test_query_allowed_actions"]
    src_090_utilities_rs__compress_path["src/090_utilities.rs::compress_path"]
    src_090_utilities_rs__collect_directory_files["src/090_utilities.rs::collect_directory_files"]
    src_090_utilities_rs__path_common_prefix_len["src/090_utilities.rs::path_common_prefix_len"]
    src_090_utilities_rs__resolve_required_layer_path["src/090_utilities.rs::resolve_required_layer_path"]
    src_090_utilities_rs__compute_move_metrics["src/090_utilities.rs::compute_move_metrics"]
    src_090_utilities_rs__collect_move_items["src/090_utilities.rs::collect_move_items"]
    src_090_utilities_rs__write_structural_batches["src/090_utilities.rs::write_structural_batches"]
    src_090_utilities_rs__write_cluster_batches["src/090_utilities.rs::write_cluster_batches"]
    src_110_cohesion_analyzer_rs__collect_functions["src/110_cohesion_analyzer.rs::collect_functions"]
    src_110_cohesion_analyzer_rs__build_call_edges["src/110_cohesion_analyzer.rs::build_call_edges"]
    src_110_cohesion_analyzer_rs__build_function_layers["src/110_cohesion_analyzer.rs::build_function_layers"]
    src_110_cohesion_analyzer_rs__build_type_maps["src/110_cohesion_analyzer.rs::build_type_maps"]
    src_110_cohesion_analyzer_rs__build_name_map["src/110_cohesion_analyzer.rs::build_name_map"]
    src_110_cohesion_analyzer_rs__build_call_analysis["src/110_cohesion_analyzer.rs::build_call_analysis"]
    src_110_cohesion_analyzer_rs__determine_status["src/110_cohesion_analyzer.rs::determine_status"]
    src_110_cohesion_analyzer_rs__suggest_file["src/110_cohesion_analyzer.rs::suggest_file"]
    src_110_cohesion_analyzer_rs__compute_cluster_cohesion["src/110_cohesion_analyzer.rs::compute_cluster_cohesion"]
    src_110_cohesion_analyzer_rs__suggest_cluster_file["src/110_cohesion_analyzer.rs::suggest_cluster_file"]
    src_110_cohesion_analyzer_rs__compute_type_coupling["src/110_cohesion_analyzer.rs::compute_type_coupling"]
    src_110_cohesion_analyzer_rs__extract_identifiers["src/110_cohesion_analyzer.rs::extract_identifiers"]
    src_110_cohesion_analyzer_rs__louvain_communities["src/110_cohesion_analyzer.rs::louvain_communities"]
    src_110_cohesion_analyzer_rs__build_undirected_graph["src/110_cohesion_analyzer.rs::build_undirected_graph"]
    src_120_directory_analyzer_rs__is_source_file["src/120_directory_analyzer.rs::is_source_file"]
    src_120_directory_analyzer_rs__should_skip_path["src/120_directory_analyzer.rs::should_skip_path"]
    src_130_control_flow_rs__sanitize_identifier["src/130_control_flow.rs::sanitize_identifier"]
    src_140_file_ordering_rs__parallel_build_file_dag["src/140_file_ordering.rs::parallel_build_file_dag"]
    src_150_julia_parser_rs__slugify_relative["src/150_julia_parser.rs::slugify_relative"]
    src_150_julia_parser_rs__resolve_julia_binary["src/150_julia_parser.rs::resolve_julia_binary"]
    src_150_julia_parser_rs__find_julia_project_dir["src/150_julia_parser.rs::find_julia_project_dir"]
    src_150_julia_parser_rs__parse_module_name["src/150_julia_parser.rs::parse_module_name"]
    src_150_julia_parser_rs__parse_struct_name["src/150_julia_parser.rs::parse_struct_name"]
    src_150_julia_parser_rs__relativize_path["src/150_julia_parser.rs::relativize_path"]
    src_150_julia_parser_rs__extract_calls_from_lines["src/150_julia_parser.rs::extract_calls_from_lines"]
    src_150_julia_parser_rs__extract_calls_from_text["src/150_julia_parser.rs::extract_calls_from_text"]
    src_150_julia_parser_rs__is_reserved["src/150_julia_parser.rs::is_reserved"]
    src_150_julia_parser_rs__paren_balance["src/150_julia_parser.rs::paren_balance"]
    src_160_rust_parser_rs__relativize_path["src/160_rust_parser.rs::relativize_path"]
    src_160_rust_parser_rs__expr_snippet["src/160_rust_parser.rs::expr_snippet"]
    src_160_rust_parser_rs__pat_snippet["src/160_rust_parser.rs::pat_snippet"]
    src_160_rust_parser_rs__truncate_label["src/160_rust_parser.rs::truncate_label"]
    src_180_report_rs__display_path["src/180_report.rs::display_path"]
    src_180_report_rs__placement_status_label["src/180_report.rs::placement_status_label"]
    src_180_report_rs__placement_status_notes["src/180_report.rs::placement_status_notes"]
    src_180_report_rs__collect_rename_items["src/180_report.rs::collect_rename_items"]
    src_180_report_rs__collect_utility_candidates["src/180_report.rs::collect_utility_candidates"]
    src_180_report_rs__directory_moves_to_plan["src/180_report.rs::directory_moves_to_plan"]
    src_180_report_rs__write_priority_section["src/180_report.rs::write_priority_section"]
    src_180_report_rs__write_structural_tips["src/180_report.rs::write_structural_tips"]
    src_180_report_rs__write_cluster_tips["src/180_report.rs::write_cluster_tips"]
    src_180_report_rs__sort_plan_items["src/180_report.rs::sort_plan_items"]
    src_180_report_rs__sort_cluster_items["src/180_report.rs::sort_cluster_items"]
    src_180_report_rs__cluster_priority["src/180_report.rs::cluster_priority"]
    src_180_report_rs__collect_cluster_items["src/180_report.rs::collect_cluster_items"]
    src_180_report_rs__load_cargo_warnings["src/180_report.rs::load_cargo_warnings"]
    src_180_report_rs__parse_dead_code_warnings["src/180_report.rs::parse_dead_code_warnings"]
    src_180_report_rs__parse_use_symbols["src/180_report.rs::parse_use_symbols"]
    src_180_report_rs__scan_crate_paths["src/180_report.rs::scan_crate_paths"]
    src_180_report_rs__collect_symbol_references["src/180_report.rs::collect_symbol_references"]
    src_180_report_rs__is_public_function["src/180_report.rs::is_public_function"]
    src_180_report_rs__path_matches["src/180_report.rs::path_matches"]
    src_180_report_rs__is_entrypoint_main["src/180_report.rs::is_entrypoint_main"]
    src_180_report_rs__referenced_elsewhere["src/180_report.rs::referenced_elsewhere"]
    src_180_report_rs__is_dead_code_candidate["src/180_report.rs::is_dead_code_candidate"]
    src_180_report_rs__filter_orphaned["src/180_report.rs::filter_orphaned"]
    src_180_report_rs__load_report_config["src/180_report.rs::load_report_config"]
    src_180_report_rs__collect_size_warnings["src/180_report.rs::collect_size_warnings"]
    src_180_report_rs__load_baseline_metrics["src/180_report.rs::load_baseline_metrics"]
    src_180_report_rs__baseline_deltas["src/180_report.rs::baseline_deltas"]
    src_180_report_rs__write_baseline_metrics["src/180_report.rs::write_baseline_metrics"]
    src_180_report_rs__collect_directories["src/180_report.rs::collect_directories"]
    src_180_report_rs__slugify_path["src/180_report.rs::slugify_path"]
    src_180_report_rs__render_mermaid_graph["src/180_report.rs::render_mermaid_graph"]
    src_180_report_rs__compute_ordering_correctness["src/180_report.rs::compute_ordering_correctness"]
    src_180_report_rs__compute_directory_cohesion["src/180_report.rs::compute_directory_cohesion"]
    src_180_report_rs__prefix_key_from_path["src/180_report.rs::prefix_key_from_path"]
    src_180_report_rs__slugify_key["src/180_report.rs::slugify_key"]
    src_180_report_rs__group_key_cmp["src/180_report.rs::group_key_cmp"]
    src_180_report_rs__function_bucket_label["src/180_report.rs::function_bucket_label"]
    src_180_report_rs__slugify_file_path["src/180_report.rs::slugify_file_path"]
    src_180_report_rs__language_label["src/180_report.rs::language_label"]
    src_180_report_rs__visibility_label["src/180_report.rs::visibility_label"]
    src_180_report_rs__short_signature["src/180_report.rs::short_signature"]
    src_180_report_rs__normalize_use_stmt["src/180_report.rs::normalize_use_stmt"]
    src_180_report_rs__sanitize_mermaid_id["src/180_report.rs::sanitize_mermaid_id"]
    src_180_report_rs__sanitize_mermaid_label["src/180_report.rs::sanitize_mermaid_label"]
    src_190_main_rs__main["src/190_main.rs::main"]
    src_191_agent_cli_rs__run_agent_cli["src/191_agent_cli.rs::run_agent_cli"]
    src_191_agent_cli_rs__check_action["src/191_agent_cli.rs::check_action"]
    src_191_agent_cli_rs__query_function["src/191_agent_cli.rs::query_function"]
    src_191_agent_cli_rs__list_invariants["src/191_agent_cli.rs::list_invariants"]
    src_191_agent_cli_rs__show_stats["src/191_agent_cli.rs::show_stats"]
    src_191_agent_cli_rs__load_invariants["src/191_agent_cli.rs::load_invariants"]
    src_191_agent_cli_rs__test_load_invariants_empty["src/191_agent_cli.rs::test_load_invariants_empty"]
    src_000_cluster_001_rs__build_directory_entry_map --> src_030_cluster_011_rs__build_module_map
    src_000_cluster_001_rs__build_directory_entry_map --> src_020_cluster_010_rs__build_dependency_map
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__build_file_layers
    src_000_cluster_001_rs__build_directory_entry_map --> src_030_cluster_011_rs__build_file_dag
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__detect_cycles
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__layer_constrained_sort
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__topological_sort
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__ordered_by_name
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__ordered_by_name
    src_000_cluster_001_rs__build_directory_entry_map --> src_000_cluster_001_rs__build_entries
    src_000_cluster_001_rs__collect_naming_warnings --> src_000_cluster_001_rs__build_directory_entry_map
    src_000_cluster_001_rs__collect_naming_warnings --> src_000_cluster_001_rs__naming_score_for_file
    src_000_cluster_001_rs__collect_naming_warnings --> src_000_cluster_001_rs__collect_naming_warnings
    src_000_cluster_001_rs__temp_dir --> src_000_cluster_001_rs__temp_dir
    src_000_cluster_001_rs__detects_cycles --> src_000_cluster_001_rs__temp_dir
    src_000_cluster_001_rs__detects_cycles --> src_000_cluster_001_rs__analyze_file_ordering
    src_000_cluster_001_rs__generates_canonical_names_and_violations --> src_000_cluster_001_rs__temp_dir
    src_000_cluster_001_rs__generates_canonical_names_and_violations --> src_000_cluster_001_rs__analyze_file_ordering
    src_000_cluster_001_rs__topo_sort_orders_dependencies --> src_000_cluster_001_rs__temp_dir
    src_000_cluster_001_rs__topo_sort_orders_dependencies --> src_000_cluster_001_rs__analyze_file_ordering
    src_000_cluster_001_rs__layer_constrained_sort --> src_010_cluster_008_rs__layer_prefix_value
    src_000_cluster_001_rs__layer_constrained_sort --> src_050_cluster_006_rs__layer_prefix_value
    src_000_cluster_001_rs__layer_constrained_sort --> src_000_cluster_001_rs__topo_sort_within
    src_000_cluster_001_rs__rust_entry_paths --> src_020_cluster_010_rs__resolve_source_root
    src_000_cluster_001_rs__rust_entry_paths --> src_070_layer_utilities_rs__resolve_source_root
    src_000_cluster_001_rs__collect_roots_from_crate --> src_000_cluster_001_rs__collect_roots_from_crate
    src_000_cluster_001_rs__collect_roots_from_crate --> src_000_cluster_001_rs__collect_roots_from_crate
    src_000_cluster_001_rs__collect_roots_from_crate --> src_000_cluster_001_rs__collect_roots_from_crate
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_020_cluster_010_rs__build_module_root_map
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_000_cluster_001_rs__rust_entry_paths
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_000_cluster_001_rs__detect_layer
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_000_cluster_001_rs__collect_rust_dependencies
    src_000_cluster_001_rs__order_rust_files_by_dependency --> src_010_cluster_008_rs__build_result
    src_000_cluster_001_rs__julia_entry_paths --> src_020_cluster_010_rs__resolve_source_root
    src_000_cluster_001_rs__julia_entry_paths --> src_070_layer_utilities_rs__resolve_source_root
    src_000_cluster_001_rs__build_file_layers --> src_000_cluster_001_rs__detect_layer
    src_000_cluster_001_rs__gather_julia_files --> src_020_cluster_010_rs__resolve_source_root
    src_000_cluster_001_rs__gather_julia_files --> src_070_layer_utilities_rs__resolve_source_root
    src_000_cluster_001_rs__gather_julia_files --> src_070_layer_utilities_rs__allow_analysis_dir
    src_000_cluster_001_rs__build_entries --> src_050_cluster_006_rs__generate_canonical_name
    src_000_cluster_001_rs__analyze_file_ordering --> src_030_cluster_011_rs__build_module_map
    src_000_cluster_001_rs__analyze_file_ordering --> src_020_cluster_010_rs__build_dependency_map
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__build_file_layers
    src_000_cluster_001_rs__analyze_file_ordering --> src_050_cluster_006_rs__order_directories
    src_000_cluster_001_rs__analyze_file_ordering --> src_030_cluster_011_rs__build_file_dag
    src_000_cluster_001_rs__analyze_file_ordering --> src_010_cluster_008_rs__detect_layer_violations
    src_000_cluster_001_rs__analyze_file_ordering --> src_020_layer_inference_rs__detect_layer_violations
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__detect_cycles
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__layer_constrained_sort
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__topological_sort
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__ordered_by_name
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__ordered_by_name
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__build_entries
    src_000_cluster_001_rs__analyze_file_ordering --> src_000_cluster_001_rs__detect_violations
    src_000_cluster_001_rs__export_complete_program_dot --> src_010_cluster_008_rs__cyclomatic_complexity
    src_000_cluster_001_rs__export_complete_program_dot --> src_010_cluster_008_rs__node_style
    src_000_cluster_001_rs__test_detects_cycles --> src_000_cluster_001_rs__detects_cycles
    src_000_cluster_001_rs__test_generates_canonical_names_and_violations --> src_000_cluster_001_rs__generates_canonical_names_and_violations
    src_005_refactor_constraints_rs__test_from_invariant_layer_fixed --> src_005_refactor_constraints_rs__from_invariant
    src_005_refactor_constraints_rs__test_check_move_allowed_blocking --> src_005_refactor_constraints_rs__check_move_allowed
    src_005_refactor_constraints_rs__test_check_move_allowed_blocking --> src_083_action_validator_rs__check_move_allowed
    src_005_refactor_constraints_rs__test_check_move_allowed_non_blocking --> src_005_refactor_constraints_rs__check_move_allowed
    src_005_refactor_constraints_rs__test_check_move_allowed_non_blocking --> src_083_action_validator_rs__check_move_allowed
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__adjacency_from_edges
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__topo_sort
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__layer_rank_map
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__is_mmsb_main
    src_010_cluster_008_rs__build_result --> src_070_layer_utilities_rs__main
    src_010_cluster_008_rs__build_result --> src_190_main_rs__main
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__is_mmsb_main
    src_010_cluster_008_rs__build_result --> src_070_layer_utilities_rs__main
    src_010_cluster_008_rs__build_result --> src_190_main_rs__main
    src_010_cluster_008_rs__build_result --> src_010_cluster_008_rs__is_layer_violation
    src_010_cluster_008_rs__topo_sort --> src_010_cluster_008_rs__insert_sorted
    src_010_cluster_008_rs__is_layer_violation --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__is_layer_violation --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__is_layer_violation --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__is_layer_violation --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_dir_layers --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_dir_layers --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_dir_layers --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_dir_layers --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_path_components --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_path_components --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_path_components --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__compare_path_components --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__layer_adheres --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__layer_adheres --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__layer_adheres --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__layer_adheres --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__structural_layer_value --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__structural_layer_value --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violations --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violations --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violations --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violations --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violation --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violation --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violation --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__detect_layer_violation --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__cluster_target_path --> src_010_cluster_008_rs__is_core_module_path
    src_010_cluster_008_rs__cluster_target_path --> src_010_cluster_008_rs__layer_prefix_value
    src_010_cluster_008_rs__cluster_target_path --> src_050_cluster_006_rs__layer_prefix_value
    src_010_cluster_008_rs__collect_cluster_plans --> src_010_cluster_008_rs__parse_cluster_members
    src_010_cluster_008_rs__collect_cluster_plans --> src_010_cluster_008_rs__cluster_target_path
    src_020_cluster_010_rs__resolve_module --> src_020_cluster_010_rs__normalize_module_name
    src_020_cluster_010_rs__resolve_module --> src_030_cluster_011_rs__resolve_path
    src_020_cluster_010_rs__build_module_root_map --> src_020_cluster_010_rs__contains_tools
    src_020_cluster_010_rs__build_module_root_map --> src_020_cluster_010_rs__normalize_module_name
    src_020_cluster_010_rs__build_module_root_map --> src_000_cluster_001_rs__detect_layer
    src_020_cluster_010_rs__order_julia_files_by_dependency --> src_000_cluster_001_rs__julia_entry_paths
    src_020_cluster_010_rs__order_julia_files_by_dependency --> src_000_cluster_001_rs__detect_layer
    src_020_cluster_010_rs__order_julia_files_by_dependency --> src_000_cluster_001_rs__collect_julia_dependencies
    src_020_cluster_010_rs__order_julia_files_by_dependency --> src_000_cluster_001_rs__detect_layer
    src_020_cluster_010_rs__order_julia_files_by_dependency --> src_020_cluster_010_rs__resolve_module
    src_020_cluster_010_rs__order_julia_files_by_dependency --> src_010_cluster_008_rs__build_result
    src_020_cluster_010_rs__extract_rust_dependencies --> src_040_dependency_rs__collect_roots
    src_020_cluster_010_rs__extract_rust_dependencies --> src_020_cluster_010_rs__resolve_module
    src_020_cluster_010_rs__extract_rust_dependencies --> src_020_cluster_010_rs__resolve_module
    src_020_cluster_010_rs__extract_julia_dependencies --> src_020_cluster_010_rs__resolve_module
    src_020_cluster_010_rs__extract_julia_dependencies --> src_030_cluster_011_rs__resolve_path
    src_020_cluster_010_rs__extract_julia_dependencies --> src_020_cluster_010_rs__resolve_module_name
    src_020_cluster_010_rs__extract_julia_dependencies --> src_020_cluster_010_rs__resolve_module_name
    src_020_cluster_010_rs__extract_julia_dependencies --> src_020_cluster_010_rs__resolve_module_name
    src_020_cluster_010_rs__extract_julia_dependencies --> src_020_cluster_010_rs__resolve_module_name
    src_020_cluster_010_rs__resolve_module_name --> src_020_cluster_010_rs__resolve_module
    src_020_cluster_010_rs__build_dependency_map --> src_020_cluster_010_rs__extract_dependencies
    src_020_cluster_010_rs__extract_dependencies --> src_020_cluster_010_rs__extract_rust_dependencies
    src_020_cluster_010_rs__extract_dependencies --> src_020_cluster_010_rs__extract_julia_dependencies
    src_020_layer_inference_rs__test_layer_inference_simple_dag --> src_020_layer_inference_rs__infer_layers
    src_020_layer_inference_rs__test_layer_inference_diamond --> src_020_layer_inference_rs__infer_layers
    src_020_layer_inference_rs__test_detect_layer_violations_none --> src_020_layer_inference_rs__infer_layers
    src_020_layer_inference_rs__test_detect_layer_violations_none --> src_010_cluster_008_rs__detect_layer_violations
    src_020_layer_inference_rs__test_detect_layer_violations_none --> src_020_layer_inference_rs__detect_layer_violations
    src_030_cluster_011_rs__build_module_map --> src_020_cluster_010_rs__normalize_module_name
    src_030_cluster_011_rs__build_module_map --> src_020_cluster_010_rs__normalize_module_name
    src_030_cluster_011_rs__resolve_path --> src_020_cluster_010_rs__normalize_module_name
    src_030_cluster_011_rs__build_directory_dag --> src_030_cluster_011_rs__build_module_map
    src_030_cluster_011_rs__build_directory_dag --> src_020_cluster_010_rs__build_dependency_map
    src_030_cluster_011_rs__build_directory_dag --> src_030_cluster_011_rs__build_file_dag
    src_030_cluster_011_rs__build_file_dependency_graph --> src_030_cluster_011_rs__build_module_map
    src_030_cluster_011_rs__build_file_dependency_graph --> src_020_cluster_010_rs__build_dependency_map
    src_030_cluster_011_rs__build_file_dependency_graph --> src_030_cluster_011_rs__build_file_dag
    src_030_cluster_011_rs__export_program_cfg_to_path --> src_000_cluster_001_rs__export_complete_program_dot
    src_030_fixpoint_solver_rs__test_fixpoint_simple --> src_030_fixpoint_solver_rs__propagate_to_fixpoint
    src_030_fixpoint_solver_rs__test_fixpoint_convergence --> src_030_fixpoint_solver_rs__propagate_to_fixpoint
    src_040_dependency_rs__collect_roots --> src_040_dependency_rs__collect_roots
    src_040_dependency_rs__collect_roots --> src_040_dependency_rs__collect_roots
    src_050_cluster_006_rs__order_directories --> src_050_cluster_006_rs__common_root
    src_050_cluster_006_rs__order_directories --> src_010_cluster_008_rs__compare_path_components
    src_050_cluster_006_rs__generate_canonical_name --> src_050_cluster_006_rs__strip_numeric_prefix
    src_050_cluster_006_rs__collect_directory_moves --> src_010_cluster_008_rs__compare_dir_layers
    src_050_cluster_006_rs__collect_directory_moves --> src_050_cluster_006_rs__strip_numeric_prefix
    src_050_cluster_006_rs__compute_cohesion_score --> src_010_cluster_008_rs__layer_adheres
    src_060_layer_core_rs__structural_cmp --> src_010_cluster_008_rs__structural_layer_value
    src_060_layer_core_rs__structural_cmp --> src_010_cluster_008_rs__structural_layer_value
    src_060_layer_core_rs__structural_cmp --> src_010_cluster_008_rs__structural_layer_value
    src_060_layer_core_rs__structural_cmp --> src_010_cluster_008_rs__structural_layer_value
    src_060_layer_core_rs__sort_structural_items --> src_010_cluster_008_rs__structural_layer_value
    src_060_layer_core_rs__sort_structural_items --> src_010_cluster_008_rs__structural_layer_value
    src_060_layer_core_rs__sort_structural_items --> src_060_layer_core_rs__structural_cmp
    src_070_invariant_integrator_rs__test_invariant_detector_creation --> src_070_invariant_integrator_rs__make_simple_analysis
    src_070_invariant_integrator_rs__test_detect_all --> src_070_invariant_integrator_rs__make_simple_analysis
    src_070_layer_utilities_rs__gather_rust_files --> src_020_cluster_010_rs__resolve_source_root
    src_070_layer_utilities_rs__gather_rust_files --> src_070_layer_utilities_rs__resolve_source_root
    src_070_layer_utilities_rs__gather_rust_files --> src_070_layer_utilities_rs__allow_analysis_dir
    src_070_layer_utilities_rs__main --> src_070_layer_utilities_rs__run_analysis
    src_070_layer_utilities_rs__run_analysis --> src_070_layer_utilities_rs__gather_rust_files
    src_070_layer_utilities_rs__run_analysis --> src_000_cluster_001_rs__order_rust_files_by_dependency
    src_070_layer_utilities_rs__run_analysis --> src_000_cluster_001_rs__analyze_file_ordering
    src_070_layer_utilities_rs__run_analysis --> src_000_cluster_001_rs__gather_julia_files
    src_070_layer_utilities_rs__run_analysis --> src_020_cluster_010_rs__order_julia_files_by_dependency
    src_070_layer_utilities_rs__run_analysis --> src_005_refactor_constraints_rs__generate_constraints
    src_070_layer_utilities_rs__run_analysis --> src_030_cluster_011_rs__export_program_cfg_to_path
    src_070_layer_utilities_rs__run_analysis --> src_080_invariant_reporter_rs__generate_invariant_report
    src_070_layer_utilities_rs__run_analysis --> src_080_invariant_reporter_rs__export_constraints_json
    src_080_invariant_reporter_rs__generate_invariant_report --> src_080_invariant_reporter_rs__export_json
    src_080_invariant_reporter_rs__generate_invariant_report --> src_082_conscience_graph_rs__generate_conscience_map
    src_080_invariant_reporter_rs__test_generate_report --> src_000_cluster_001_rs__temp_dir
    src_080_invariant_reporter_rs__test_generate_report --> src_080_invariant_reporter_rs__generate_invariant_report
    src_082_conscience_graph_rs__test_generate_stats --> src_082_conscience_graph_rs__generate_conscience_stats
    src_082_conscience_graph_rs__test_strength_emoji --> src_082_conscience_graph_rs__make_test_invariant
    src_082_conscience_graph_rs__test_strength_emoji --> src_085_agent_conscience_rs__make_test_invariant
    src_082_conscience_graph_rs__test_strength_emoji --> src_082_conscience_graph_rs__make_test_invariant
    src_082_conscience_graph_rs__test_strength_emoji --> src_085_agent_conscience_rs__make_test_invariant
    src_082_conscience_graph_rs__test_strength_emoji --> src_082_conscience_graph_rs__make_test_invariant
    src_082_conscience_graph_rs__test_strength_emoji --> src_085_agent_conscience_rs__make_test_invariant
    src_083_action_validator_rs__validate_action --> src_083_action_validator_rs__extract_layer
    src_083_action_validator_rs__validate_action --> src_083_action_validator_rs__extract_layer
    src_083_action_validator_rs__check_move_allowed --> src_083_action_validator_rs__validate_action
    src_083_action_validator_rs__test_validate_no_move_constraint --> src_083_action_validator_rs__validate_action
    src_083_action_validator_rs__test_validate_layer_fixed_constraint --> src_083_action_validator_rs__validate_action
    src_083_action_validator_rs__test_validate_preserve_signature --> src_083_action_validator_rs__validate_action
    src_083_action_validator_rs__test_validate_allowed_action --> src_083_action_validator_rs__validate_action
    src_083_action_validator_rs__test_check_move_allowed --> src_005_refactor_constraints_rs__check_move_allowed
    src_083_action_validator_rs__test_check_move_allowed --> src_083_action_validator_rs__check_move_allowed
    src_085_agent_conscience_rs__test_conscience_blocks_invalid_move --> src_082_conscience_graph_rs__make_test_invariant
    src_085_agent_conscience_rs__test_conscience_blocks_invalid_move --> src_085_agent_conscience_rs__make_test_invariant
    src_085_agent_conscience_rs__test_conscience_blocks_invalid_move --> src_191_agent_cli_rs__check_action
    src_085_agent_conscience_rs__test_conscience_allows_valid_action --> src_082_conscience_graph_rs__make_test_invariant
    src_085_agent_conscience_rs__test_conscience_allows_valid_action --> src_085_agent_conscience_rs__make_test_invariant
    src_085_agent_conscience_rs__test_conscience_allows_valid_action --> src_191_agent_cli_rs__check_action
    src_085_agent_conscience_rs__test_query_allowed_actions --> src_082_conscience_graph_rs__make_test_invariant
    src_085_agent_conscience_rs__test_query_allowed_actions --> src_085_agent_conscience_rs__make_test_invariant
    src_090_utilities_rs__collect_directory_files --> src_090_utilities_rs__collect_directory_files
    src_090_utilities_rs__resolve_required_layer_path --> src_090_utilities_rs__collect_directory_files
    src_090_utilities_rs__resolve_required_layer_path --> src_090_utilities_rs__path_common_prefix_len
    src_090_utilities_rs__collect_move_items --> src_090_utilities_rs__compute_move_metrics
    src_090_utilities_rs__collect_move_items --> src_090_utilities_rs__compress_path
    src_090_utilities_rs__collect_move_items --> src_090_utilities_rs__resolve_required_layer_path
    src_090_utilities_rs__collect_move_items --> src_090_utilities_rs__compress_path
    src_090_utilities_rs__collect_move_items --> src_090_utilities_rs__compute_move_metrics
    src_090_utilities_rs__write_structural_batches --> src_090_utilities_rs__compress_path
    src_090_utilities_rs__write_structural_batches --> src_090_utilities_rs__compress_path
    src_090_utilities_rs__write_cluster_batches --> src_090_utilities_rs__compress_path
    src_110_cohesion_analyzer_rs__build_call_edges --> src_110_cohesion_analyzer_rs__collect_functions
    src_110_cohesion_analyzer_rs__build_call_edges --> src_110_cohesion_analyzer_rs__build_name_map
    src_110_cohesion_analyzer_rs__build_call_analysis --> src_110_cohesion_analyzer_rs__compute_type_coupling
    src_110_cohesion_analyzer_rs__compute_type_coupling --> src_110_cohesion_analyzer_rs__extract_identifiers
    src_110_cohesion_analyzer_rs__louvain_communities --> src_110_cohesion_analyzer_rs__build_undirected_graph
    src_140_file_ordering_rs__parallel_build_file_dag --> src_030_cluster_011_rs__build_directory_dag
    src_150_julia_parser_rs__extract_calls_from_lines --> src_150_julia_parser_rs__extract_calls_from_text
    src_150_julia_parser_rs__extract_calls_from_text --> src_150_julia_parser_rs__is_reserved
    src_160_rust_parser_rs__expr_snippet --> src_160_rust_parser_rs__truncate_label
    src_160_rust_parser_rs__pat_snippet --> src_160_rust_parser_rs__truncate_label
    src_180_report_rs__collect_cluster_items --> src_180_report_rs__cluster_priority
    src_180_report_rs__collect_symbol_references --> src_180_report_rs__parse_use_symbols
    src_180_report_rs__collect_symbol_references --> src_180_report_rs__scan_crate_paths
    src_180_report_rs__referenced_elsewhere --> src_180_report_rs__path_matches
    src_180_report_rs__is_dead_code_candidate --> src_180_report_rs__path_matches
    src_180_report_rs__filter_orphaned --> src_180_report_rs__collect_symbol_references
    src_180_report_rs__filter_orphaned --> src_180_report_rs__load_cargo_warnings
    src_180_report_rs__filter_orphaned --> src_070_layer_utilities_rs__main
    src_180_report_rs__filter_orphaned --> src_180_report_rs__is_entrypoint_main
    src_180_report_rs__filter_orphaned --> src_190_main_rs__main
    src_180_report_rs__filter_orphaned --> src_180_report_rs__is_public_function
    src_180_report_rs__filter_orphaned --> src_180_report_rs__referenced_elsewhere
    src_180_report_rs__filter_orphaned --> src_180_report_rs__is_dead_code_candidate
    src_180_report_rs__collect_size_warnings --> src_180_report_rs__collect_size_warnings
    src_180_report_rs__collect_directories --> src_180_report_rs__collect_directories
    src_190_main_rs__main --> src_191_agent_cli_rs__run_agent_cli
    src_190_main_rs__main --> src_070_layer_utilities_rs__main
    src_190_main_rs__main --> src_190_main_rs__main
    src_191_agent_cli_rs__run_agent_cli --> src_191_agent_cli_rs__check_action
    src_191_agent_cli_rs__run_agent_cli --> src_191_agent_cli_rs__query_function
    src_191_agent_cli_rs__run_agent_cli --> src_191_agent_cli_rs__list_invariants
    src_191_agent_cli_rs__run_agent_cli --> src_191_agent_cli_rs__show_stats
    src_191_agent_cli_rs__check_action --> src_191_agent_cli_rs__load_invariants
    src_191_agent_cli_rs__check_action --> src_191_agent_cli_rs__check_action
    src_191_agent_cli_rs__query_function --> src_191_agent_cli_rs__load_invariants
    src_191_agent_cli_rs__list_invariants --> src_191_agent_cli_rs__load_invariants
    src_191_agent_cli_rs__show_stats --> src_191_agent_cli_rs__load_invariants
    src_191_agent_cli_rs__test_load_invariants_empty --> src_000_cluster_001_rs__temp_dir
    src_191_agent_cli_rs__test_load_invariants_empty --> src_191_agent_cli_rs__load_invariants
```
