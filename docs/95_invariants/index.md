# Invariant Analysis Report

Generated: 2025-12-30 22:29:05

## Summary

- **Total Invariants**: 1208
- **Proven**: 997 (82.5%)
- **Empirical**: 190
- **Heuristic**: 21 (1.7%) ⚠️ LOW CONFIDENCE
- **Violations**: 0

### By Kind

- **Structural**: 1007
- **Semantic**: 201
- **Delta**: 0
- **Path-Intersection**: 0

## Proven Invariants (Mechanical Truth)

These invariants are mathematically proven from graph structure and should **always block refactorings**.

### display_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_symbolic_abstraction_merge

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Layer 0 assignment is proven from call graph

### temp_dir

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### check_move_allowed

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 2 assignment is proven from call graph

### collect_utility_candidates

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_scc_compression_cycle

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_scc_compressor.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_detect_pure_function_heuristic

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_conscience_blocks_invalid_move

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/085_agent_conscience.rs
- **Description**: Layer 2 assignment is proven from call graph

### path_matches

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_extract_layer

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 0 assignment is proven from call graph

### placement_status_notes

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### gather_rust_files

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/070_layer_utilities.rs
- **Description**: Layer 1 assignment is proven from call graph

### main

- **Type**: Structural: LayerFixed { layer: 3 }
- **File**: src/190_main.rs
- **Description**: Layer 3 assignment is proven from call graph

### detects_cycles

- **Type**: Structural: LayerFixed { layer: 3 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 3 assignment is proven from call graph

### build_function_layers

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_generate_report

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Layer 2 assignment is proven from call graph

### generate_invariant_report

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Layer 1 assignment is proven from call graph

### collect_size_warnings

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### parse_struct_name

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Layer 0 assignment is proven from call graph

### sanitize_mermaid_id

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### short_signature

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_file_dependency_graph

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/030_cluster_011.rs
- **Description**: Layer 1 assignment is proven from call graph

### allow_analysis_dir

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/070_layer_utilities.rs
- **Description**: Layer 0 assignment is proven from call graph

### detect_layer_violation

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 1 assignment is proven from call graph

### suggest_file

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### slugify_relative

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Layer 0 assignment is proven from call graph

### query_function

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_fixpoint_simple

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Layer 1 assignment is proven from call graph

### compute_move_metrics

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/090_utilities.rs
- **Description**: Layer 0 assignment is proven from call graph

### resolve_module

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_invariant_detector_creation

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/070_invariant_integrator.rs
- **Description**: Layer 1 assignment is proven from call graph

### is_public_function

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### parse_cluster_members

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_detect_degree_stable

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/040_structural_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_call_analysis

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 2 assignment is proven from call graph

### propagate_to_fixpoint

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_result

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 2 assignment is proven from call graph

### test_path_stats

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/060_path_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### layer_rank_map

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_detect_leaf_root

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/040_structural_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### slugify_file_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### paren_balance

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Layer 0 assignment is proven from call graph

### is_source_file

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/120_directory_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_entries

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### is_layer_violation

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 1 assignment is proven from call graph

### sanitize_identifier

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/130_control_flow.rs
- **Description**: Layer 0 assignment is proven from call graph

### sort_plan_items

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### generate_canonical_name

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Layer 1 assignment is proven from call graph

### collect_cluster_items

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/180_report.rs
- **Description**: Layer 1 assignment is proven from call graph

### cluster_priority

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### slugify_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### compute_cohesion_score

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/050_cluster_006.rs
- **Description**: Layer 0 assignment is proven from call graph

### collect_julia_dependencies

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### sanitize_mermaid_label

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### resolve_required_layer_path

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/090_utilities.rs
- **Description**: Layer 1 assignment is proven from call graph

### generate_constraints

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_conscience_allows_valid_action

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/085_agent_conscience.rs
- **Description**: Layer 2 assignment is proven from call graph

### export_json

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Layer 0 assignment is proven from call graph

### export_complete_program_dot

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_layer_inference_simple_dag

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Layer 1 assignment is proven from call graph

### detect_cycles

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### kind_name

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/082_conscience_graph.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_dependency_map

- **Type**: Structural: LayerFixed { layer: 5 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 5 assignment is proven from call graph

### validate_action

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 1 assignment is proven from call graph

### determine_status

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### infer_layers

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/020_layer_inference.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_detects_cycles

- **Type**: Structural: LayerFixed { layer: 4 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 4 assignment is proven from call graph

### strip_numeric_prefix

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/050_cluster_006.rs
- **Description**: Layer 0 assignment is proven from call graph

### directory_moves_to_plan

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### truncate_label

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/160_rust_parser.rs
- **Description**: Layer 0 assignment is proven from call graph

### analyze_file_ordering

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 2 assignment is proven from call graph

### test_is_blocking

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_invariant_types.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_scc_compression_mixed

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_scc_compressor.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_check_move_allowed_non_blocking

- **Type**: Structural: LayerFixed { layer: 3 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Layer 3 assignment is proven from call graph

### normalize_use_stmt

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### write_cluster_tips

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### layer_adheres

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 1 assignment is proven from call graph

### generates_canonical_names_and_violations

- **Type**: Structural: LayerFixed { layer: 3 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 3 assignment is proven from call graph

### order_julia_files_by_dependency

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 2 assignment is proven from call graph

### collect_roots

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/040_dependency.rs
- **Description**: Layer 0 assignment is proven from call graph

### structural_cmp

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/060_layer_core.rs
- **Description**: Layer 2 assignment is proven from call graph

### test_scc_compression_dag

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_scc_compressor.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_from_invariant_layer_fixed

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Layer 1 assignment is proven from call graph

### render_mermaid_graph

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### detect_violations

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_call_edges

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_query_allowed_actions

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/085_agent_conscience.rs
- **Description**: Layer 1 assignment is proven from call graph

### collect_move_items

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/090_utilities.rs
- **Description**: Layer 2 assignment is proven from call graph

### extract_rust_dependencies

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 2 assignment is proven from call graph

### cyclomatic_complexity

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_path_detector_simple

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/060_path_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### group_key_cmp

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### write_priority_section

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### collect_roots_from_crate

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### rust_entry_paths

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_load_invariants_empty

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Layer 1 assignment is proven from call graph

### detect_layer_violations

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Layer 1 assignment is proven from call graph

### topo_sort

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 1 assignment is proven from call graph

### strength_emoji

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/082_conscience_graph.rs
- **Description**: Layer 0 assignment is proven from call graph

### write_baseline_metrics

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_file_dag

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/030_cluster_011.rs
- **Description**: Layer 0 assignment is proven from call graph

### topological_sort

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### is_entrypoint_main

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### louvain_communities

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 1 assignment is proven from call graph

### collect_directory_files

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/090_utilities.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_path_detector_diamond

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/060_path_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### node_style

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 0 assignment is proven from call graph

### compress_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/090_utilities.rs
- **Description**: Layer 0 assignment is proven from call graph

### contains_tools

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 0 assignment is proven from call graph

### pat_snippet

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/160_rust_parser.rs
- **Description**: Layer 1 assignment is proven from call graph

### matches_function

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 0 assignment is proven from call graph

### detect_layer

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### is_reserved

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Layer 0 assignment is proven from call graph

### slugify_key

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_strength_emoji

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/082_conscience_graph.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_layer_inference_diamond

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_stats_calculation

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_invariant_types.rs
- **Description**: Layer 0 assignment is proven from call graph

### list_invariants

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Layer 1 assignment is proven from call graph

### language_label

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### collect_symbol_references

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/180_report.rs
- **Description**: Layer 1 assignment is proven from call graph

### path_common_prefix_len

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/090_utilities.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_file_layers

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 1 assignment is proven from call graph

### parse_module_name

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_extract_facts_from_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/060_path_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### compute_directory_cohesion

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### should_skip_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/120_directory_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_generate_stats

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/082_conscience_graph.rs
- **Description**: Layer 1 assignment is proven from call graph

### check_action

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Layer 1 assignment is proven from call graph

### cluster_target_path

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 1 assignment is proven from call graph

### is_dead_code_candidate

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/180_report.rs
- **Description**: Layer 1 assignment is proven from call graph

### normalize_module_name

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 0 assignment is proven from call graph

### run_analysis

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/070_layer_utilities.rs
- **Description**: Layer 2 assignment is proven from call graph

### compare_dir_layers

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 1 assignment is proven from call graph

### build_undirected_graph

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### resolve_julia_binary

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Layer 0 assignment is proven from call graph

### write_cluster_batches

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/090_utilities.rs
- **Description**: Layer 1 assignment is proven from call graph

### ordered_by_name

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### sort_structural_items

- **Type**: Structural: LayerFixed { layer: 3 }
- **File**: src/060_layer_core.rs
- **Description**: Layer 3 assignment is proven from call graph

### sort_cluster_items

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### extract_julia_dependencies

- **Type**: Structural: LayerFixed { layer: 3 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 3 assignment is proven from call graph

### test_detect_all

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/070_invariant_integrator.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_validate_allowed_action

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 2 assignment is proven from call graph

### resolve_module_name

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 2 assignment is proven from call graph

### gather_julia_files

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### collect_directories

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### collect_naming_warnings

- **Type**: Structural: LayerFixed { layer: 7 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 7 assignment is proven from call graph

### test_detect_idempotent_heuristic

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### collect_rename_items

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_module_root_map

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 1 assignment is proven from call graph

### build_type_maps

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### compute_cluster_cohesion

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### collect_cluster_plans

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 2 assignment is proven from call graph

### collect_functions

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### load_report_config

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_detect_type_stable

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_detect_layer_violations_none

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/020_layer_inference.rs
- **Description**: Layer 2 assignment is proven from call graph

### load_baseline_metrics

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_module_map

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/030_cluster_011.rs
- **Description**: Layer 0 assignment is proven from call graph

### generate_conscience_stats

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/082_conscience_graph.rs
- **Description**: Layer 0 assignment is proven from call graph

### resolve_source_root

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/070_layer_utilities.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_constraint_is_blocking

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Layer 0 assignment is proven from call graph

### export_program_cfg_to_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/030_cluster_011.rs
- **Description**: Layer 0 assignment is proven from call graph

### placement_status_label

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### layer_constrained_sort

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 1 assignment is proven from call graph

### expr_snippet

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/160_rust_parser.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_validate_preserve_signature

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 2 assignment is proven from call graph

### parallel_build_file_dag

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/140_file_ordering.rs
- **Description**: Layer 0 assignment is proven from call graph

### write_structural_batches

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/090_utilities.rs
- **Description**: Layer 1 assignment is proven from call graph

### compute_type_coupling

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_no_pure_for_mutable

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### resolve_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/030_cluster_011.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_conscience_stats

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/085_agent_conscience.rs
- **Description**: Layer 0 assignment is proven from call graph

### order_directories

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Layer 1 assignment is proven from call graph

### make_simple_analysis

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/070_invariant_integrator.rs
- **Description**: Layer 0 assignment is proven from call graph

### extract_layer

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 0 assignment is proven from call graph

### extract_calls_from_lines

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/150_julia_parser.rs
- **Description**: Layer 2 assignment is proven from call graph

### from_invariant

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Layer 0 assignment is proven from call graph

### structural_layer_value

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 1 assignment is proven from call graph

### collect_rust_dependencies

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### parse_use_symbols

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### baseline_deltas

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_name_map

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### prefix_key_from_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### extract_calls_from_text

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Layer 1 assignment is proven from call graph

### extract_identifiers

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### julia_entry_paths

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### generate_conscience_map

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/082_conscience_graph.rs
- **Description**: Layer 0 assignment is proven from call graph

### collect_directory_moves

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Layer 1 assignment is proven from call graph

### layer_prefix_value

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/050_cluster_006.rs
- **Description**: Layer 0 assignment is proven from call graph

### suggest_cluster_file

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Layer 0 assignment is proven from call graph

### referenced_elsewhere

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/180_report.rs
- **Description**: Layer 1 assignment is proven from call graph

### show_stats

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Layer 1 assignment is proven from call graph

### naming_score_for_file

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### escape_dot

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### visibility_label

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_validate_layer_fixed_constraint

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 2 assignment is proven from call graph

### load_invariants

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/191_agent_cli.rs
- **Description**: Layer 0 assignment is proven from call graph

### make_function

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### compare_path_components

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 1 assignment is proven from call graph

### load_cargo_warnings

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_all_structural_invariants_proven

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/040_structural_detector.rs
- **Description**: Layer 0 assignment is proven from call graph

### is_core_module_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_fixpoint_convergence

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_validate_no_move_constraint

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 2 assignment is proven from call graph

### insert_sorted

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 0 assignment is proven from call graph

### extract_dependencies

- **Type**: Structural: LayerFixed { layer: 4 }
- **File**: src/020_cluster_010.rs
- **Description**: Layer 4 assignment is proven from call graph

### filter_orphaned

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/180_report.rs
- **Description**: Layer 2 assignment is proven from call graph

### relativize_path

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/160_rust_parser.rs
- **Description**: Layer 0 assignment is proven from call graph

### parse_dead_code_warnings

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### function_bucket_label

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### common_root

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/050_cluster_006.rs
- **Description**: Layer 0 assignment is proven from call graph

### run_agent_cli

- **Type**: Structural: LayerFixed { layer: 2 }
- **File**: src/191_agent_cli.rs
- **Description**: Layer 2 assignment is proven from call graph

### order_rust_files_by_dependency

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 1 assignment is proven from call graph

### test_check_move_allowed

- **Type**: Structural: LayerFixed { layer: 3 }
- **File**: src/083_action_validator.rs
- **Description**: Layer 3 assignment is proven from call graph

### write_structural_tips

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_directory_entry_map

- **Type**: Structural: LayerFixed { layer: 6 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 6 assignment is proven from call graph

### make_test_invariant

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/085_agent_conscience.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_check_move_allowed_blocking

- **Type**: Structural: LayerFixed { layer: 3 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Layer 3 assignment is proven from call graph

### compute_ordering_correctness

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_generates_canonical_names_and_violations

- **Type**: Structural: LayerFixed { layer: 4 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 4 assignment is proven from call graph

### build_directory_dag

- **Type**: Structural: LayerFixed { layer: 1 }
- **File**: src/030_cluster_011.rs
- **Description**: Layer 1 assignment is proven from call graph

### is_mmsb_main

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 0 assignment is proven from call graph

### test_confidence_from_strength

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_invariant_types.rs
- **Description**: Layer 0 assignment is proven from call graph

### topo_sort_within

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 0 assignment is proven from call graph

### adjacency_from_edges

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Layer 0 assignment is proven from call graph

### topo_sort_orders_dependencies

- **Type**: Structural: LayerFixed { layer: 3 }
- **File**: src/000_cluster_001.rs
- **Description**: Layer 3 assignment is proven from call graph

### export_constraints_json

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Layer 0 assignment is proven from call graph

### scan_crate_paths

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/180_report.rs
- **Description**: Layer 0 assignment is proven from call graph

### find_julia_project_dir

- **Type**: Structural: LayerFixed { layer: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Layer 0 assignment is proven from call graph

### build_directory_entry_map

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 9 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=9 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 9

### collect_naming_warnings

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### temp_dir

- **Type**: Structural: DegreeStable { in_degree: 3, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=3, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 3
  - Out-degree: 0

### detects_cycles

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### generates_canonical_names_and_violations

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### topo_sort_orders_dependencies

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### layer_constrained_sort

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### topo_sort_within

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### detect_layer

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### rust_entry_paths

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### collect_rust_dependencies

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### collect_roots_from_crate

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### order_rust_files_by_dependency

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 3 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=0, out=3 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 3

### collect_julia_dependencies

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### julia_entry_paths

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### build_file_layers

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=2, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 1

### gather_julia_files

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### topological_sort

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### ordered_by_name

- **Type**: Structural: DegreeStable { in_degree: 4, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=4, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 4
  - Out-degree: 0

### build_entries

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### analyze_file_ordering

- **Type**: Structural: DegreeStable { in_degree: 3, out_degree: 7 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=3, out=7 (proven from graph)
- **Evidence**:
  - In-degree: 3
  - Out-degree: 7

### naming_score_for_file

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### detect_cycles

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### detect_violations

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### export_complete_program_dot

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### escape_dot

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_detects_cycles

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_generates_canonical_names_and_violations

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_confidence_from_strength

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/000_invariant_types.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_is_blocking

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/000_invariant_types.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_stats_calculation

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/000_invariant_types.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### from_invariant

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### check_move_allowed

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### generate_constraints

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### test_from_invariant_layer_fixed

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_check_move_allowed_blocking

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_check_move_allowed_non_blocking

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_constraint_is_blocking

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### build_result

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 6 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=0, out=6 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 6

### adjacency_from_edges

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### topo_sort

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### layer_rank_map

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### insert_sorted

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### is_mmsb_main

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### is_layer_violation

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### layer_prefix_value

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/050_cluster_006.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### compare_dir_layers

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### compare_path_components

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### layer_adheres

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### structural_layer_value

- **Type**: Structural: DegreeStable { in_degree: 6, out_degree: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=6, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 6
  - Out-degree: 1

### detect_layer_violations

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/020_layer_inference.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### detect_layer_violation

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### parse_cluster_members

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### is_core_module_path

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### cluster_target_path

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### collect_cluster_plans

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### node_style

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### cyclomatic_complexity

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/010_cluster_008.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_scc_compression_dag

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/010_scc_compressor.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_scc_compression_cycle

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/010_scc_compressor.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_scc_compression_mixed

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/010_scc_compressor.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### normalize_module_name

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### resolve_module

- **Type**: Structural: DegreeStable { in_degree: 5, out_degree: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=5, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 5
  - Out-degree: 1

### contains_tools

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### build_module_root_map

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### resolve_source_root

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/070_layer_utilities.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### order_julia_files_by_dependency

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### extract_rust_dependencies

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### extract_julia_dependencies

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 5 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=1, out=5 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 5

### resolve_module_name

- **Type**: Structural: DegreeStable { in_degree: 4, out_degree: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=4, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 4
  - Out-degree: 1

### build_dependency_map

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### extract_dependencies

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/020_cluster_010.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### infer_layers

- **Type**: Structural: DegreeStable { in_degree: 3, out_degree: 0 }
- **File**: src/020_layer_inference.rs
- **Description**: Degree: in=3, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 3
  - Out-degree: 0

### detect_layer_violations

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/020_layer_inference.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### test_layer_inference_simple_dag

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_layer_inference_diamond

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_detect_layer_violations_none

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/020_layer_inference.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### build_module_map

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/030_cluster_011.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### resolve_path

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/030_cluster_011.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### build_directory_dag

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/030_cluster_011.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### build_file_dependency_graph

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/030_cluster_011.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### export_program_cfg_to_path

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/030_cluster_011.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### build_file_dag

- **Type**: Structural: DegreeStable { in_degree: 3, out_degree: 0 }
- **File**: src/030_cluster_011.rs
- **Description**: Degree: in=3, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 3
  - Out-degree: 0

### propagate_to_fixpoint

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### test_symbolic_abstraction_merge

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_fixpoint_simple

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_fixpoint_convergence

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### collect_roots

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/040_dependency.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_detect_leaf_root

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/040_structural_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_detect_degree_stable

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/040_structural_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_all_structural_invariants_proven

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/040_structural_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### layer_prefix_value

- **Type**: Structural: DegreeStable { in_degree: 15, out_degree: 0 }
- **File**: src/050_cluster_006.rs
- **Description**: Degree: in=15, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 15
  - Out-degree: 0

### order_directories

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### common_root

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/050_cluster_006.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### strip_numeric_prefix

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/050_cluster_006.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### generate_canonical_name

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### collect_directory_moves

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### compute_cohesion_score

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/050_cluster_006.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### make_function

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_detect_type_stable

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_detect_pure_function_heuristic

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_detect_idempotent_heuristic

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_no_pure_for_mutable

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/050_semantic_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### structural_cmp

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 4 }
- **File**: src/060_layer_core.rs
- **Description**: Degree: in=1, out=4 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 4

### sort_structural_items

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 3 }
- **File**: src/060_layer_core.rs
- **Description**: Degree: in=0, out=3 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 3

### test_path_detector_simple

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/060_path_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_path_detector_diamond

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/060_path_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_extract_facts_from_path

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/060_path_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_path_stats

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/060_path_detector.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### make_simple_analysis

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/070_invariant_integrator.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### test_invariant_detector_creation

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/070_invariant_integrator.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_detect_all

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/070_invariant_integrator.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### resolve_source_root

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/070_layer_utilities.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### allow_analysis_dir

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/070_layer_utilities.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### gather_rust_files

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/070_layer_utilities.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### main

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/190_main.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### run_analysis

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 4 }
- **File**: src/070_layer_utilities.rs
- **Description**: Degree: in=1, out=4 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 4

### generate_invariant_report

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### export_json

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### export_constraints_json

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_generate_report

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### generate_conscience_map

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/082_conscience_graph.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### strength_emoji

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/082_conscience_graph.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### kind_name

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/082_conscience_graph.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### generate_conscience_stats

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/082_conscience_graph.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### make_test_invariant

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/085_agent_conscience.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_generate_stats

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/082_conscience_graph.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_strength_emoji

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 3 }
- **File**: src/082_conscience_graph.rs
- **Description**: Degree: in=0, out=3 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 3

### extract_layer

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### matches_function

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### validate_action

- **Type**: Structural: DegreeStable { in_degree: 5, out_degree: 2 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=5, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 5
  - Out-degree: 2

### check_move_allowed

- **Type**: Structural: DegreeStable { in_degree: 3, out_degree: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=3, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 3
  - Out-degree: 1

### test_extract_layer

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_validate_no_move_constraint

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_validate_layer_fixed_constraint

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_validate_preserve_signature

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_validate_allowed_action

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### test_check_move_allowed

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### make_test_invariant

- **Type**: Structural: DegreeStable { in_degree: 6, out_degree: 0 }
- **File**: src/085_agent_conscience.rs
- **Description**: Degree: in=6, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 6
  - Out-degree: 0

### test_conscience_blocks_invalid_move

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/085_agent_conscience.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### test_conscience_allows_valid_action

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/085_agent_conscience.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### test_conscience_stats

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/085_agent_conscience.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### test_query_allowed_actions

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/085_agent_conscience.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### compress_path

- **Type**: Structural: DegreeStable { in_degree: 5, out_degree: 0 }
- **File**: src/090_utilities.rs
- **Description**: Degree: in=5, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 5
  - Out-degree: 0

### collect_directory_files

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/090_utilities.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### path_common_prefix_len

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/090_utilities.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### resolve_required_layer_path

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/090_utilities.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### compute_move_metrics

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/090_utilities.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### collect_move_items

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 5 }
- **File**: src/090_utilities.rs
- **Description**: Degree: in=0, out=5 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 5

### write_structural_batches

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/090_utilities.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### write_cluster_batches

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/090_utilities.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### collect_functions

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### build_call_edges

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 2 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=0, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 2

### build_function_layers

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### build_type_maps

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### build_name_map

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### build_call_analysis

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### determine_status

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### suggest_file

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### compute_cluster_cohesion

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### suggest_cluster_file

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### compute_type_coupling

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### extract_identifiers

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### louvain_communities

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### build_undirected_graph

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### is_source_file

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/120_directory_analyzer.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### should_skip_path

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/120_directory_analyzer.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### sanitize_identifier

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/130_control_flow.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### parallel_build_file_dag

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/140_file_ordering.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### slugify_relative

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### resolve_julia_binary

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### find_julia_project_dir

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### parse_module_name

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### parse_struct_name

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### relativize_path

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/160_rust_parser.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### extract_calls_from_lines

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### extract_calls_from_text

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### is_reserved

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### paren_balance

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/150_julia_parser.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### relativize_path

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/160_rust_parser.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### expr_snippet

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/160_rust_parser.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### pat_snippet

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/160_rust_parser.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### truncate_label

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/160_rust_parser.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### display_path

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### placement_status_label

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### placement_status_notes

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### collect_rename_items

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### collect_utility_candidates

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### directory_moves_to_plan

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### write_priority_section

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### write_structural_tips

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### write_cluster_tips

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### sort_plan_items

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### sort_cluster_items

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### cluster_priority

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### collect_cluster_items

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### load_cargo_warnings

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### parse_dead_code_warnings

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### parse_use_symbols

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### scan_crate_paths

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### collect_symbol_references

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 2 }
- **File**: src/180_report.rs
- **Description**: Degree: in=1, out=2 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 2

### is_public_function

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### path_matches

- **Type**: Structural: DegreeStable { in_degree: 2, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=2, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 2
  - Out-degree: 0

### is_entrypoint_main

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=1, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 0

### referenced_elsewhere

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/180_report.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### is_dead_code_candidate

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/180_report.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### filter_orphaned

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 6 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=6 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 6

### load_report_config

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### collect_size_warnings

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### load_baseline_metrics

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### baseline_deltas

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### write_baseline_metrics

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### collect_directories

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### slugify_path

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### render_mermaid_graph

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### compute_ordering_correctness

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### compute_directory_cohesion

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### prefix_key_from_path

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### slugify_key

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### group_key_cmp

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### function_bucket_label

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### slugify_file_path

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### language_label

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### visibility_label

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### short_signature

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### normalize_use_stmt

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### sanitize_mermaid_id

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### sanitize_mermaid_label

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 0 }
- **File**: src/180_report.rs
- **Description**: Degree: in=0, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 0

### main

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/190_main.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### run_agent_cli

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 4 }
- **File**: src/191_agent_cli.rs
- **Description**: Degree: in=0, out=4 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 4

### check_action

- **Type**: Structural: DegreeStable { in_degree: 3, out_degree: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Degree: in=3, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 3
  - Out-degree: 1

### query_function

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### list_invariants

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### show_stats

- **Type**: Structural: DegreeStable { in_degree: 1, out_degree: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Degree: in=1, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 1
  - Out-degree: 1

### load_invariants

- **Type**: Structural: DegreeStable { in_degree: 5, out_degree: 0 }
- **File**: src/191_agent_cli.rs
- **Description**: Degree: in=5, out=0 (proven from graph)
- **Evidence**:
  - In-degree: 5
  - Out-degree: 0

### test_load_invariants_empty

- **Type**: Structural: DegreeStable { in_degree: 0, out_degree: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Degree: in=0, out=1 (proven from graph)
- **Evidence**:
  - In-degree: 0
  - Out-degree: 1

### collect_naming_warnings

- **Type**: Structural: Root
- **File**: src/000_cluster_001.rs
- **Description**: Root node (called by no other functions)

### temp_dir

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### topo_sort_orders_dependencies

- **Type**: Structural: Root
- **File**: src/000_cluster_001.rs
- **Description**: Root node (called by no other functions)

### topo_sort_within

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### detect_layer

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### rust_entry_paths

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### collect_rust_dependencies

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### collect_roots_from_crate

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### collect_roots_from_crate

- **Type**: Structural: Root
- **File**: src/000_cluster_001.rs
- **Description**: Root node (called by no other functions)

### order_rust_files_by_dependency

- **Type**: Structural: Root
- **File**: src/000_cluster_001.rs
- **Description**: Root node (called by no other functions)

### collect_julia_dependencies

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### julia_entry_paths

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### julia_entry_paths

- **Type**: Structural: Root
- **File**: src/000_cluster_001.rs
- **Description**: Root node (called by no other functions)

### gather_julia_files

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### topological_sort

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### ordered_by_name

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### build_entries

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### naming_score_for_file

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### detect_cycles

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### detect_violations

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### export_complete_program_dot

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### export_complete_program_dot

- **Type**: Structural: Root
- **File**: src/000_cluster_001.rs
- **Description**: Root node (called by no other functions)

### escape_dot

- **Type**: Structural: Leaf
- **File**: src/000_cluster_001.rs
- **Description**: Leaf node (calls no other functions)

### escape_dot

- **Type**: Structural: Root
- **File**: src/000_cluster_001.rs
- **Description**: Root node (called by no other functions)

### test_detects_cycles

- **Type**: Structural: Root
- **File**: src/000_cluster_001.rs
- **Description**: Root node (called by no other functions)

### test_generates_canonical_names_and_violations

- **Type**: Structural: Root
- **File**: src/000_cluster_001.rs
- **Description**: Root node (called by no other functions)

### test_confidence_from_strength

- **Type**: Structural: Leaf
- **File**: src/000_invariant_types.rs
- **Description**: Leaf node (calls no other functions)

### test_confidence_from_strength

- **Type**: Structural: Root
- **File**: src/000_invariant_types.rs
- **Description**: Root node (called by no other functions)

### test_is_blocking

- **Type**: Structural: Leaf
- **File**: src/000_invariant_types.rs
- **Description**: Leaf node (calls no other functions)

### test_is_blocking

- **Type**: Structural: Root
- **File**: src/000_invariant_types.rs
- **Description**: Root node (called by no other functions)

### test_stats_calculation

- **Type**: Structural: Leaf
- **File**: src/000_invariant_types.rs
- **Description**: Leaf node (calls no other functions)

### test_stats_calculation

- **Type**: Structural: Root
- **File**: src/000_invariant_types.rs
- **Description**: Root node (called by no other functions)

### from_invariant

- **Type**: Structural: Leaf
- **File**: src/005_refactor_constraints.rs
- **Description**: Leaf node (calls no other functions)

### check_move_allowed

- **Type**: Structural: Leaf
- **File**: src/083_action_validator.rs
- **Description**: Leaf node (calls no other functions)

### check_move_allowed

- **Type**: Structural: Root
- **File**: src/083_action_validator.rs
- **Description**: Root node (called by no other functions)

### generate_constraints

- **Type**: Structural: Leaf
- **File**: src/005_refactor_constraints.rs
- **Description**: Leaf node (calls no other functions)

### test_from_invariant_layer_fixed

- **Type**: Structural: Root
- **File**: src/005_refactor_constraints.rs
- **Description**: Root node (called by no other functions)

### test_check_move_allowed_blocking

- **Type**: Structural: Root
- **File**: src/005_refactor_constraints.rs
- **Description**: Root node (called by no other functions)

### test_check_move_allowed_non_blocking

- **Type**: Structural: Root
- **File**: src/005_refactor_constraints.rs
- **Description**: Root node (called by no other functions)

### test_constraint_is_blocking

- **Type**: Structural: Leaf
- **File**: src/005_refactor_constraints.rs
- **Description**: Leaf node (calls no other functions)

### test_constraint_is_blocking

- **Type**: Structural: Root
- **File**: src/005_refactor_constraints.rs
- **Description**: Root node (called by no other functions)

### build_result

- **Type**: Structural: Root
- **File**: src/010_cluster_008.rs
- **Description**: Root node (called by no other functions)

### adjacency_from_edges

- **Type**: Structural: Leaf
- **File**: src/010_cluster_008.rs
- **Description**: Leaf node (calls no other functions)

### layer_rank_map

- **Type**: Structural: Leaf
- **File**: src/010_cluster_008.rs
- **Description**: Leaf node (calls no other functions)

### insert_sorted

- **Type**: Structural: Leaf
- **File**: src/010_cluster_008.rs
- **Description**: Leaf node (calls no other functions)

### is_mmsb_main

- **Type**: Structural: Leaf
- **File**: src/010_cluster_008.rs
- **Description**: Leaf node (calls no other functions)

### layer_prefix_value

- **Type**: Structural: Leaf
- **File**: src/050_cluster_006.rs
- **Description**: Leaf node (calls no other functions)

### layer_prefix_value

- **Type**: Structural: Root
- **File**: src/050_cluster_006.rs
- **Description**: Root node (called by no other functions)

### compare_dir_layers

- **Type**: Structural: Root
- **File**: src/010_cluster_008.rs
- **Description**: Root node (called by no other functions)

### compare_path_components

- **Type**: Structural: Root
- **File**: src/010_cluster_008.rs
- **Description**: Root node (called by no other functions)

### layer_adheres

- **Type**: Structural: Root
- **File**: src/010_cluster_008.rs
- **Description**: Root node (called by no other functions)

### detect_layer_violations

- **Type**: Structural: Leaf
- **File**: src/020_layer_inference.rs
- **Description**: Leaf node (calls no other functions)

### detect_layer_violations

- **Type**: Structural: Root
- **File**: src/020_layer_inference.rs
- **Description**: Root node (called by no other functions)

### detect_layer_violation

- **Type**: Structural: Root
- **File**: src/010_cluster_008.rs
- **Description**: Root node (called by no other functions)

### parse_cluster_members

- **Type**: Structural: Leaf
- **File**: src/010_cluster_008.rs
- **Description**: Leaf node (calls no other functions)

### is_core_module_path

- **Type**: Structural: Leaf
- **File**: src/010_cluster_008.rs
- **Description**: Leaf node (calls no other functions)

### collect_cluster_plans

- **Type**: Structural: Root
- **File**: src/010_cluster_008.rs
- **Description**: Root node (called by no other functions)

### node_style

- **Type**: Structural: Leaf
- **File**: src/010_cluster_008.rs
- **Description**: Leaf node (calls no other functions)

### node_style

- **Type**: Structural: Root
- **File**: src/010_cluster_008.rs
- **Description**: Root node (called by no other functions)

### cyclomatic_complexity

- **Type**: Structural: Leaf
- **File**: src/010_cluster_008.rs
- **Description**: Leaf node (calls no other functions)

### cyclomatic_complexity

- **Type**: Structural: Root
- **File**: src/010_cluster_008.rs
- **Description**: Root node (called by no other functions)

### test_scc_compression_dag

- **Type**: Structural: Leaf
- **File**: src/010_scc_compressor.rs
- **Description**: Leaf node (calls no other functions)

### test_scc_compression_dag

- **Type**: Structural: Root
- **File**: src/010_scc_compressor.rs
- **Description**: Root node (called by no other functions)

### test_scc_compression_cycle

- **Type**: Structural: Leaf
- **File**: src/010_scc_compressor.rs
- **Description**: Leaf node (calls no other functions)

### test_scc_compression_cycle

- **Type**: Structural: Root
- **File**: src/010_scc_compressor.rs
- **Description**: Root node (called by no other functions)

### test_scc_compression_mixed

- **Type**: Structural: Leaf
- **File**: src/010_scc_compressor.rs
- **Description**: Leaf node (calls no other functions)

### test_scc_compression_mixed

- **Type**: Structural: Root
- **File**: src/010_scc_compressor.rs
- **Description**: Root node (called by no other functions)

### normalize_module_name

- **Type**: Structural: Leaf
- **File**: src/020_cluster_010.rs
- **Description**: Leaf node (calls no other functions)

### contains_tools

- **Type**: Structural: Leaf
- **File**: src/020_cluster_010.rs
- **Description**: Leaf node (calls no other functions)

### build_module_root_map

- **Type**: Structural: Root
- **File**: src/020_cluster_010.rs
- **Description**: Root node (called by no other functions)

### resolve_source_root

- **Type**: Structural: Leaf
- **File**: src/070_layer_utilities.rs
- **Description**: Leaf node (calls no other functions)

### resolve_source_root

- **Type**: Structural: Root
- **File**: src/070_layer_utilities.rs
- **Description**: Root node (called by no other functions)

### order_julia_files_by_dependency

- **Type**: Structural: Root
- **File**: src/020_cluster_010.rs
- **Description**: Root node (called by no other functions)

### infer_layers

- **Type**: Structural: Leaf
- **File**: src/020_layer_inference.rs
- **Description**: Leaf node (calls no other functions)

### test_layer_inference_simple_dag

- **Type**: Structural: Root
- **File**: src/020_layer_inference.rs
- **Description**: Root node (called by no other functions)

### test_layer_inference_diamond

- **Type**: Structural: Root
- **File**: src/020_layer_inference.rs
- **Description**: Root node (called by no other functions)

### test_detect_layer_violations_none

- **Type**: Structural: Root
- **File**: src/020_layer_inference.rs
- **Description**: Root node (called by no other functions)

### build_module_map

- **Type**: Structural: Leaf
- **File**: src/030_cluster_011.rs
- **Description**: Leaf node (calls no other functions)

### resolve_path

- **Type**: Structural: Leaf
- **File**: src/030_cluster_011.rs
- **Description**: Leaf node (calls no other functions)

### resolve_path

- **Type**: Structural: Root
- **File**: src/030_cluster_011.rs
- **Description**: Root node (called by no other functions)

### build_directory_dag

- **Type**: Structural: Root
- **File**: src/030_cluster_011.rs
- **Description**: Root node (called by no other functions)

### build_file_dependency_graph

- **Type**: Structural: Root
- **File**: src/030_cluster_011.rs
- **Description**: Root node (called by no other functions)

### export_program_cfg_to_path

- **Type**: Structural: Leaf
- **File**: src/030_cluster_011.rs
- **Description**: Leaf node (calls no other functions)

### build_file_dag

- **Type**: Structural: Leaf
- **File**: src/030_cluster_011.rs
- **Description**: Leaf node (calls no other functions)

### propagate_to_fixpoint

- **Type**: Structural: Leaf
- **File**: src/030_fixpoint_solver.rs
- **Description**: Leaf node (calls no other functions)

### test_symbolic_abstraction_merge

- **Type**: Structural: Leaf
- **File**: src/030_fixpoint_solver.rs
- **Description**: Leaf node (calls no other functions)

### test_symbolic_abstraction_merge

- **Type**: Structural: Root
- **File**: src/030_fixpoint_solver.rs
- **Description**: Root node (called by no other functions)

### test_fixpoint_simple

- **Type**: Structural: Root
- **File**: src/030_fixpoint_solver.rs
- **Description**: Root node (called by no other functions)

### test_fixpoint_convergence

- **Type**: Structural: Root
- **File**: src/030_fixpoint_solver.rs
- **Description**: Root node (called by no other functions)

### collect_roots

- **Type**: Structural: Leaf
- **File**: src/040_dependency.rs
- **Description**: Leaf node (calls no other functions)

### collect_roots

- **Type**: Structural: Root
- **File**: src/040_dependency.rs
- **Description**: Root node (called by no other functions)

### test_detect_leaf_root

- **Type**: Structural: Leaf
- **File**: src/040_structural_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_detect_leaf_root

- **Type**: Structural: Root
- **File**: src/040_structural_detector.rs
- **Description**: Root node (called by no other functions)

### test_detect_degree_stable

- **Type**: Structural: Leaf
- **File**: src/040_structural_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_detect_degree_stable

- **Type**: Structural: Root
- **File**: src/040_structural_detector.rs
- **Description**: Root node (called by no other functions)

### test_all_structural_invariants_proven

- **Type**: Structural: Leaf
- **File**: src/040_structural_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_all_structural_invariants_proven

- **Type**: Structural: Root
- **File**: src/040_structural_detector.rs
- **Description**: Root node (called by no other functions)

### layer_prefix_value

- **Type**: Structural: Leaf
- **File**: src/050_cluster_006.rs
- **Description**: Leaf node (calls no other functions)

### order_directories

- **Type**: Structural: Root
- **File**: src/050_cluster_006.rs
- **Description**: Root node (called by no other functions)

### common_root

- **Type**: Structural: Leaf
- **File**: src/050_cluster_006.rs
- **Description**: Leaf node (calls no other functions)

### strip_numeric_prefix

- **Type**: Structural: Leaf
- **File**: src/050_cluster_006.rs
- **Description**: Leaf node (calls no other functions)

### generate_canonical_name

- **Type**: Structural: Root
- **File**: src/050_cluster_006.rs
- **Description**: Root node (called by no other functions)

### collect_directory_moves

- **Type**: Structural: Root
- **File**: src/050_cluster_006.rs
- **Description**: Root node (called by no other functions)

### compute_cohesion_score

- **Type**: Structural: Leaf
- **File**: src/050_cluster_006.rs
- **Description**: Leaf node (calls no other functions)

### compute_cohesion_score

- **Type**: Structural: Root
- **File**: src/050_cluster_006.rs
- **Description**: Root node (called by no other functions)

### make_function

- **Type**: Structural: Leaf
- **File**: src/050_semantic_detector.rs
- **Description**: Leaf node (calls no other functions)

### make_function

- **Type**: Structural: Root
- **File**: src/050_semantic_detector.rs
- **Description**: Root node (called by no other functions)

### test_detect_type_stable

- **Type**: Structural: Leaf
- **File**: src/050_semantic_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_detect_type_stable

- **Type**: Structural: Root
- **File**: src/050_semantic_detector.rs
- **Description**: Root node (called by no other functions)

### test_detect_pure_function_heuristic

- **Type**: Structural: Leaf
- **File**: src/050_semantic_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_detect_pure_function_heuristic

- **Type**: Structural: Root
- **File**: src/050_semantic_detector.rs
- **Description**: Root node (called by no other functions)

### test_detect_idempotent_heuristic

- **Type**: Structural: Leaf
- **File**: src/050_semantic_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_detect_idempotent_heuristic

- **Type**: Structural: Root
- **File**: src/050_semantic_detector.rs
- **Description**: Root node (called by no other functions)

### test_no_pure_for_mutable

- **Type**: Structural: Leaf
- **File**: src/050_semantic_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_no_pure_for_mutable

- **Type**: Structural: Root
- **File**: src/050_semantic_detector.rs
- **Description**: Root node (called by no other functions)

### sort_structural_items

- **Type**: Structural: Root
- **File**: src/060_layer_core.rs
- **Description**: Root node (called by no other functions)

### test_path_detector_simple

- **Type**: Structural: Leaf
- **File**: src/060_path_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_path_detector_simple

- **Type**: Structural: Root
- **File**: src/060_path_detector.rs
- **Description**: Root node (called by no other functions)

### test_path_detector_diamond

- **Type**: Structural: Leaf
- **File**: src/060_path_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_path_detector_diamond

- **Type**: Structural: Root
- **File**: src/060_path_detector.rs
- **Description**: Root node (called by no other functions)

### test_extract_facts_from_path

- **Type**: Structural: Leaf
- **File**: src/060_path_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_extract_facts_from_path

- **Type**: Structural: Root
- **File**: src/060_path_detector.rs
- **Description**: Root node (called by no other functions)

### test_path_stats

- **Type**: Structural: Leaf
- **File**: src/060_path_detector.rs
- **Description**: Leaf node (calls no other functions)

### test_path_stats

- **Type**: Structural: Root
- **File**: src/060_path_detector.rs
- **Description**: Root node (called by no other functions)

### make_simple_analysis

- **Type**: Structural: Leaf
- **File**: src/070_invariant_integrator.rs
- **Description**: Leaf node (calls no other functions)

### test_invariant_detector_creation

- **Type**: Structural: Root
- **File**: src/070_invariant_integrator.rs
- **Description**: Root node (called by no other functions)

### test_detect_all

- **Type**: Structural: Root
- **File**: src/070_invariant_integrator.rs
- **Description**: Root node (called by no other functions)

### resolve_source_root

- **Type**: Structural: Leaf
- **File**: src/070_layer_utilities.rs
- **Description**: Leaf node (calls no other functions)

### allow_analysis_dir

- **Type**: Structural: Leaf
- **File**: src/070_layer_utilities.rs
- **Description**: Leaf node (calls no other functions)

### main

- **Type**: Structural: Leaf
- **File**: src/190_main.rs
- **Description**: Leaf node (calls no other functions)

### main

- **Type**: Structural: Root
- **File**: src/190_main.rs
- **Description**: Root node (called by no other functions)

### export_json

- **Type**: Structural: Leaf
- **File**: src/080_invariant_reporter.rs
- **Description**: Leaf node (calls no other functions)

### export_constraints_json

- **Type**: Structural: Leaf
- **File**: src/080_invariant_reporter.rs
- **Description**: Leaf node (calls no other functions)

### export_constraints_json

- **Type**: Structural: Root
- **File**: src/080_invariant_reporter.rs
- **Description**: Root node (called by no other functions)

### test_generate_report

- **Type**: Structural: Root
- **File**: src/080_invariant_reporter.rs
- **Description**: Root node (called by no other functions)

### generate_conscience_map

- **Type**: Structural: Leaf
- **File**: src/082_conscience_graph.rs
- **Description**: Leaf node (calls no other functions)

### generate_conscience_map

- **Type**: Structural: Root
- **File**: src/082_conscience_graph.rs
- **Description**: Root node (called by no other functions)

### strength_emoji

- **Type**: Structural: Leaf
- **File**: src/082_conscience_graph.rs
- **Description**: Leaf node (calls no other functions)

### strength_emoji

- **Type**: Structural: Root
- **File**: src/082_conscience_graph.rs
- **Description**: Root node (called by no other functions)

### kind_name

- **Type**: Structural: Leaf
- **File**: src/082_conscience_graph.rs
- **Description**: Leaf node (calls no other functions)

### kind_name

- **Type**: Structural: Root
- **File**: src/082_conscience_graph.rs
- **Description**: Root node (called by no other functions)

### generate_conscience_stats

- **Type**: Structural: Leaf
- **File**: src/082_conscience_graph.rs
- **Description**: Leaf node (calls no other functions)

### make_test_invariant

- **Type**: Structural: Leaf
- **File**: src/085_agent_conscience.rs
- **Description**: Leaf node (calls no other functions)

### make_test_invariant

- **Type**: Structural: Root
- **File**: src/085_agent_conscience.rs
- **Description**: Root node (called by no other functions)

### test_generate_stats

- **Type**: Structural: Root
- **File**: src/082_conscience_graph.rs
- **Description**: Root node (called by no other functions)

### test_strength_emoji

- **Type**: Structural: Root
- **File**: src/082_conscience_graph.rs
- **Description**: Root node (called by no other functions)

### extract_layer

- **Type**: Structural: Leaf
- **File**: src/083_action_validator.rs
- **Description**: Leaf node (calls no other functions)

### matches_function

- **Type**: Structural: Leaf
- **File**: src/083_action_validator.rs
- **Description**: Leaf node (calls no other functions)

### matches_function

- **Type**: Structural: Root
- **File**: src/083_action_validator.rs
- **Description**: Root node (called by no other functions)

### test_extract_layer

- **Type**: Structural: Leaf
- **File**: src/083_action_validator.rs
- **Description**: Leaf node (calls no other functions)

### test_extract_layer

- **Type**: Structural: Root
- **File**: src/083_action_validator.rs
- **Description**: Root node (called by no other functions)

### test_validate_no_move_constraint

- **Type**: Structural: Root
- **File**: src/083_action_validator.rs
- **Description**: Root node (called by no other functions)

### test_validate_layer_fixed_constraint

- **Type**: Structural: Root
- **File**: src/083_action_validator.rs
- **Description**: Root node (called by no other functions)

### test_validate_preserve_signature

- **Type**: Structural: Root
- **File**: src/083_action_validator.rs
- **Description**: Root node (called by no other functions)

### test_validate_allowed_action

- **Type**: Structural: Root
- **File**: src/083_action_validator.rs
- **Description**: Root node (called by no other functions)

### test_check_move_allowed

- **Type**: Structural: Root
- **File**: src/083_action_validator.rs
- **Description**: Root node (called by no other functions)

### make_test_invariant

- **Type**: Structural: Leaf
- **File**: src/085_agent_conscience.rs
- **Description**: Leaf node (calls no other functions)

### test_conscience_blocks_invalid_move

- **Type**: Structural: Root
- **File**: src/085_agent_conscience.rs
- **Description**: Root node (called by no other functions)

### test_conscience_allows_valid_action

- **Type**: Structural: Root
- **File**: src/085_agent_conscience.rs
- **Description**: Root node (called by no other functions)

### test_conscience_stats

- **Type**: Structural: Leaf
- **File**: src/085_agent_conscience.rs
- **Description**: Leaf node (calls no other functions)

### test_conscience_stats

- **Type**: Structural: Root
- **File**: src/085_agent_conscience.rs
- **Description**: Root node (called by no other functions)

### test_query_allowed_actions

- **Type**: Structural: Root
- **File**: src/085_agent_conscience.rs
- **Description**: Root node (called by no other functions)

### compress_path

- **Type**: Structural: Leaf
- **File**: src/090_utilities.rs
- **Description**: Leaf node (calls no other functions)

### collect_directory_files

- **Type**: Structural: Leaf
- **File**: src/090_utilities.rs
- **Description**: Leaf node (calls no other functions)

### path_common_prefix_len

- **Type**: Structural: Leaf
- **File**: src/090_utilities.rs
- **Description**: Leaf node (calls no other functions)

### compute_move_metrics

- **Type**: Structural: Leaf
- **File**: src/090_utilities.rs
- **Description**: Leaf node (calls no other functions)

### collect_move_items

- **Type**: Structural: Root
- **File**: src/090_utilities.rs
- **Description**: Root node (called by no other functions)

### write_structural_batches

- **Type**: Structural: Root
- **File**: src/090_utilities.rs
- **Description**: Root node (called by no other functions)

### write_cluster_batches

- **Type**: Structural: Root
- **File**: src/090_utilities.rs
- **Description**: Root node (called by no other functions)

### collect_functions

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### build_call_edges

- **Type**: Structural: Root
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Root node (called by no other functions)

### build_function_layers

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### build_function_layers

- **Type**: Structural: Root
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Root node (called by no other functions)

### build_type_maps

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### build_type_maps

- **Type**: Structural: Root
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Root node (called by no other functions)

### build_name_map

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### build_call_analysis

- **Type**: Structural: Root
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Root node (called by no other functions)

### determine_status

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### determine_status

- **Type**: Structural: Root
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Root node (called by no other functions)

### suggest_file

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### suggest_file

- **Type**: Structural: Root
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Root node (called by no other functions)

### compute_cluster_cohesion

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### compute_cluster_cohesion

- **Type**: Structural: Root
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Root node (called by no other functions)

### suggest_cluster_file

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### suggest_cluster_file

- **Type**: Structural: Root
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Root node (called by no other functions)

### extract_identifiers

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### louvain_communities

- **Type**: Structural: Root
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Root node (called by no other functions)

### build_undirected_graph

- **Type**: Structural: Leaf
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### is_source_file

- **Type**: Structural: Leaf
- **File**: src/120_directory_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### is_source_file

- **Type**: Structural: Root
- **File**: src/120_directory_analyzer.rs
- **Description**: Root node (called by no other functions)

### should_skip_path

- **Type**: Structural: Leaf
- **File**: src/120_directory_analyzer.rs
- **Description**: Leaf node (calls no other functions)

### should_skip_path

- **Type**: Structural: Root
- **File**: src/120_directory_analyzer.rs
- **Description**: Root node (called by no other functions)

### sanitize_identifier

- **Type**: Structural: Leaf
- **File**: src/130_control_flow.rs
- **Description**: Leaf node (calls no other functions)

### sanitize_identifier

- **Type**: Structural: Root
- **File**: src/130_control_flow.rs
- **Description**: Root node (called by no other functions)

### parallel_build_file_dag

- **Type**: Structural: Leaf
- **File**: src/140_file_ordering.rs
- **Description**: Leaf node (calls no other functions)

### parallel_build_file_dag

- **Type**: Structural: Root
- **File**: src/140_file_ordering.rs
- **Description**: Root node (called by no other functions)

### slugify_relative

- **Type**: Structural: Leaf
- **File**: src/150_julia_parser.rs
- **Description**: Leaf node (calls no other functions)

### slugify_relative

- **Type**: Structural: Root
- **File**: src/150_julia_parser.rs
- **Description**: Root node (called by no other functions)

### resolve_julia_binary

- **Type**: Structural: Leaf
- **File**: src/150_julia_parser.rs
- **Description**: Leaf node (calls no other functions)

### resolve_julia_binary

- **Type**: Structural: Root
- **File**: src/150_julia_parser.rs
- **Description**: Root node (called by no other functions)

### find_julia_project_dir

- **Type**: Structural: Leaf
- **File**: src/150_julia_parser.rs
- **Description**: Leaf node (calls no other functions)

### find_julia_project_dir

- **Type**: Structural: Root
- **File**: src/150_julia_parser.rs
- **Description**: Root node (called by no other functions)

### parse_module_name

- **Type**: Structural: Leaf
- **File**: src/150_julia_parser.rs
- **Description**: Leaf node (calls no other functions)

### parse_module_name

- **Type**: Structural: Root
- **File**: src/150_julia_parser.rs
- **Description**: Root node (called by no other functions)

### parse_struct_name

- **Type**: Structural: Leaf
- **File**: src/150_julia_parser.rs
- **Description**: Leaf node (calls no other functions)

### parse_struct_name

- **Type**: Structural: Root
- **File**: src/150_julia_parser.rs
- **Description**: Root node (called by no other functions)

### relativize_path

- **Type**: Structural: Leaf
- **File**: src/160_rust_parser.rs
- **Description**: Leaf node (calls no other functions)

### relativize_path

- **Type**: Structural: Root
- **File**: src/160_rust_parser.rs
- **Description**: Root node (called by no other functions)

### extract_calls_from_lines

- **Type**: Structural: Root
- **File**: src/150_julia_parser.rs
- **Description**: Root node (called by no other functions)

### is_reserved

- **Type**: Structural: Leaf
- **File**: src/150_julia_parser.rs
- **Description**: Leaf node (calls no other functions)

### paren_balance

- **Type**: Structural: Leaf
- **File**: src/150_julia_parser.rs
- **Description**: Leaf node (calls no other functions)

### paren_balance

- **Type**: Structural: Root
- **File**: src/150_julia_parser.rs
- **Description**: Root node (called by no other functions)

### relativize_path

- **Type**: Structural: Leaf
- **File**: src/160_rust_parser.rs
- **Description**: Leaf node (calls no other functions)

### relativize_path

- **Type**: Structural: Root
- **File**: src/160_rust_parser.rs
- **Description**: Root node (called by no other functions)

### expr_snippet

- **Type**: Structural: Root
- **File**: src/160_rust_parser.rs
- **Description**: Root node (called by no other functions)

### pat_snippet

- **Type**: Structural: Root
- **File**: src/160_rust_parser.rs
- **Description**: Root node (called by no other functions)

### truncate_label

- **Type**: Structural: Leaf
- **File**: src/160_rust_parser.rs
- **Description**: Leaf node (calls no other functions)

### display_path

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### display_path

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### placement_status_label

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### placement_status_label

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### placement_status_notes

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### placement_status_notes

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### collect_rename_items

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### collect_rename_items

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### collect_utility_candidates

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### collect_utility_candidates

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### directory_moves_to_plan

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### directory_moves_to_plan

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### write_priority_section

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### write_priority_section

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### write_structural_tips

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### write_structural_tips

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### write_cluster_tips

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### write_cluster_tips

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### sort_plan_items

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### sort_plan_items

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### sort_cluster_items

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### sort_cluster_items

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### cluster_priority

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### collect_cluster_items

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### load_cargo_warnings

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### parse_dead_code_warnings

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### parse_dead_code_warnings

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### parse_use_symbols

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### scan_crate_paths

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### is_public_function

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### path_matches

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### is_entrypoint_main

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### filter_orphaned

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### load_report_config

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### load_report_config

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### collect_size_warnings

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### collect_size_warnings

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### load_baseline_metrics

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### load_baseline_metrics

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### baseline_deltas

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### baseline_deltas

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### write_baseline_metrics

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### write_baseline_metrics

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### collect_directories

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### collect_directories

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### slugify_path

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### slugify_path

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### render_mermaid_graph

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### render_mermaid_graph

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### compute_ordering_correctness

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### compute_ordering_correctness

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### compute_directory_cohesion

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### compute_directory_cohesion

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### prefix_key_from_path

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### prefix_key_from_path

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### slugify_key

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### slugify_key

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### group_key_cmp

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### group_key_cmp

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### function_bucket_label

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### function_bucket_label

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### slugify_file_path

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### slugify_file_path

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### language_label

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### language_label

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### visibility_label

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### visibility_label

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### short_signature

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### short_signature

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### normalize_use_stmt

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### normalize_use_stmt

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### sanitize_mermaid_id

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### sanitize_mermaid_id

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### sanitize_mermaid_label

- **Type**: Structural: Leaf
- **File**: src/180_report.rs
- **Description**: Leaf node (calls no other functions)

### sanitize_mermaid_label

- **Type**: Structural: Root
- **File**: src/180_report.rs
- **Description**: Root node (called by no other functions)

### main

- **Type**: Structural: Root
- **File**: src/190_main.rs
- **Description**: Root node (called by no other functions)

### run_agent_cli

- **Type**: Structural: Root
- **File**: src/191_agent_cli.rs
- **Description**: Root node (called by no other functions)

### load_invariants

- **Type**: Structural: Leaf
- **File**: src/191_agent_cli.rs
- **Description**: Leaf node (calls no other functions)

### test_load_invariants_empty

- **Type**: Structural: Root
- **File**: src/191_agent_cli.rs
- **Description**: Root node (called by no other functions)

### build_entries

- **Type**: Structural: SccMembership { scc_id: 0, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 0 (size: 1)

### ordered_by_name

- **Type**: Structural: SccMembership { scc_id: 1, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 1 (size: 1)

### topological_sort

- **Type**: Structural: SccMembership { scc_id: 2, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 2 (size: 1)

### topo_sort_within

- **Type**: Structural: SccMembership { scc_id: 3, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 3 (size: 1)

### layer_prefix_value

- **Type**: Structural: SccMembership { scc_id: 4, scc_size: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Member of SCC 4 (size: 1)

### layer_constrained_sort

- **Type**: Structural: SccMembership { scc_id: 5, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 5 (size: 1)

### detect_cycles

- **Type**: Structural: SccMembership { scc_id: 6, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 6 (size: 1)

### build_file_dag

- **Type**: Structural: SccMembership { scc_id: 7, scc_size: 1 }
- **File**: src/030_cluster_011.rs
- **Description**: Member of SCC 7 (size: 1)

### detect_layer

- **Type**: Structural: SccMembership { scc_id: 8, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 8 (size: 1)

### build_file_layers

- **Type**: Structural: SccMembership { scc_id: 9, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 9 (size: 1)

### normalize_module_name

- **Type**: Structural: SccMembership { scc_id: 10, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 10 (size: 1)

### resolve_module

- **Type**: Structural: SccMembership { scc_id: 11, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 11 (size: 1)

### resolve_module_name

- **Type**: Structural: SccMembership { scc_id: 12, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 12 (size: 1)

### extract_julia_dependencies

- **Type**: Structural: SccMembership { scc_id: 13, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 13 (size: 1)

### extract_rust_dependencies

- **Type**: Structural: SccMembership { scc_id: 14, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 14 (size: 1)

### extract_dependencies

- **Type**: Structural: SccMembership { scc_id: 15, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 15 (size: 1)

### build_dependency_map

- **Type**: Structural: SccMembership { scc_id: 16, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 16 (size: 1)

### build_directory_entry_map

- **Type**: Structural: SccMembership { scc_id: 17, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 17 (size: 1)

### naming_score_for_file

- **Type**: Structural: SccMembership { scc_id: 18, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 18 (size: 1)

### collect_naming_warnings

- **Type**: Structural: SccMembership { scc_id: 19, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 19 (size: 1)

### temp_dir

- **Type**: Structural: SccMembership { scc_id: 20, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 20 (size: 1)

### detect_violations

- **Type**: Structural: SccMembership { scc_id: 21, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 21 (size: 1)

### analyze_file_ordering

- **Type**: Structural: SccMembership { scc_id: 22, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 22 (size: 1)

### detects_cycles

- **Type**: Structural: SccMembership { scc_id: 23, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 23 (size: 1)

### generates_canonical_names_and_violations

- **Type**: Structural: SccMembership { scc_id: 24, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 24 (size: 1)

### topo_sort_orders_dependencies

- **Type**: Structural: SccMembership { scc_id: 25, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 25 (size: 1)

### rust_entry_paths

- **Type**: Structural: SccMembership { scc_id: 26, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 26 (size: 1)

### collect_rust_dependencies

- **Type**: Structural: SccMembership { scc_id: 27, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 27 (size: 1)

### collect_roots_from_crate

- **Type**: Structural: SccMembership { scc_id: 28, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 28 (size: 1)

### order_rust_files_by_dependency

- **Type**: Structural: SccMembership { scc_id: 29, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 29 (size: 1)

### collect_julia_dependencies

- **Type**: Structural: SccMembership { scc_id: 30, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 30 (size: 1)

### julia_entry_paths

- **Type**: Structural: SccMembership { scc_id: 31, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 31 (size: 1)

### gather_julia_files

- **Type**: Structural: SccMembership { scc_id: 32, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 32 (size: 1)

### export_complete_program_dot

- **Type**: Structural: SccMembership { scc_id: 33, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 33 (size: 1)

### escape_dot

- **Type**: Structural: SccMembership { scc_id: 34, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 34 (size: 1)

### test_detects_cycles

- **Type**: Structural: SccMembership { scc_id: 35, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 35 (size: 1)

### test_generates_canonical_names_and_violations

- **Type**: Structural: SccMembership { scc_id: 36, scc_size: 1 }
- **File**: src/000_cluster_001.rs
- **Description**: Member of SCC 36 (size: 1)

### test_confidence_from_strength

- **Type**: Structural: SccMembership { scc_id: 37, scc_size: 1 }
- **File**: src/000_invariant_types.rs
- **Description**: Member of SCC 37 (size: 1)

### test_is_blocking

- **Type**: Structural: SccMembership { scc_id: 38, scc_size: 1 }
- **File**: src/000_invariant_types.rs
- **Description**: Member of SCC 38 (size: 1)

### test_stats_calculation

- **Type**: Structural: SccMembership { scc_id: 39, scc_size: 1 }
- **File**: src/000_invariant_types.rs
- **Description**: Member of SCC 39 (size: 1)

### from_invariant

- **Type**: Structural: SccMembership { scc_id: 40, scc_size: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Member of SCC 40 (size: 1)

### check_move_allowed

- **Type**: Structural: SccMembership { scc_id: 41, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 41 (size: 1)

### generate_constraints

- **Type**: Structural: SccMembership { scc_id: 42, scc_size: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Member of SCC 42 (size: 1)

### test_from_invariant_layer_fixed

- **Type**: Structural: SccMembership { scc_id: 43, scc_size: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Member of SCC 43 (size: 1)

### extract_layer

- **Type**: Structural: SccMembership { scc_id: 44, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 44 (size: 1)

### validate_action

- **Type**: Structural: SccMembership { scc_id: 45, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 45 (size: 1)

### check_move_allowed

- **Type**: Structural: SccMembership { scc_id: 46, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 46 (size: 1)

### test_check_move_allowed_blocking

- **Type**: Structural: SccMembership { scc_id: 47, scc_size: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Member of SCC 47 (size: 1)

### test_check_move_allowed_non_blocking

- **Type**: Structural: SccMembership { scc_id: 48, scc_size: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Member of SCC 48 (size: 1)

### test_constraint_is_blocking

- **Type**: Structural: SccMembership { scc_id: 49, scc_size: 1 }
- **File**: src/005_refactor_constraints.rs
- **Description**: Member of SCC 49 (size: 1)

### is_layer_violation

- **Type**: Structural: SccMembership { scc_id: 50, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 50 (size: 1)

### is_mmsb_main

- **Type**: Structural: SccMembership { scc_id: 51, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 51 (size: 1)

### layer_rank_map

- **Type**: Structural: SccMembership { scc_id: 52, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 52 (size: 1)

### insert_sorted

- **Type**: Structural: SccMembership { scc_id: 53, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 53 (size: 1)

### topo_sort

- **Type**: Structural: SccMembership { scc_id: 54, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 54 (size: 1)

### adjacency_from_edges

- **Type**: Structural: SccMembership { scc_id: 55, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 55 (size: 1)

### build_result

- **Type**: Structural: SccMembership { scc_id: 56, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 56 (size: 1)

### layer_prefix_value

- **Type**: Structural: SccMembership { scc_id: 57, scc_size: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Member of SCC 57 (size: 1)

### compare_dir_layers

- **Type**: Structural: SccMembership { scc_id: 58, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 58 (size: 1)

### compare_path_components

- **Type**: Structural: SccMembership { scc_id: 59, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 59 (size: 1)

### layer_adheres

- **Type**: Structural: SccMembership { scc_id: 60, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 60 (size: 1)

### structural_layer_value

- **Type**: Structural: SccMembership { scc_id: 61, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 61 (size: 1)

### detect_layer_violations

- **Type**: Structural: SccMembership { scc_id: 62, scc_size: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Member of SCC 62 (size: 1)

### detect_layer_violation

- **Type**: Structural: SccMembership { scc_id: 63, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 63 (size: 1)

### parse_cluster_members

- **Type**: Structural: SccMembership { scc_id: 64, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 64 (size: 1)

### is_core_module_path

- **Type**: Structural: SccMembership { scc_id: 65, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 65 (size: 1)

### cluster_target_path

- **Type**: Structural: SccMembership { scc_id: 66, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 66 (size: 1)

### collect_cluster_plans

- **Type**: Structural: SccMembership { scc_id: 67, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 67 (size: 1)

### node_style

- **Type**: Structural: SccMembership { scc_id: 68, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 68 (size: 1)

### cyclomatic_complexity

- **Type**: Structural: SccMembership { scc_id: 69, scc_size: 1 }
- **File**: src/010_cluster_008.rs
- **Description**: Member of SCC 69 (size: 1)

### test_scc_compression_dag

- **Type**: Structural: SccMembership { scc_id: 70, scc_size: 1 }
- **File**: src/010_scc_compressor.rs
- **Description**: Member of SCC 70 (size: 1)

### test_scc_compression_cycle

- **Type**: Structural: SccMembership { scc_id: 71, scc_size: 1 }
- **File**: src/010_scc_compressor.rs
- **Description**: Member of SCC 71 (size: 1)

### test_scc_compression_mixed

- **Type**: Structural: SccMembership { scc_id: 72, scc_size: 1 }
- **File**: src/010_scc_compressor.rs
- **Description**: Member of SCC 72 (size: 1)

### contains_tools

- **Type**: Structural: SccMembership { scc_id: 73, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 73 (size: 1)

### build_module_root_map

- **Type**: Structural: SccMembership { scc_id: 74, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 74 (size: 1)

### resolve_source_root

- **Type**: Structural: SccMembership { scc_id: 75, scc_size: 1 }
- **File**: src/070_layer_utilities.rs
- **Description**: Member of SCC 75 (size: 1)

### order_julia_files_by_dependency

- **Type**: Structural: SccMembership { scc_id: 76, scc_size: 1 }
- **File**: src/020_cluster_010.rs
- **Description**: Member of SCC 76 (size: 1)

### infer_layers

- **Type**: Structural: SccMembership { scc_id: 77, scc_size: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Member of SCC 77 (size: 1)

### detect_layer_violations

- **Type**: Structural: SccMembership { scc_id: 78, scc_size: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Member of SCC 78 (size: 1)

### test_layer_inference_simple_dag

- **Type**: Structural: SccMembership { scc_id: 79, scc_size: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Member of SCC 79 (size: 1)

### test_layer_inference_diamond

- **Type**: Structural: SccMembership { scc_id: 80, scc_size: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Member of SCC 80 (size: 1)

### test_detect_layer_violations_none

- **Type**: Structural: SccMembership { scc_id: 81, scc_size: 1 }
- **File**: src/020_layer_inference.rs
- **Description**: Member of SCC 81 (size: 1)

### build_module_map

- **Type**: Structural: SccMembership { scc_id: 82, scc_size: 1 }
- **File**: src/030_cluster_011.rs
- **Description**: Member of SCC 82 (size: 1)

### resolve_path

- **Type**: Structural: SccMembership { scc_id: 83, scc_size: 1 }
- **File**: src/030_cluster_011.rs
- **Description**: Member of SCC 83 (size: 1)

### build_directory_dag

- **Type**: Structural: SccMembership { scc_id: 84, scc_size: 1 }
- **File**: src/030_cluster_011.rs
- **Description**: Member of SCC 84 (size: 1)

### build_file_dependency_graph

- **Type**: Structural: SccMembership { scc_id: 85, scc_size: 1 }
- **File**: src/030_cluster_011.rs
- **Description**: Member of SCC 85 (size: 1)

### export_program_cfg_to_path

- **Type**: Structural: SccMembership { scc_id: 86, scc_size: 1 }
- **File**: src/030_cluster_011.rs
- **Description**: Member of SCC 86 (size: 1)

### propagate_to_fixpoint

- **Type**: Structural: SccMembership { scc_id: 87, scc_size: 1 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Member of SCC 87 (size: 1)

### test_symbolic_abstraction_merge

- **Type**: Structural: SccMembership { scc_id: 88, scc_size: 1 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Member of SCC 88 (size: 1)

### test_fixpoint_simple

- **Type**: Structural: SccMembership { scc_id: 89, scc_size: 1 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Member of SCC 89 (size: 1)

### test_fixpoint_convergence

- **Type**: Structural: SccMembership { scc_id: 90, scc_size: 1 }
- **File**: src/030_fixpoint_solver.rs
- **Description**: Member of SCC 90 (size: 1)

### collect_roots

- **Type**: Structural: SccMembership { scc_id: 91, scc_size: 1 }
- **File**: src/040_dependency.rs
- **Description**: Member of SCC 91 (size: 1)

### test_detect_leaf_root

- **Type**: Structural: SccMembership { scc_id: 92, scc_size: 1 }
- **File**: src/040_structural_detector.rs
- **Description**: Member of SCC 92 (size: 1)

### test_detect_degree_stable

- **Type**: Structural: SccMembership { scc_id: 93, scc_size: 1 }
- **File**: src/040_structural_detector.rs
- **Description**: Member of SCC 93 (size: 1)

### test_all_structural_invariants_proven

- **Type**: Structural: SccMembership { scc_id: 94, scc_size: 1 }
- **File**: src/040_structural_detector.rs
- **Description**: Member of SCC 94 (size: 1)

### common_root

- **Type**: Structural: SccMembership { scc_id: 95, scc_size: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Member of SCC 95 (size: 1)

### order_directories

- **Type**: Structural: SccMembership { scc_id: 96, scc_size: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Member of SCC 96 (size: 1)

### strip_numeric_prefix

- **Type**: Structural: SccMembership { scc_id: 97, scc_size: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Member of SCC 97 (size: 1)

### generate_canonical_name

- **Type**: Structural: SccMembership { scc_id: 98, scc_size: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Member of SCC 98 (size: 1)

### collect_directory_moves

- **Type**: Structural: SccMembership { scc_id: 99, scc_size: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Member of SCC 99 (size: 1)

### compute_cohesion_score

- **Type**: Structural: SccMembership { scc_id: 100, scc_size: 1 }
- **File**: src/050_cluster_006.rs
- **Description**: Member of SCC 100 (size: 1)

### make_function

- **Type**: Structural: SccMembership { scc_id: 101, scc_size: 1 }
- **File**: src/050_semantic_detector.rs
- **Description**: Member of SCC 101 (size: 1)

### test_detect_type_stable

- **Type**: Structural: SccMembership { scc_id: 102, scc_size: 1 }
- **File**: src/050_semantic_detector.rs
- **Description**: Member of SCC 102 (size: 1)

### test_detect_pure_function_heuristic

- **Type**: Structural: SccMembership { scc_id: 103, scc_size: 1 }
- **File**: src/050_semantic_detector.rs
- **Description**: Member of SCC 103 (size: 1)

### test_detect_idempotent_heuristic

- **Type**: Structural: SccMembership { scc_id: 104, scc_size: 1 }
- **File**: src/050_semantic_detector.rs
- **Description**: Member of SCC 104 (size: 1)

### test_no_pure_for_mutable

- **Type**: Structural: SccMembership { scc_id: 105, scc_size: 1 }
- **File**: src/050_semantic_detector.rs
- **Description**: Member of SCC 105 (size: 1)

### structural_cmp

- **Type**: Structural: SccMembership { scc_id: 106, scc_size: 1 }
- **File**: src/060_layer_core.rs
- **Description**: Member of SCC 106 (size: 1)

### sort_structural_items

- **Type**: Structural: SccMembership { scc_id: 107, scc_size: 1 }
- **File**: src/060_layer_core.rs
- **Description**: Member of SCC 107 (size: 1)

### test_path_detector_simple

- **Type**: Structural: SccMembership { scc_id: 108, scc_size: 1 }
- **File**: src/060_path_detector.rs
- **Description**: Member of SCC 108 (size: 1)

### test_path_detector_diamond

- **Type**: Structural: SccMembership { scc_id: 109, scc_size: 1 }
- **File**: src/060_path_detector.rs
- **Description**: Member of SCC 109 (size: 1)

### test_extract_facts_from_path

- **Type**: Structural: SccMembership { scc_id: 110, scc_size: 1 }
- **File**: src/060_path_detector.rs
- **Description**: Member of SCC 110 (size: 1)

### test_path_stats

- **Type**: Structural: SccMembership { scc_id: 111, scc_size: 1 }
- **File**: src/060_path_detector.rs
- **Description**: Member of SCC 111 (size: 1)

### make_simple_analysis

- **Type**: Structural: SccMembership { scc_id: 112, scc_size: 1 }
- **File**: src/070_invariant_integrator.rs
- **Description**: Member of SCC 112 (size: 1)

### test_invariant_detector_creation

- **Type**: Structural: SccMembership { scc_id: 113, scc_size: 1 }
- **File**: src/070_invariant_integrator.rs
- **Description**: Member of SCC 113 (size: 1)

### test_detect_all

- **Type**: Structural: SccMembership { scc_id: 114, scc_size: 1 }
- **File**: src/070_invariant_integrator.rs
- **Description**: Member of SCC 114 (size: 1)

### resolve_source_root

- **Type**: Structural: SccMembership { scc_id: 115, scc_size: 1 }
- **File**: src/070_layer_utilities.rs
- **Description**: Member of SCC 115 (size: 1)

### allow_analysis_dir

- **Type**: Structural: SccMembership { scc_id: 116, scc_size: 1 }
- **File**: src/070_layer_utilities.rs
- **Description**: Member of SCC 116 (size: 1)

### gather_rust_files

- **Type**: Structural: SccMembership { scc_id: 117, scc_size: 1 }
- **File**: src/070_layer_utilities.rs
- **Description**: Member of SCC 117 (size: 1)

### main

- **Type**: Structural: SccMembership { scc_id: 118, scc_size: 1 }
- **File**: src/190_main.rs
- **Description**: Member of SCC 118 (size: 1)

### run_analysis

- **Type**: Structural: SccMembership { scc_id: 119, scc_size: 1 }
- **File**: src/070_layer_utilities.rs
- **Description**: Member of SCC 119 (size: 1)

### export_json

- **Type**: Structural: SccMembership { scc_id: 120, scc_size: 1 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Member of SCC 120 (size: 1)

### generate_invariant_report

- **Type**: Structural: SccMembership { scc_id: 121, scc_size: 1 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Member of SCC 121 (size: 1)

### export_constraints_json

- **Type**: Structural: SccMembership { scc_id: 122, scc_size: 1 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Member of SCC 122 (size: 1)

### test_generate_report

- **Type**: Structural: SccMembership { scc_id: 123, scc_size: 1 }
- **File**: src/080_invariant_reporter.rs
- **Description**: Member of SCC 123 (size: 1)

### generate_conscience_map

- **Type**: Structural: SccMembership { scc_id: 124, scc_size: 1 }
- **File**: src/082_conscience_graph.rs
- **Description**: Member of SCC 124 (size: 1)

### strength_emoji

- **Type**: Structural: SccMembership { scc_id: 125, scc_size: 1 }
- **File**: src/082_conscience_graph.rs
- **Description**: Member of SCC 125 (size: 1)

### kind_name

- **Type**: Structural: SccMembership { scc_id: 126, scc_size: 1 }
- **File**: src/082_conscience_graph.rs
- **Description**: Member of SCC 126 (size: 1)

### generate_conscience_stats

- **Type**: Structural: SccMembership { scc_id: 127, scc_size: 1 }
- **File**: src/082_conscience_graph.rs
- **Description**: Member of SCC 127 (size: 1)

### make_test_invariant

- **Type**: Structural: SccMembership { scc_id: 128, scc_size: 1 }
- **File**: src/085_agent_conscience.rs
- **Description**: Member of SCC 128 (size: 1)

### test_generate_stats

- **Type**: Structural: SccMembership { scc_id: 129, scc_size: 1 }
- **File**: src/082_conscience_graph.rs
- **Description**: Member of SCC 129 (size: 1)

### make_test_invariant

- **Type**: Structural: SccMembership { scc_id: 130, scc_size: 1 }
- **File**: src/085_agent_conscience.rs
- **Description**: Member of SCC 130 (size: 1)

### test_strength_emoji

- **Type**: Structural: SccMembership { scc_id: 131, scc_size: 1 }
- **File**: src/082_conscience_graph.rs
- **Description**: Member of SCC 131 (size: 1)

### matches_function

- **Type**: Structural: SccMembership { scc_id: 132, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 132 (size: 1)

### test_extract_layer

- **Type**: Structural: SccMembership { scc_id: 133, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 133 (size: 1)

### test_validate_no_move_constraint

- **Type**: Structural: SccMembership { scc_id: 134, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 134 (size: 1)

### test_validate_layer_fixed_constraint

- **Type**: Structural: SccMembership { scc_id: 135, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 135 (size: 1)

### test_validate_preserve_signature

- **Type**: Structural: SccMembership { scc_id: 136, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 136 (size: 1)

### test_validate_allowed_action

- **Type**: Structural: SccMembership { scc_id: 137, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 137 (size: 1)

### test_check_move_allowed

- **Type**: Structural: SccMembership { scc_id: 138, scc_size: 1 }
- **File**: src/083_action_validator.rs
- **Description**: Member of SCC 138 (size: 1)

### load_invariants

- **Type**: Structural: SccMembership { scc_id: 139, scc_size: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Member of SCC 139 (size: 1)

### check_action

- **Type**: Structural: SccMembership { scc_id: 140, scc_size: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Member of SCC 140 (size: 1)

### test_conscience_blocks_invalid_move

- **Type**: Structural: SccMembership { scc_id: 141, scc_size: 1 }
- **File**: src/085_agent_conscience.rs
- **Description**: Member of SCC 141 (size: 1)

### test_conscience_allows_valid_action

- **Type**: Structural: SccMembership { scc_id: 142, scc_size: 1 }
- **File**: src/085_agent_conscience.rs
- **Description**: Member of SCC 142 (size: 1)

### test_conscience_stats

- **Type**: Structural: SccMembership { scc_id: 143, scc_size: 1 }
- **File**: src/085_agent_conscience.rs
- **Description**: Member of SCC 143 (size: 1)

### test_query_allowed_actions

- **Type**: Structural: SccMembership { scc_id: 144, scc_size: 1 }
- **File**: src/085_agent_conscience.rs
- **Description**: Member of SCC 144 (size: 1)

### compress_path

- **Type**: Structural: SccMembership { scc_id: 145, scc_size: 1 }
- **File**: src/090_utilities.rs
- **Description**: Member of SCC 145 (size: 1)

### collect_directory_files

- **Type**: Structural: SccMembership { scc_id: 146, scc_size: 1 }
- **File**: src/090_utilities.rs
- **Description**: Member of SCC 146 (size: 1)

### path_common_prefix_len

- **Type**: Structural: SccMembership { scc_id: 147, scc_size: 1 }
- **File**: src/090_utilities.rs
- **Description**: Member of SCC 147 (size: 1)

### resolve_required_layer_path

- **Type**: Structural: SccMembership { scc_id: 148, scc_size: 1 }
- **File**: src/090_utilities.rs
- **Description**: Member of SCC 148 (size: 1)

### compute_move_metrics

- **Type**: Structural: SccMembership { scc_id: 149, scc_size: 1 }
- **File**: src/090_utilities.rs
- **Description**: Member of SCC 149 (size: 1)

### collect_move_items

- **Type**: Structural: SccMembership { scc_id: 150, scc_size: 1 }
- **File**: src/090_utilities.rs
- **Description**: Member of SCC 150 (size: 1)

### write_structural_batches

- **Type**: Structural: SccMembership { scc_id: 151, scc_size: 1 }
- **File**: src/090_utilities.rs
- **Description**: Member of SCC 151 (size: 1)

### write_cluster_batches

- **Type**: Structural: SccMembership { scc_id: 152, scc_size: 1 }
- **File**: src/090_utilities.rs
- **Description**: Member of SCC 152 (size: 1)

### collect_functions

- **Type**: Structural: SccMembership { scc_id: 153, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 153 (size: 1)

### build_name_map

- **Type**: Structural: SccMembership { scc_id: 154, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 154 (size: 1)

### build_call_edges

- **Type**: Structural: SccMembership { scc_id: 155, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 155 (size: 1)

### build_function_layers

- **Type**: Structural: SccMembership { scc_id: 156, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 156 (size: 1)

### build_type_maps

- **Type**: Structural: SccMembership { scc_id: 157, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 157 (size: 1)

### extract_identifiers

- **Type**: Structural: SccMembership { scc_id: 158, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 158 (size: 1)

### compute_type_coupling

- **Type**: Structural: SccMembership { scc_id: 159, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 159 (size: 1)

### build_call_analysis

- **Type**: Structural: SccMembership { scc_id: 160, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 160 (size: 1)

### determine_status

- **Type**: Structural: SccMembership { scc_id: 161, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 161 (size: 1)

### suggest_file

- **Type**: Structural: SccMembership { scc_id: 162, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 162 (size: 1)

### compute_cluster_cohesion

- **Type**: Structural: SccMembership { scc_id: 163, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 163 (size: 1)

### suggest_cluster_file

- **Type**: Structural: SccMembership { scc_id: 164, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 164 (size: 1)

### build_undirected_graph

- **Type**: Structural: SccMembership { scc_id: 165, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 165 (size: 1)

### louvain_communities

- **Type**: Structural: SccMembership { scc_id: 166, scc_size: 1 }
- **File**: src/110_cohesion_analyzer.rs
- **Description**: Member of SCC 166 (size: 1)

### is_source_file

- **Type**: Structural: SccMembership { scc_id: 167, scc_size: 1 }
- **File**: src/120_directory_analyzer.rs
- **Description**: Member of SCC 167 (size: 1)

### should_skip_path

- **Type**: Structural: SccMembership { scc_id: 168, scc_size: 1 }
- **File**: src/120_directory_analyzer.rs
- **Description**: Member of SCC 168 (size: 1)

### sanitize_identifier

- **Type**: Structural: SccMembership { scc_id: 169, scc_size: 1 }
- **File**: src/130_control_flow.rs
- **Description**: Member of SCC 169 (size: 1)

### parallel_build_file_dag

- **Type**: Structural: SccMembership { scc_id: 170, scc_size: 1 }
- **File**: src/140_file_ordering.rs
- **Description**: Member of SCC 170 (size: 1)

### slugify_relative

- **Type**: Structural: SccMembership { scc_id: 171, scc_size: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Member of SCC 171 (size: 1)

### resolve_julia_binary

- **Type**: Structural: SccMembership { scc_id: 172, scc_size: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Member of SCC 172 (size: 1)

### find_julia_project_dir

- **Type**: Structural: SccMembership { scc_id: 173, scc_size: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Member of SCC 173 (size: 1)

### parse_module_name

- **Type**: Structural: SccMembership { scc_id: 174, scc_size: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Member of SCC 174 (size: 1)

### parse_struct_name

- **Type**: Structural: SccMembership { scc_id: 175, scc_size: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Member of SCC 175 (size: 1)

### relativize_path

- **Type**: Structural: SccMembership { scc_id: 176, scc_size: 1 }
- **File**: src/160_rust_parser.rs
- **Description**: Member of SCC 176 (size: 1)

### is_reserved

- **Type**: Structural: SccMembership { scc_id: 177, scc_size: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Member of SCC 177 (size: 1)

### extract_calls_from_text

- **Type**: Structural: SccMembership { scc_id: 178, scc_size: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Member of SCC 178 (size: 1)

### extract_calls_from_lines

- **Type**: Structural: SccMembership { scc_id: 179, scc_size: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Member of SCC 179 (size: 1)

### paren_balance

- **Type**: Structural: SccMembership { scc_id: 180, scc_size: 1 }
- **File**: src/150_julia_parser.rs
- **Description**: Member of SCC 180 (size: 1)

### relativize_path

- **Type**: Structural: SccMembership { scc_id: 181, scc_size: 1 }
- **File**: src/160_rust_parser.rs
- **Description**: Member of SCC 181 (size: 1)

### truncate_label

- **Type**: Structural: SccMembership { scc_id: 182, scc_size: 1 }
- **File**: src/160_rust_parser.rs
- **Description**: Member of SCC 182 (size: 1)

### expr_snippet

- **Type**: Structural: SccMembership { scc_id: 183, scc_size: 1 }
- **File**: src/160_rust_parser.rs
- **Description**: Member of SCC 183 (size: 1)

### pat_snippet

- **Type**: Structural: SccMembership { scc_id: 184, scc_size: 1 }
- **File**: src/160_rust_parser.rs
- **Description**: Member of SCC 184 (size: 1)

### display_path

- **Type**: Structural: SccMembership { scc_id: 185, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 185 (size: 1)

### placement_status_label

- **Type**: Structural: SccMembership { scc_id: 186, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 186 (size: 1)

### placement_status_notes

- **Type**: Structural: SccMembership { scc_id: 187, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 187 (size: 1)

### collect_rename_items

- **Type**: Structural: SccMembership { scc_id: 188, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 188 (size: 1)

### collect_utility_candidates

- **Type**: Structural: SccMembership { scc_id: 189, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 189 (size: 1)

### directory_moves_to_plan

- **Type**: Structural: SccMembership { scc_id: 190, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 190 (size: 1)

### write_priority_section

- **Type**: Structural: SccMembership { scc_id: 191, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 191 (size: 1)

### write_structural_tips

- **Type**: Structural: SccMembership { scc_id: 192, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 192 (size: 1)

### write_cluster_tips

- **Type**: Structural: SccMembership { scc_id: 193, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 193 (size: 1)

### sort_plan_items

- **Type**: Structural: SccMembership { scc_id: 194, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 194 (size: 1)

### sort_cluster_items

- **Type**: Structural: SccMembership { scc_id: 195, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 195 (size: 1)

### cluster_priority

- **Type**: Structural: SccMembership { scc_id: 196, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 196 (size: 1)

### collect_cluster_items

- **Type**: Structural: SccMembership { scc_id: 197, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 197 (size: 1)

### load_cargo_warnings

- **Type**: Structural: SccMembership { scc_id: 198, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 198 (size: 1)

### parse_dead_code_warnings

- **Type**: Structural: SccMembership { scc_id: 199, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 199 (size: 1)

### parse_use_symbols

- **Type**: Structural: SccMembership { scc_id: 200, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 200 (size: 1)

### scan_crate_paths

- **Type**: Structural: SccMembership { scc_id: 201, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 201 (size: 1)

### collect_symbol_references

- **Type**: Structural: SccMembership { scc_id: 202, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 202 (size: 1)

### is_public_function

- **Type**: Structural: SccMembership { scc_id: 203, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 203 (size: 1)

### path_matches

- **Type**: Structural: SccMembership { scc_id: 204, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 204 (size: 1)

### is_entrypoint_main

- **Type**: Structural: SccMembership { scc_id: 205, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 205 (size: 1)

### referenced_elsewhere

- **Type**: Structural: SccMembership { scc_id: 206, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 206 (size: 1)

### is_dead_code_candidate

- **Type**: Structural: SccMembership { scc_id: 207, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 207 (size: 1)

### filter_orphaned

- **Type**: Structural: SccMembership { scc_id: 208, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 208 (size: 1)

### load_report_config

- **Type**: Structural: SccMembership { scc_id: 209, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 209 (size: 1)

### collect_size_warnings

- **Type**: Structural: SccMembership { scc_id: 210, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 210 (size: 1)

### load_baseline_metrics

- **Type**: Structural: SccMembership { scc_id: 211, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 211 (size: 1)

### baseline_deltas

- **Type**: Structural: SccMembership { scc_id: 212, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 212 (size: 1)

### write_baseline_metrics

- **Type**: Structural: SccMembership { scc_id: 213, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 213 (size: 1)

### collect_directories

- **Type**: Structural: SccMembership { scc_id: 214, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 214 (size: 1)

### slugify_path

- **Type**: Structural: SccMembership { scc_id: 215, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 215 (size: 1)

### render_mermaid_graph

- **Type**: Structural: SccMembership { scc_id: 216, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 216 (size: 1)

### compute_ordering_correctness

- **Type**: Structural: SccMembership { scc_id: 217, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 217 (size: 1)

### compute_directory_cohesion

- **Type**: Structural: SccMembership { scc_id: 218, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 218 (size: 1)

### prefix_key_from_path

- **Type**: Structural: SccMembership { scc_id: 219, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 219 (size: 1)

### slugify_key

- **Type**: Structural: SccMembership { scc_id: 220, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 220 (size: 1)

### group_key_cmp

- **Type**: Structural: SccMembership { scc_id: 221, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 221 (size: 1)

### function_bucket_label

- **Type**: Structural: SccMembership { scc_id: 222, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 222 (size: 1)

### slugify_file_path

- **Type**: Structural: SccMembership { scc_id: 223, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 223 (size: 1)

### language_label

- **Type**: Structural: SccMembership { scc_id: 224, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 224 (size: 1)

### visibility_label

- **Type**: Structural: SccMembership { scc_id: 225, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 225 (size: 1)

### short_signature

- **Type**: Structural: SccMembership { scc_id: 226, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 226 (size: 1)

### normalize_use_stmt

- **Type**: Structural: SccMembership { scc_id: 227, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 227 (size: 1)

### sanitize_mermaid_id

- **Type**: Structural: SccMembership { scc_id: 228, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 228 (size: 1)

### sanitize_mermaid_label

- **Type**: Structural: SccMembership { scc_id: 229, scc_size: 1 }
- **File**: src/180_report.rs
- **Description**: Member of SCC 229 (size: 1)

### main

- **Type**: Structural: SccMembership { scc_id: 230, scc_size: 1 }
- **File**: src/190_main.rs
- **Description**: Member of SCC 230 (size: 1)

### show_stats

- **Type**: Structural: SccMembership { scc_id: 231, scc_size: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Member of SCC 231 (size: 1)

### list_invariants

- **Type**: Structural: SccMembership { scc_id: 232, scc_size: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Member of SCC 232 (size: 1)

### query_function

- **Type**: Structural: SccMembership { scc_id: 233, scc_size: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Member of SCC 233 (size: 1)

### run_agent_cli

- **Type**: Structural: SccMembership { scc_id: 234, scc_size: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Member of SCC 234 (size: 1)

### test_load_invariants_empty

- **Type**: Structural: SccMembership { scc_id: 235, scc_size: 1 }
- **File**: src/191_agent_cli.rs
- **Description**: Member of SCC 235 (size: 1)

## Empirical Invariants (High Confidence)

These invariants were observed across multiple paths/samples and have high confidence.

### topo_sort

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### build_dependency_map

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### generate_invariant_report

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### compute_type_coupling

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### extract_calls_from_text

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### referenced_elsewhere

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### is_dead_code_candidate

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### query_function

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### list_invariants

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### show_stats

- **Type**: Structural: Bridge
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Potential bridge node in call graph

### build_directory_entry_map

- **Type**: Semantic: TypeStable { signature: "pub fn build_directory_entry_map (files : & [PathBuf] ,) -> Result < HashMap < PathBuf , crate :: types :: FileOrderEntry > > { use crate :: file_ordering :: { build_dependency_map , build_entries , build_file_dag , detect_cycles , ordered_by_name , topological_sort , } ; use crate :: layer_core :: layer_constrained_sort ; use crate :: layer_utilities :: build_file_layers ; use crate :: types :: FileOrderingResult ; use std :: collections :: HashSet ; const DEFAULT_STEP : usize = 10 ; if files . is_empty () { return Ok (HashMap :: new ()) ; } let file_set : HashSet < PathBuf > = files . iter () . cloned () . collect () ; let module_map = crate :: cluster_011 :: build_module_map (files) ; let dep_map = build_dependency_map (files , & file_set , & module_map) ? ; let file_layers = build_file_layers (files) ; let (graph , node_map) = build_file_dag (files , & dep_map) ; let cycles = detect_cycles (& graph , files) ; let ordered_nodes = if cycles . is_empty () { layer_constrained_sort (& graph , & file_layers) . unwrap_or_else (| _ | { topological_sort (& graph) . unwrap_or_else (| _ | ordered_by_name (files , & node_map)) }) } else { ordered_by_name (files , & node_map) } ; let ordered_files = ordered_nodes . into_iter () . map (| idx | graph [idx] . clone ()) . collect :: < Vec < _ > > () ; let ordering = FileOrderingResult { ordered_files : build_entries (& ordered_files , DEFAULT_STEP) , violations : Vec :: new () , layer_violations : Vec :: new () , ordered_directories : Vec :: new () , cycles , } ; let mut map = HashMap :: new () ; for entry in ordering . ordered_files { map . insert (entry . current_path . clone () , entry) ; } Ok (map) } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: pub fn build_directory_entry_map (files : & [PathBuf] ,) -> Result < HashMap < PathBuf , crate :: types :: FileOrderEntry > > { use crate :: file_ordering :: { build_dependency_map , build_entries , build_file_dag , detect_cycles , ordered_by_name , topological_sort , } ; use crate :: layer_core :: layer_constrained_sort ; use crate :: layer_utilities :: build_file_layers ; use crate :: types :: FileOrderingResult ; use std :: collections :: HashSet ; const DEFAULT_STEP : usize = 10 ; if files . is_empty () { return Ok (HashMap :: new ()) ; } let file_set : HashSet < PathBuf > = files . iter () . cloned () . collect () ; let module_map = crate :: cluster_011 :: build_module_map (files) ; let dep_map = build_dependency_map (files , & file_set , & module_map) ? ; let file_layers = build_file_layers (files) ; let (graph , node_map) = build_file_dag (files , & dep_map) ; let cycles = detect_cycles (& graph , files) ; let ordered_nodes = if cycles . is_empty () { layer_constrained_sort (& graph , & file_layers) . unwrap_or_else (| _ | { topological_sort (& graph) . unwrap_or_else (| _ | ordered_by_name (files , & node_map)) }) } else { ordered_by_name (files , & node_map) } ; let ordered_files = ordered_nodes . into_iter () . map (| idx | graph [idx] . clone ()) . collect :: < Vec < _ > > () ; let ordering = FileOrderingResult { ordered_files : build_entries (& ordered_files , DEFAULT_STEP) , violations : Vec :: new () , layer_violations : Vec :: new () , ordered_directories : Vec :: new () , cycles , } ; let mut map = HashMap :: new () ; for entry in ordering . ordered_files { map . insert (entry . current_path . clone () , entry) ; } Ok (map) } . sig

### collect_naming_warnings

- **Type**: Semantic: TypeStable { signature: "pub fn collect_naming_warnings (directory : & crate :: types :: DirectoryAnalysis , config : & crate :: report :: ReportConfig , warnings : & mut Vec < String > ,) -> Result < () > { use crate :: utilities :: compress_path ; use crate :: dependency :: naming_score_for_file ; if directory . path . components () . any (| comp | comp . as_os_str () == \"_old\") { return Ok (()) ; } let file_map = build_directory_entry_map (& directory . files) ? ; for file in & directory . files { if file . components () . any (| comp | comp . as_os_str () == \"_old\") { continue ; } let entry = file_map . get (file) ; if let Some (score) = naming_score_for_file (file , entry) { if score < config . naming_score_warning { let suggested = entry . map (| e | e . suggested_name . as_str ()) . unwrap_or (\"suggested name unavailable\") ; warnings . push (format ! (\"File `{}` has naming score {:.0}; consider renaming to `{}`.\" , compress_path (file . to_string_lossy () . as_ref ()) , score , suggested ,)) ; } } } for child in & directory . subdirectories { collect_naming_warnings (child , config , warnings) ? ; } Ok (()) } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: pub fn collect_naming_warnings (directory : & crate :: types :: DirectoryAnalysis , config : & crate :: report :: ReportConfig , warnings : & mut Vec < String > ,) -> Result < () > { use crate :: utilities :: compress_path ; use crate :: dependency :: naming_score_for_file ; if directory . path . components () . any (| comp | comp . as_os_str () == "_old") { return Ok (()) ; } let file_map = build_directory_entry_map (& directory . files) ? ; for file in & directory . files { if file . components () . any (| comp | comp . as_os_str () == "_old") { continue ; } let entry = file_map . get (file) ; if let Some (score) = naming_score_for_file (file , entry) { if score < config . naming_score_warning { let suggested = entry . map (| e | e . suggested_name . as_str ()) . unwrap_or ("suggested name unavailable") ; warnings . push (format ! ("File `{}` has naming score {:.0}; consider renaming to `{}`." , compress_path (file . to_string_lossy () . as_ref ()) , score , suggested ,)) ; } } } for child in & directory . subdirectories { collect_naming_warnings (child , config , warnings) ? ; } Ok (()) } . sig

### temp_dir

- **Type**: Semantic: TypeStable { signature: "# [cfg (test)] fn temp_dir (name : & str) -> PathBuf { let mut dir = std :: env :: temp_dir () ; dir . push (format ! (\"mmsb_analyzer_{}_{}\" , name , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_nanos ())) ; dir } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: # [cfg (test)] fn temp_dir (name : & str) -> PathBuf { let mut dir = std :: env :: temp_dir () ; dir . push (format ! ("mmsb_analyzer_{}_{}" , name , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_nanos ())) ; dir } . sig

### detects_cycles

- **Type**: Semantic: TypeStable { signature: "# [cfg (test)] fn detects_cycles () -> Result < () > { use crate :: dependency :: analyze_file_ordering ; use std :: fs :: { create_dir_all , write } ; let dir = temp_dir (\"cycle\") ; create_dir_all (& dir) ? ; let a = dir . join (\"a.rs\") ; let b = dir . join (\"b.rs\") ; write (& a , \"use crate::b; pub fn a() {}\") ? ; write (& b , \"use crate::a; pub fn b() {}\") ? ; let result = analyze_file_ordering (& [a . clone () , b . clone ()] , Some (10)) ? ; assert ! (! result . cycles . is_empty ()) ; Ok (()) } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: # [cfg (test)] fn detects_cycles () -> Result < () > { use crate :: dependency :: analyze_file_ordering ; use std :: fs :: { create_dir_all , write } ; let dir = temp_dir ("cycle") ; create_dir_all (& dir) ? ; let a = dir . join ("a.rs") ; let b = dir . join ("b.rs") ; write (& a , "use crate::b; pub fn a() {}") ? ; write (& b , "use crate::a; pub fn b() {}") ? ; let result = analyze_file_ordering (& [a . clone () , b . clone ()] , Some (10)) ? ; assert ! (! result . cycles . is_empty ()) ; Ok (()) } . sig

### generates_canonical_names_and_violations

- **Type**: Semantic: TypeStable { signature: "# [cfg (test)] fn generates_canonical_names_and_violations () -> Result < () > { use crate :: dependency :: analyze_file_ordering ; use std :: fs :: { create_dir_all , write } ; let dir = temp_dir (\"names\") ; create_dir_all (& dir) ? ; let a = dir . join (\"a.rs\") ; let b = dir . join (\"b.rs\") ; write (& a , \"use crate::b; pub fn a() {}\") ? ; write (& b , \"pub fn b() {}\") ? ; let result = analyze_file_ordering (& [a . clone () , b . clone ()] , Some (10)) ? ; let entries = & result . ordered_files ; assert_eq ! (entries [0] . suggested_name , \"000_b.rs\") ; assert_eq ! (entries [1] . suggested_name , \"010_a.rs\") ; assert ! (! result . violations . is_empty ()) ; Ok (()) } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: # [cfg (test)] fn generates_canonical_names_and_violations () -> Result < () > { use crate :: dependency :: analyze_file_ordering ; use std :: fs :: { create_dir_all , write } ; let dir = temp_dir ("names") ; create_dir_all (& dir) ? ; let a = dir . join ("a.rs") ; let b = dir . join ("b.rs") ; write (& a , "use crate::b; pub fn a() {}") ? ; write (& b , "pub fn b() {}") ? ; let result = analyze_file_ordering (& [a . clone () , b . clone ()] , Some (10)) ? ; let entries = & result . ordered_files ; assert_eq ! (entries [0] . suggested_name , "000_b.rs") ; assert_eq ! (entries [1] . suggested_name , "010_a.rs") ; assert ! (! result . violations . is_empty ()) ; Ok (()) } . sig

### topo_sort_orders_dependencies

- **Type**: Semantic: TypeStable { signature: "# [cfg (test)] # [allow (dead_code)] fn topo_sort_orders_dependencies () -> Result < () > { use crate :: dependency :: analyze_file_ordering ; use std :: fs :: { create_dir_all , write } ; let dir = temp_dir (\"topo\") ; create_dir_all (& dir) ? ; let a = dir . join (\"a.rs\") ; let b = dir . join (\"b.rs\") ; let c = dir . join (\"c.rs\") ; write (& a , \"pub fn a() {}\") ? ; write (& b , \"use crate::a; pub fn b() {}\") ? ; write (& c , \"use crate::b; pub fn c() {}\") ? ; let result = analyze_file_ordering (& [c . clone () , b . clone () , a . clone ()] , Some (10)) ? ; let ordered : Vec < _ > = result . ordered_files . iter () . map (| entry | entry . current_path . clone ()) . collect () ; assert_eq ! (ordered , vec ! [a , b , c]) ; assert ! (result . cycles . is_empty ()) ; Ok (()) } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: # [cfg (test)] # [allow (dead_code)] fn topo_sort_orders_dependencies () -> Result < () > { use crate :: dependency :: analyze_file_ordering ; use std :: fs :: { create_dir_all , write } ; let dir = temp_dir ("topo") ; create_dir_all (& dir) ? ; let a = dir . join ("a.rs") ; let b = dir . join ("b.rs") ; let c = dir . join ("c.rs") ; write (& a , "pub fn a() {}") ? ; write (& b , "use crate::a; pub fn b() {}") ? ; write (& c , "use crate::b; pub fn c() {}") ? ; let result = analyze_file_ordering (& [c . clone () , b . clone () , a . clone ()] , Some (10)) ? ; let ordered : Vec < _ > = result . ordered_files . iter () . map (| entry | entry . current_path . clone ()) . collect () ; assert_eq ! (ordered , vec ! [a , b , c]) ; assert ! (result . cycles . is_empty ()) ; Ok (()) } . sig

### layer_constrained_sort

- **Type**: Semantic: TypeStable { signature: "pub fn layer_constrained_sort (graph : & DiGraph < PathBuf , () > , file_layers : & HashMap < PathBuf , String > ,) -> Result < Vec < NodeIndex > > { use crate :: cluster_006 :: layer_prefix_value ; let mut layer_nodes : BTreeMap < i32 , Vec < NodeIndex > > = BTreeMap :: new () ; for node in graph . node_indices () { let file = & graph [node] ; let layer_name = file_layers . get (file) . cloned () . unwrap_or_else (| | \"root\" . to_string ()) ; let layer_value = layer_prefix_value (& layer_name) . unwrap_or (0) ; layer_nodes . entry (layer_value) . or_default () . push (node) ; } let mut ordered = Vec :: new () ; for (_layer , nodes) in layer_nodes { let sorted = topo_sort_within (graph , & nodes) ? ; ordered . extend (sorted) ; } Ok (ordered) } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: pub fn layer_constrained_sort (graph : & DiGraph < PathBuf , () > , file_layers : & HashMap < PathBuf , String > ,) -> Result < Vec < NodeIndex > > { use crate :: cluster_006 :: layer_prefix_value ; let mut layer_nodes : BTreeMap < i32 , Vec < NodeIndex > > = BTreeMap :: new () ; for node in graph . node_indices () { let file = & graph [node] ; let layer_name = file_layers . get (file) . cloned () . unwrap_or_else (| | "root" . to_string ()) ; let layer_value = layer_prefix_value (& layer_name) . unwrap_or (0) ; layer_nodes . entry (layer_value) . or_default () . push (node) ; } let mut ordered = Vec :: new () ; for (_layer , nodes) in layer_nodes { let sorted = topo_sort_within (graph , & nodes) ? ; ordered . extend (sorted) ; } Ok (ordered) } . sig

### topo_sort_within

- **Type**: Semantic: TypeStable { signature: "pub fn topo_sort_within (graph : & DiGraph < PathBuf , () > , nodes : & [NodeIndex] ,) -> Result < Vec < NodeIndex > > { let node_set : HashSet < NodeIndex > = nodes . iter () . copied () . collect () ; let mut indegree : HashMap < NodeIndex , usize > = HashMap :: new () ; for & node in nodes { indegree . insert (node , 0) ; } for & node in nodes { let incoming = graph . neighbors_directed (node , petgraph :: Direction :: Incoming) . filter (| n | node_set . contains (n)) . count () ; indegree . insert (node , incoming) ; } let mut queue = std :: collections :: VecDeque :: new () ; for & node in nodes { if indegree . get (& node) . copied () . unwrap_or (0) == 0 { queue . push_back (node) ; } } let mut ordered = Vec :: new () ; while let Some (node) = queue . pop_front () { ordered . push (node) ; for neighbor in graph . neighbors_directed (node , petgraph :: Direction :: Outgoing) { if ! node_set . contains (& neighbor) { continue ; } if let Some (entry) = indegree . get_mut (& neighbor) { * entry = entry . saturating_sub (1) ; if * entry == 0 { queue . push_back (neighbor) ; } } } } if ordered . len () != nodes . len () { return Err (anyhow :: anyhow ! (\"Cycle detected within layer group\")) ; } Ok (ordered) } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: pub fn topo_sort_within (graph : & DiGraph < PathBuf , () > , nodes : & [NodeIndex] ,) -> Result < Vec < NodeIndex > > { let node_set : HashSet < NodeIndex > = nodes . iter () . copied () . collect () ; let mut indegree : HashMap < NodeIndex , usize > = HashMap :: new () ; for & node in nodes { indegree . insert (node , 0) ; } for & node in nodes { let incoming = graph . neighbors_directed (node , petgraph :: Direction :: Incoming) . filter (| n | node_set . contains (n)) . count () ; indegree . insert (node , incoming) ; } let mut queue = std :: collections :: VecDeque :: new () ; for & node in nodes { if indegree . get (& node) . copied () . unwrap_or (0) == 0 { queue . push_back (node) ; } } let mut ordered = Vec :: new () ; while let Some (node) = queue . pop_front () { ordered . push (node) ; for neighbor in graph . neighbors_directed (node , petgraph :: Direction :: Outgoing) { if ! node_set . contains (& neighbor) { continue ; } if let Some (entry) = indegree . get_mut (& neighbor) { * entry = entry . saturating_sub (1) ; if * entry == 0 { queue . push_back (neighbor) ; } } } } if ordered . len () != nodes . len () { return Err (anyhow :: anyhow ! ("Cycle detected within layer group")) ; } Ok (ordered) } . sig

### detect_layer

- **Type**: Semantic: TypeStable { signature: "# [doc = \" Detects the layer identifier from a path by finding first digit-prefixed component\"] pub fn detect_layer (path : & Path) -> String { for component in path . components () { if let Some (name) = component . as_os_str () . to_str () { if let Some (first) = name . chars () . next () { if first . is_ascii_digit () { if let Some (pos) = name . find ('_') { if name [.. pos] . chars () . all (| c | c . is_ascii_digit ()) { return name . to_string () ; } } } } } } \"root\" . to_string () } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: # [doc = " Detects the layer identifier from a path by finding first digit-prefixed component"] pub fn detect_layer (path : & Path) -> String { for component in path . components () { if let Some (name) = component . as_os_str () . to_str () { if let Some (first) = name . chars () . next () { if first . is_ascii_digit () { if let Some (pos) = name . find ('_') { if name [.. pos] . chars () . all (| c | c . is_ascii_digit ()) { return name . to_string () ; } } } } } } "root" . to_string () } . sig

### rust_entry_paths

- **Type**: Semantic: TypeStable { signature: "pub fn rust_entry_paths (root : & Path) -> BTreeSet < PathBuf > { let src_dir = crate :: layer_utilities :: resolve_source_root (root) ; [\"lib.rs\" , \"main.rs\"] . iter () . map (| rel | src_dir . join (rel)) . filter (| p | p . exists ()) . collect () } . sig" }
- **Strength**: EMPIRICAL (paths: 1)
- **Confidence**: 0.50
- **Description**: Type signature: pub fn rust_entry_paths (root : & Path) -> BTreeSet < PathBuf > { let src_dir = crate :: layer_utilities :: resolve_source_root (root) ; ["lib.rs" , "main.rs"] . iter () . map (| rel | src_dir . join (rel)) . filter (| p | p . exists ()) . collect () } . sig

*... and 170 more*

## Heuristic Signals (Low Confidence - Review Required)

⚠️ **WARNING**: These are based on naming patterns and heuristics. They require manual verification and should **NOT block refactorings**.

- **is_mmsb_main**: Likely pure function (HEURISTIC - verify manually) (src/010_cluster_008.rs)
- **is_layer_violation**: Likely pure function (HEURISTIC - verify manually) (src/010_cluster_008.rs)
- **parse_cluster_members**: Likely pure function (HEURISTIC - verify manually) (src/010_cluster_008.rs)
- **is_core_module_path**: Likely pure function (HEURISTIC - verify manually) (src/010_cluster_008.rs)
- **compute_cohesion_score**: Likely pure function (HEURISTIC - verify manually) (src/050_cluster_006.rs)
- **compute_move_metrics**: Likely pure function (HEURISTIC - verify manually) (src/090_utilities.rs)
- **compute_cluster_cohesion**: Likely pure function (HEURISTIC - verify manually) (src/110_cohesion_analyzer.rs)
- **compute_type_coupling**: Likely pure function (HEURISTIC - verify manually) (src/110_cohesion_analyzer.rs)
- **is_source_file**: Likely pure function (HEURISTIC - verify manually) (src/120_directory_analyzer.rs)
- **parse_module_name**: Likely pure function (HEURISTIC - verify manually) (src/150_julia_parser.rs)

*... and 11 more (see JSON export)*

