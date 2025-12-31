# Layer Dependency Report

Generated: 2025-12-30 22:29:05

## Rust Layer Graph

### Layer Order
1. `000_cluster_001.rs`
2. `000_invariant_types.rs`
3. `005_refactor_constraints.rs`
4. `010_cluster_008.rs`
5. `010_scc_compressor.rs`
6. `020_cluster_010.rs`
7. `020_layer_inference.rs`
8. `030_cluster_011.rs`
9. `030_fixpoint_solver.rs`
10. `040_dependency.rs`
11. `040_structural_detector.rs`
12. `050_cluster_006.rs`
13. `050_semantic_detector.rs`
14. `060_layer_core.rs`
15. `060_path_detector.rs`
16. `070_invariant_integrator.rs`
17. `070_layer_utilities.rs`
18. `080_file_gathering.rs`
19. `080_invariant_reporter.rs`
20. `082_conscience_graph.rs`
21. `083_action_validator.rs`
22. `085_agent_conscience.rs`
23. `090_utilities.rs`
24. `100_types.rs`
25. `110_cohesion_analyzer.rs`
26. `120_directory_analyzer.rs`
27. `130_control_flow.rs`
28. `140_file_ordering.rs`
29. `150_julia_parser.rs`
30. `160_rust_parser.rs`
31. `170_dot_exporter.rs`
32. `180_report.rs`
33. `190_main.rs`
34. `191_agent_cli.rs`
35. `200_lib.rs`

### Layer Violations
- None detected.

### Dependency Edges
- No cross-layer dependencies recorded.

### Unresolved References
- src/150_julia_parser.rs → `use crate :: types :: * ;`
- src/030_cluster_011.rs → `use crate :: types :: ProgramCFG ;`
- src/120_directory_analyzer.rs → `use crate :: dependency :: detect_layer ;`
- src/120_directory_analyzer.rs → `use crate :: layer_utilities :: allow_analysis_dir ;`
- src/120_directory_analyzer.rs → `use crate :: types :: DirectoryAnalysis ;`
- src/085_agent_conscience.rs → `use crate :: action_validator :: { validate_action , AgentAction , ConstraintViolation } ;`
- src/085_agent_conscience.rs → `use crate :: invariant_types :: { Invariant , InvariantStrength } ;`
- src/085_agent_conscience.rs → `use crate :: refactor_constraints :: { from_invariant , RefactorConstraint } ;`
- src/085_agent_conscience.rs → `use crate :: invariant_types :: { InvariantKind , SemanticInvariant } ;`
- src/085_agent_conscience.rs → `use crate :: invariant_types :: { InvariantKind , StructuralInvariant } ;`
- src/140_file_ordering.rs → `use crate :: dependency :: build_module_map ;`
- src/140_file_ordering.rs → `use crate :: cluster_010 :: extract_dependencies ;`
- src/140_file_ordering.rs → `pub use crate :: cluster_001 :: { ordered_by_name , topological_sort } ;`
- src/140_file_ordering.rs → `pub use crate :: cluster_010 :: build_dependency_map ;`
- src/140_file_ordering.rs → `pub (crate) use crate :: cluster_001 :: build_entries ;`
- src/140_file_ordering.rs → `pub (crate) use crate :: cluster_011 :: build_file_dag ;`
- src/140_file_ordering.rs → `pub (crate) use crate :: cluster_001 :: detect_cycles ;`
- src/080_invariant_reporter.rs → `use crate :: invariant_types :: * ;`
- src/080_invariant_reporter.rs → `use crate :: refactor_constraints :: RefactorConstraint ;`
- src/090_utilities.rs → `use crate :: types :: { DirectoryAnalysis , FunctionPlacement , PlacementStatus } ;`
- src/090_utilities.rs → `use crate :: report :: { PlanItem , Priority , ActionKind , ClusterPlan } ;`
- src/050_cluster_006.rs → `use crate :: cluster_008 :: FunctionInfo ;`
- src/020_layer_inference.rs → `use crate :: invariant_types :: LayerInfo ;`
- src/060_path_detector.rs → `use crate :: invariant_types :: * ;`
- src/060_path_detector.rs → `use crate :: scc_compressor :: SccCompression ;`
- src/110_cohesion_analyzer.rs → `use crate :: cluster_006 :: compute_cohesion_score ;`
- src/110_cohesion_analyzer.rs → `use crate :: cluster_008 :: { detect_layer_violation , FunctionInfo } ;`
- src/110_cohesion_analyzer.rs → `use crate :: types :: { CallAnalysis , FunctionCluster , FunctionPlacement , PlacementStatus } ;`
- src/110_cohesion_analyzer.rs → `use crate :: types :: { AnalysisResult , ElementType } ;`
- src/050_semantic_detector.rs → `use crate :: invariant_types :: * ;`
- src/050_semantic_detector.rs → `use crate :: types :: { CodeElement , ElementType } ;`
- src/050_semantic_detector.rs → `use crate :: types :: { Language , Visibility } ;`
- src/010_cluster_008.rs → `use crate :: dependency :: { LayerEdge , LayerGraph , ReferenceDetail , UnresolvedDependency , } ;`
- src/010_cluster_008.rs → `use crate :: types :: { FileLayerViolation , NodeType } ;`
- src/130_control_flow.rs → `use crate :: types :: * ;`
- src/130_control_flow.rs → `use crate :: utilities :: compress_path ;`
- src/040_dependency.rs → `pub use crate :: cluster_010 :: order_julia_files_by_dependency ;`
- src/040_dependency.rs → `# [allow (unused_imports)] pub use crate :: cluster_001 :: { detect_layer , julia_entry_paths , order_rust_files_by_dependency } ;`
- src/040_dependency.rs → `pub use crate :: cluster_011 :: build_module_map ;`
- src/040_dependency.rs → `pub use crate :: cluster_001 :: { build_directory_entry_map , collect_naming_warnings } ;`
- src/040_dependency.rs → `# [allow (unused_imports)] pub use crate :: cluster_010 :: { extract_julia_dependencies , extract_rust_dependencies } ;`
- src/040_dependency.rs → `pub use crate :: cluster_001 :: { analyze_file_ordering , naming_score_for_file } ;`
- src/040_dependency.rs → `pub use crate :: cluster_011 :: { build_directory_dag , build_file_dependency_graph } ;`
- src/170_dot_exporter.rs → `# [allow (unused_imports)] pub use crate :: cluster_001 :: export_complete_program_dot ;`
- src/170_dot_exporter.rs → `pub use crate :: cluster_011 :: export_program_cfg_to_path ;`
- src/082_conscience_graph.rs → `use crate :: invariant_types :: * ;`
- src/180_report.rs → `use crate :: cluster_008 :: collect_cluster_plans ;`
- src/180_report.rs → `use crate :: layer_core :: { collect_directory_moves , sort_structural_items } ;`
- src/180_report.rs → `use crate :: utilities :: { compress_path , collect_move_items , write_structural_batches , write_cluster_batches } ;`
- src/180_report.rs → `use crate :: control_flow :: ControlFlowAnalyzer ;`
- src/180_report.rs → `use crate :: dependency :: { LayerGraph , build_directory_entry_map , build_file_dependency_graph , collect_naming_warnings } ;`
- src/180_report.rs → `use crate :: file_ordering :: DirectoryMove ;`
- src/180_report.rs → `use crate :: types :: * ;`
- src/180_report.rs → `use crate :: refactor_constraints :: RefactorConstraint ;`
- src/180_report.rs → `use crate :: action_validator :: check_move_allowed ;`
- src/040_structural_detector.rs → `use crate :: invariant_types :: * ;`
- src/040_structural_detector.rs → `use crate :: layer_inference :: infer_layers ;`
- src/040_structural_detector.rs → `use crate :: scc_compressor :: SccCompression ;`
- src/040_structural_detector.rs → `use crate :: types :: { AnalysisResult , ElementType } ;`
- src/070_invariant_integrator.rs → `use crate :: invariant_types :: * ;`
- src/070_invariant_integrator.rs → `use crate :: layer_inference :: { detect_layer_violations , infer_layers } ;`
- src/070_invariant_integrator.rs → `use crate :: path_detector :: PathDetector ;`
- src/070_invariant_integrator.rs → `use crate :: refactor_constraints :: generate_constraints ;`
- src/070_invariant_integrator.rs → `use crate :: semantic_detector :: SemanticDetector ;`
- src/070_invariant_integrator.rs → `use crate :: structural_detector :: StructuralDetector ;`
- src/070_invariant_integrator.rs → `use crate :: types :: { AnalysisResult , CallGraphNode } ;`
- src/070_invariant_integrator.rs → `use crate :: types :: { CodeElement , ElementType , Language , Visibility } ;`
- src/191_agent_cli.rs → `use crate :: action_validator :: AgentAction ;`
- src/191_agent_cli.rs → `use crate :: agent_conscience :: AgentConscience ;`
- src/191_agent_cli.rs → `use crate :: invariant_types :: Invariant ;`
- src/005_refactor_constraints.rs → `use crate :: invariant_types :: * ;`
- src/020_cluster_010.rs → `use crate :: dependency :: RootState ;`
- src/020_cluster_010.rs → `use crate :: cluster_001 :: { collect_julia_dependencies , JuliaTarget } ;`
- src/020_cluster_010.rs → `use crate :: dependency :: ReferenceDetail ;`
- src/000_cluster_001.rs → `use crate :: dependency :: { LayerGraph , ReferenceDetail , UnresolvedDependency } ;`
- src/000_cluster_001.rs → `use crate :: file_ordering :: { build_dependency_map , build_entries , build_file_dag , detect_cycles , ordered_by_name , topological_sort , } ;`
- src/000_cluster_001.rs → `use crate :: layer_core :: layer_constrained_sort ;`
- src/000_cluster_001.rs → `use crate :: layer_utilities :: build_file_layers ;`
- src/000_cluster_001.rs → `use crate :: types :: FileOrderingResult ;`
- src/000_cluster_001.rs → `use crate :: utilities :: compress_path ;`
- src/000_cluster_001.rs → `use crate :: dependency :: naming_score_for_file ;`
- src/000_cluster_001.rs → `use crate :: dependency :: analyze_file_ordering ;`
- src/000_cluster_001.rs → `use crate :: dependency :: analyze_file_ordering ;`
- src/000_cluster_001.rs → `use crate :: dependency :: analyze_file_ordering ;`
- src/000_cluster_001.rs → `use crate :: cluster_006 :: layer_prefix_value ;`
- src/070_layer_utilities.rs → `# [allow (unused_imports)] pub use crate :: cluster_001 :: { build_file_layers , detect_layer , gather_julia_files , julia_entry_paths } ;`
- src/070_layer_utilities.rs → `# [allow (unused_imports)] pub use crate :: cluster_010 :: contains_tools ;`
- src/070_layer_utilities.rs → `use crate :: control_flow :: ControlFlowAnalyzer ;`
- src/070_layer_utilities.rs → `use crate :: cohesion_analyzer :: FunctionCohesionAnalyzer ;`
- src/070_layer_utilities.rs → `use crate :: dependency :: LayerGraph ;`
- src/070_layer_utilities.rs → `use crate :: directory_analyzer :: DirectoryAnalyzer ;`
- src/070_layer_utilities.rs → `use crate :: dot_exporter :: export_program_cfg_to_path ;`
- src/070_layer_utilities.rs → `use crate :: julia_parser :: JuliaAnalyzer ;`
- src/070_layer_utilities.rs → `use crate :: report :: ReportGenerator ;`
- src/070_layer_utilities.rs → `use crate :: rust_parser :: RustAnalyzer ;`
- src/070_layer_utilities.rs → `use crate :: types :: { AnalysisResult , FileOrderingResult } ;`
- src/070_layer_utilities.rs → `use crate :: invariant_integrator :: InvariantDetector ;`
- src/070_layer_utilities.rs → `use crate :: invariant_reporter ;`
- src/083_action_validator.rs → `use crate :: refactor_constraints :: RefactorConstraint ;`
- src/083_action_validator.rs → `use crate :: invariant_types :: InvariantStrength ;`
- src/160_rust_parser.rs → `use crate :: types :: { AnalysisResult , CfgEdge , CfgNode , CodeElement , ElementType , FunctionCfg , Language , ModuleInfo , NodeType , Visibility , } ;`
- src/100_types.rs → `use crate :: invariant_types :: InvariantAnalysisResult ;`
- src/100_types.rs → `use crate :: refactor_constraints :: RefactorConstraint ;`
- src/060_layer_core.rs → `# [allow (unused_imports)] pub use crate :: cluster_006 :: { layer_prefix_value , order_directories , collect_directory_moves , } ;`
- src/060_layer_core.rs → `# [allow (unused_imports)] pub use crate :: cluster_008 :: detect_layer_violations ;`
- src/060_layer_core.rs → `# [allow (unused_imports)] pub use crate :: cluster_001 :: { layer_constrained_sort , topo_sort_within } ;`
- src/060_layer_core.rs → `use crate :: cluster_008 :: structural_layer_value ;`

## Julia Layer Graph

No layers discovered.

