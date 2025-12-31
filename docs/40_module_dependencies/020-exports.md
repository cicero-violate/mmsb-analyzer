# Module Exports

## src/050_cluster_010.rs (050_cluster_010.rs)

Module `050_cluster_010`

- `crate :: cluster_001 :: order_julia_files_by_dependency`
- `moved_gather_rust_files :: gather_rust_files`

## src/090_dependency.rs (090_dependency.rs)

Module `090_dependency`

- `# [allow (unused_imports)] pub use crate :: cluster_001 :: { detect_layer , julia_entry_paths , order_rust_files_by_dependency }`
- `# [allow (unused_imports)] pub use crate :: cluster_010 :: { extract_julia_dependencies , extract_rust_dependencies }`
- `crate :: cluster_001 :: { analyze_file_ordering , naming_score_for_file }`
- `crate :: cluster_001 :: { build_directory_entry_map , collect_naming_warnings }`
- `crate :: cluster_010 :: order_julia_files_by_dependency`
- `crate :: cluster_011 :: build_module_map`
- `crate :: cluster_011 :: { build_directory_dag , build_file_dependency_graph }`

## src/130_layer_core.rs (130_layer_core.rs)

Module `130_layer_core`

- `# [allow (unused_imports)] pub use crate :: cluster_001 :: { layer_constrained_sort , topo_sort_within }`
- `# [allow (unused_imports)] pub use crate :: cluster_006 :: { layer_prefix_value , order_directories , collect_directory_moves , }`
- `# [allow (unused_imports)] pub use crate :: cluster_008 :: detect_layer_violations`
- `crate :: cluster_008 :: sort_structural_items`

## src/160_layer_utilities.rs (160_layer_utilities.rs)

Module `160_layer_utilities`

- `# [allow (unused_imports)] pub use crate :: cluster_001 :: { build_file_layers , detect_layer , gather_julia_files , julia_entry_paths }`
- `# [allow (unused_imports)] pub use crate :: cluster_010 :: contains_tools`

## src/260_file_ordering.rs (260_file_ordering.rs)

Module `260_file_ordering`

- `crate :: cluster_001 :: { ordered_by_name , topological_sort }`
- `crate :: cluster_010 :: build_dependency_map`

## src/290_dot_exporter.rs (290_dot_exporter.rs)

Module `290_dot_exporter`

- `# [allow (unused_imports)] pub use crate :: cluster_001 :: export_complete_program_dot`
- `crate :: cluster_011 :: export_program_cfg_to_path`

## src/340_lib.rs (340_lib.rs)

Module `340_lib`

- `action_validator :: { AgentAction , ConstraintViolation , ViolationSeverity }`
- `agent_conscience :: { ActionPermission , AgentConscience }`
- `cohesion_analyzer :: FunctionCohesionAnalyzer`
- `conscience_graph :: { generate_conscience_map , generate_conscience_stats }`
- `control_flow :: ControlFlowAnalyzer`
- `dependency :: { julia_entry_paths , order_julia_files_by_dependency , order_rust_files_by_dependency , LayerGraph , build_file_dependency_graph , analyze_file_ordering , }`
- `directory_analyzer :: DirectoryAnalyzer`
- `dot_exporter :: { export_complete_program_dot , export_program_cfg_to_path }`
- `file_ordering :: { DagCache , parallel_build_file_dag }`
- `invariant_types :: *`
- `julia_parser :: JuliaAnalyzer`
- `refactor_constraints :: *`
- `report :: ReportGenerator`
- `rust_parser :: RustAnalyzer`
- `types :: AnalysisResult`

## src/390_dead_code_intent.rs (390_dead_code_intent.rs)

Module `390_dead_code_intent`

- `crate :: dead_code_attribute_parser :: detect_intent_signals`

## src/490_dead_code_cli.rs (490_dead_code_cli.rs)

Module `490_dead_code_cli`

- `crate :: layer_utilities :: run_dead_code_pipeline`

## src/600_action_impact_estimator.rs (600_action_impact_estimator.rs)

Module `600_action_impact_estimator`

- `crate :: quality_delta_calculator :: estimate_impact`

## src/630_correction_intelligence_report.rs (630_correction_intelligence_report.rs)

Module `630_correction_intelligence_report`

- `crate :: correction_plan_serializer :: write_intelligence_outputs_at`
- `crate :: violation_predictor :: generate_intelligence_report`

