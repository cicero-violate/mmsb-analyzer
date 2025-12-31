# Dead Code Plans (Review Only)

Generated: 2025-12-31T06:34:48.840984454-05:00

Policy: review_only. No automatic deletion or moves.
Guards: never delete public API; delete_safe requires manual confirmation + compiler dead_code warnings.

## Planned Items

- `temp_dir` in `src/000_cluster_001.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `topo_sort_orders_dependencies` in `src/000_cluster_001.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `collect_roots_from_crate` in `src/000_cluster_001.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `escape_dot` in `src/000_cluster_001.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `strength_emoji` in `src/180_conscience_graph.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `kind_name` in `src/180_conscience_graph.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `collect_functions` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `build_call_edges` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `build_function_layers` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `build_type_maps` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `build_name_map` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `build_call_analysis` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `determine_status` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `suggest_file` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `compute_cluster_cohesion` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `suggest_cluster_file` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `compute_type_coupling` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `extract_identifiers` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `louvain_communities` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `build_undirected_graph` in `src/230_cohesion_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `is_source_file` in `src/240_directory_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `should_skip_path` in `src/240_directory_analyzer.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `sanitize_identifier` in `src/250_control_flow.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `slugify_relative` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `resolve_julia_binary` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `find_julia_project_dir` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `parse_module_name` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `parse_struct_name` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `relativize_path` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `extract_calls_from_lines` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `extract_calls_from_text` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `is_reserved` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `paren_balance` in `src/270_julia_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `relativize_path` in `src/280_rust_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `expr_snippet` in `src/280_rust_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `pat_snippet` in `src/280_rust_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `truncate_label` in `src/280_rust_parser.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `display_path` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `placement_status_label` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `placement_status_notes` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `collect_rename_items` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `collect_utility_candidates` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `directory_moves_to_plan` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `write_priority_section` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `write_structural_tips` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `write_cluster_tips` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `sort_plan_items` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `sort_cluster_items` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `cluster_priority` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning
- `collect_cluster_items` in `src/310_report.rs` — Unreachable / CallGraph / DeleteSafe
  Plan: review_only; options: keep | quarantine | delete_safe (manual confirm); requires dead_code warning

