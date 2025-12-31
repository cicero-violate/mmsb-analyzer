# Layer Dependency Report

Generated: 2025-12-31 06:34:48

## Rust Layer Graph

### Layer Order
1. `000_cluster_001.rs`
2. `010_cluster_008.rs`
3. `020_cluster_010.rs`
4. `020_invariant_types.rs`
5. `030_refactor_constraints.rs`
6. `040_scc_compressor.rs`
7. `050_cluster_010.rs`
8. `060_layer_inference.rs`
9. `070_cluster_011.rs`
10. `080_fixpoint_solver.rs`
11. `090_dependency.rs`
12. `100_structural_detector.rs`
13. `110_cluster_006.rs`
14. `120_semantic_detector.rs`
15. `130_layer_core.rs`
16. `140_path_detector.rs`
17. `150_invariant_integrator.rs`
18. `160_layer_utilities.rs`
19. `170_invariant_reporter.rs`
20. `180_conscience_graph.rs`
21. `190_action_validator.rs`
22. `200_agent_conscience.rs`
23. `210_utilities.rs`
24. `211_dead_code_attribute_parser.rs`
25. `220_types.rs`
26. `230_cohesion_analyzer.rs`
27. `240_directory_analyzer.rs`
28. `250_control_flow.rs`
29. `260_file_ordering.rs`
30. `270_julia_parser.rs`
31. `280_rust_parser.rs`
32. `290_dot_exporter.rs`
33. `300_file_gathering.rs`
34. `310_report.rs`
35. `320_main.rs`
36. `330_agent_cli.rs`
37. `340_lib.rs`
38. `350_dead_code_types.rs`
39. `370_dead_code_doc_comment_parser.rs`
40. `380_dead_code_call_graph.rs`
41. `390_dead_code_intent.rs`
42. `400_dead_code_test_boundaries.rs`
43. `410_dead_code_entrypoints.rs`
44. `420_dead_code_classifier.rs`
45. `430_dead_code_confidence.rs`
46. `440_dead_code_actions.rs`
47. `450_correction_plan_types.rs`
48. `460_dead_code_report.rs`
49. `470_dead_code_filter.rs`
50. `480_verification_policy_types.rs`
51. `490_dead_code_cli.rs`
52. `500_quality_delta_types.rs`
53. `510_dead_code_policy.rs`
54. `520_violation_predictor.rs`
55. `530_dead_code_report_split.rs`
56. `540_tier_classifier.rs`
57. `550_confidence_scorer.rs`
58. `560_correction_plan_generator.rs`
59. `570_verification_scope_planner.rs`
60. `580_rollback_criteria_builder.rs`
61. `590_quality_delta_calculator.rs`
62. `600_action_impact_estimator.rs`
63. `610_correction_plan_serializer.rs`
64. `620_verification_policy_emitter.rs`
65. `630_correction_intelligence_report.rs`

### Layer Violations
- None detected.

### Dependency Edges
- No cross-layer dependencies recorded.

### Unresolved References
- src/580_rollback_criteria_builder.rs → `use crate :: correction_plan_types :: { CorrectionPlan , ErrorTier , RefactorAction , ViolationType } ;`
- src/580_rollback_criteria_builder.rs → `use crate :: quality_delta_types :: { RollbackCondition , RollbackCriteria } ;`
- src/050_cluster_010.rs → `use crate :: layer_utilities :: resolve_source_root ;`
- src/050_cluster_010.rs → `pub use crate :: cluster_001 :: order_julia_files_by_dependency ;`
- src/050_cluster_010.rs → `use crate :: dependency :: RootState ;`
- src/490_dead_code_cli.rs → `use crate :: dead_code_intent :: DeadCodePolicy ;`
- src/490_dead_code_cli.rs → `use crate :: dead_code_types :: DeadCodeCategory ;`
- src/490_dead_code_cli.rs → `pub use crate :: layer_utilities :: run_dead_code_pipeline ;`
- src/200_agent_conscience.rs → `use crate :: action_validator :: { validate_action , AgentAction , ConstraintViolation } ;`
- src/200_agent_conscience.rs → `use crate :: invariant_types :: { Invariant , InvariantStrength } ;`
- src/200_agent_conscience.rs → `use crate :: refactor_constraints :: { from_invariant , RefactorConstraint } ;`
- src/200_agent_conscience.rs → `use crate :: invariant_types :: { InvariantKind , SemanticInvariant } ;`
- src/200_agent_conscience.rs → `use crate :: invariant_types :: { InvariantKind , StructuralInvariant } ;`
- src/470_dead_code_filter.rs → `use crate :: dead_code_types :: { DeadCodeCategory } ;`
- src/470_dead_code_filter.rs → `use crate :: dead_code_report :: DeadCodeReportWithMeta ;`
- src/470_dead_code_filter.rs → `use crate :: types :: CodeElement ;`
- src/150_invariant_integrator.rs → `use crate :: invariant_types :: * ;`
- src/150_invariant_integrator.rs → `use crate :: layer_inference :: { detect_layer_violations , infer_layers } ;`
- src/150_invariant_integrator.rs → `use crate :: path_detector :: PathDetector ;`
- src/150_invariant_integrator.rs → `use crate :: refactor_constraints :: generate_constraints ;`
- src/150_invariant_integrator.rs → `use crate :: semantic_detector :: SemanticDetector ;`
- src/150_invariant_integrator.rs → `use crate :: structural_detector :: StructuralDetector ;`
- src/150_invariant_integrator.rs → `use crate :: types :: { AnalysisResult , CallGraphNode } ;`
- src/150_invariant_integrator.rs → `use crate :: types :: { CodeElement , ElementType , Language , Visibility } ;`
- src/550_confidence_scorer.rs → `use crate :: correction_plan_types :: { ViolationPrediction , ViolationType } ;`
- src/600_action_impact_estimator.rs → `use crate :: correction_plan_types :: RefactorAction ;`
- src/600_action_impact_estimator.rs → `use crate :: quality_delta_calculator :: Metrics ;`
- src/600_action_impact_estimator.rs → `pub use crate :: quality_delta_calculator :: estimate_impact ;`
- src/370_dead_code_doc_comment_parser.rs → `use crate :: dead_code_types :: { IntentMarker , IntentMap } ;`
- src/410_dead_code_entrypoints.rs → `use crate :: dead_code_intent :: DeadCodePolicy ;`
- src/410_dead_code_entrypoints.rs → `use crate :: types :: { CodeElement , ElementType , Visibility } ;`
- src/330_agent_cli.rs → `use crate :: action_validator :: AgentAction ;`
- src/330_agent_cli.rs → `use crate :: agent_conscience :: AgentConscience ;`
- src/330_agent_cli.rs → `use crate :: invariant_types :: Invariant ;`
- src/230_cohesion_analyzer.rs → `use crate :: cluster_006 :: compute_cohesion_score ;`
- src/230_cohesion_analyzer.rs → `use crate :: cluster_008 :: { detect_layer_violation , FunctionInfo } ;`
- src/230_cohesion_analyzer.rs → `use crate :: types :: { CallAnalysis , FunctionCluster , FunctionPlacement , PlacementStatus } ;`
- src/230_cohesion_analyzer.rs → `use crate :: types :: { AnalysisResult , ElementType } ;`
- src/590_quality_delta_calculator.rs → `use crate :: correction_plan_types :: RefactorAction ;`
- src/590_quality_delta_calculator.rs → `use crate :: quality_delta_types :: QualityDelta ;`
- src/590_quality_delta_calculator.rs → `use crate :: action_impact_estimator :: { AnalysisState , simulate_action } ;`
- src/430_dead_code_confidence.rs → `use crate :: dead_code_types :: { ConfidenceLevel , DeadCodeCategory , DeadCodeItem } ;`
- src/380_dead_code_call_graph.rs → `use crate :: dead_code_intent :: DeadCodePolicy ;`
- src/380_dead_code_call_graph.rs → `use crate :: dead_code_test_boundaries :: TestBoundaries ;`
- src/380_dead_code_call_graph.rs → `use crate :: dead_code_types :: { DeadCodeCategory , IntentMap } ;`
- src/380_dead_code_call_graph.rs → `use crate :: types :: { CodeElement , ElementType , Language } ;`
- src/620_verification_policy_emitter.rs → `use crate :: verification_policy_types :: { QualityThresholds , VerificationCheck , VerificationPolicy , VerificationScope , } ;`
- src/390_dead_code_intent.rs → `pub use crate :: dead_code_attribute_parser :: detect_intent_signals ;`
- src/390_dead_code_intent.rs → `use crate :: dead_code_doc_comment_parser :: item_name ;`
- src/390_dead_code_intent.rs → `use crate :: dead_code_types :: { IntentMap , IntentMarker , IntentMetadata , IntentSource , } ;`
- src/090_dependency.rs → `pub use crate :: cluster_010 :: order_julia_files_by_dependency ;`
- src/090_dependency.rs → `# [allow (unused_imports)] pub use crate :: cluster_001 :: { detect_layer , julia_entry_paths , order_rust_files_by_dependency } ;`
- src/090_dependency.rs → `pub use crate :: cluster_011 :: build_module_map ;`
- src/090_dependency.rs → `pub use crate :: cluster_001 :: { build_directory_entry_map , collect_naming_warnings } ;`
- src/090_dependency.rs → `# [allow (unused_imports)] pub use crate :: cluster_010 :: { extract_julia_dependencies , extract_rust_dependencies } ;`
- src/090_dependency.rs → `pub use crate :: cluster_001 :: { analyze_file_ordering , naming_score_for_file } ;`
- src/090_dependency.rs → `pub use crate :: cluster_011 :: { build_directory_dag , build_file_dependency_graph } ;`
- src/070_cluster_011.rs → `use crate :: types :: ProgramCFG ;`
- src/290_dot_exporter.rs → `# [allow (unused_imports)] pub use crate :: cluster_001 :: export_complete_program_dot ;`
- src/290_dot_exporter.rs → `pub use crate :: cluster_011 :: export_program_cfg_to_path ;`
- src/500_quality_delta_types.rs → `use crate :: correction_plan_types :: ViolationType ;`
- src/210_utilities.rs → `use crate :: types :: { DirectoryAnalysis , FunctionPlacement , PlacementStatus } ;`
- src/210_utilities.rs → `use crate :: report :: { PlanItem , Priority , ActionKind , ClusterPlan } ;`
- src/260_file_ordering.rs → `use crate :: dependency :: build_module_map ;`
- src/260_file_ordering.rs → `use crate :: cluster_010 :: extract_dependencies ;`
- src/260_file_ordering.rs → `pub use crate :: cluster_001 :: { ordered_by_name , topological_sort } ;`
- src/260_file_ordering.rs → `pub use crate :: cluster_010 :: build_dependency_map ;`
- src/260_file_ordering.rs → `pub (crate) use crate :: cluster_001 :: build_entries ;`
- src/260_file_ordering.rs → `pub (crate) use crate :: cluster_011 :: build_file_dag ;`
- src/260_file_ordering.rs → `pub (crate) use crate :: cluster_001 :: detect_cycles ;`
- src/130_layer_core.rs → `# [allow (unused_imports)] pub use crate :: cluster_006 :: { layer_prefix_value , order_directories , collect_directory_moves , } ;`
- src/130_layer_core.rs → `# [allow (unused_imports)] pub use crate :: cluster_008 :: detect_layer_violations ;`
- src/130_layer_core.rs → `# [allow (unused_imports)] pub use crate :: cluster_001 :: { layer_constrained_sort , topo_sort_within } ;`
- src/130_layer_core.rs → `pub use crate :: cluster_008 :: sort_structural_items ;`
- src/240_directory_analyzer.rs → `use crate :: dependency :: detect_layer ;`
- src/240_directory_analyzer.rs → `use crate :: layer_utilities :: allow_analysis_dir ;`
- src/240_directory_analyzer.rs → `use crate :: types :: DirectoryAnalysis ;`
- src/010_cluster_008.rs → `use crate :: dependency :: { LayerEdge , LayerGraph , ReferenceDetail , UnresolvedDependency , } ;`
- src/010_cluster_008.rs → `use crate :: types :: { FileLayerViolation , NodeType } ;`
- src/400_dead_code_test_boundaries.rs → `use crate :: dead_code_call_graph :: { build_reverse_call_graph , CallGraph } ;`
- src/630_correction_intelligence_report.rs → `use crate :: correction_plan_types :: { CorrectionPlan , CorrectionStrategy , ErrorTier , RefactorAction , ViolationPrediction , } ;`
- src/630_correction_intelligence_report.rs → `use crate :: quality_delta_calculator :: Metrics ;`
- src/630_correction_intelligence_report.rs → `use crate :: quality_delta_types :: { QualityDelta , RollbackCriteria } ;`
- src/630_correction_intelligence_report.rs → `use crate :: invariant_types :: InvariantAnalysisResult ;`
- src/630_correction_intelligence_report.rs → `use crate :: types :: { AnalysisResult , CallGraphNode , CodeElement } ;`
- src/630_correction_intelligence_report.rs → `pub use crate :: correction_plan_serializer :: write_intelligence_outputs_at ;`
- src/630_correction_intelligence_report.rs → `pub use crate :: violation_predictor :: generate_intelligence_report ;`
- src/250_control_flow.rs → `use crate :: types :: * ;`
- src/250_control_flow.rs → `use crate :: utilities :: compress_path ;`
- src/110_cluster_006.rs → `use crate :: cluster_008 :: FunctionInfo ;`
- src/450_correction_plan_types.rs → `use crate :: types :: Visibility ;`
- src/510_dead_code_policy.rs → `use crate :: dead_code_intent :: DeadCodePolicy ;`
- src/270_julia_parser.rs → `use crate :: types :: * ;`
- src/220_types.rs → `use crate :: invariant_types :: InvariantAnalysisResult ;`
- src/220_types.rs → `use crate :: refactor_constraints :: RefactorConstraint ;`
- src/540_tier_classifier.rs → `use crate :: correction_plan_types :: { ErrorTier , Severity , ViolationPrediction , ViolationType } ;`
- src/610_correction_plan_serializer.rs → `use crate :: correction_intelligence_report :: CorrectionIntelligenceReport ;`
- src/610_correction_plan_serializer.rs → `use crate :: correction_plan_types :: { CorrectionPlan , CorrectionStrategy } ;`
- src/610_correction_plan_serializer.rs → `use crate :: quality_delta_types :: RollbackCriteria ;`
- src/610_correction_plan_serializer.rs → `use crate :: verification_policy_emitter :: emit_verification_policy ;`
- src/610_correction_plan_serializer.rs → `use crate :: verification_policy_types :: { QualityThresholds , VerificationCheck , VerificationPolicy , VerificationScope } ;`
- src/190_action_validator.rs → `use crate :: refactor_constraints :: RefactorConstraint ;`
- src/190_action_validator.rs → `use crate :: invariant_types :: InvariantStrength ;`
- src/160_layer_utilities.rs → `use crate :: cluster_001 :: run_analysis ;`
- src/160_layer_utilities.rs → `use crate :: cluster_010 :: gather_rust_files ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_actions :: recommend_action ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_call_graph :: { build_call_graph , classify_symbol , is_reachable } ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_cli :: { DeadCodeRunConfig , is_test_path , merge_intent_map , reason_for_category } ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_confidence :: { assign_confidence , Evidence } ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_entrypoints :: { collect_entrypoints , collect_exports , is_public_api } ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_attribute_parser :: { detect_test_modules , detect_test_symbols } ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_intent :: detect_intent_signals ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_report :: { build_report , write_outputs , DeadCodeReportMetadata , DeadCodeReportWithMeta , } ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_test_boundaries :: TestBoundaries ;`
- src/160_layer_utilities.rs → `use crate :: dead_code_types :: { DeadCodeCategory , DeadCodeItem } ;`
- src/160_layer_utilities.rs → `use crate :: types :: { CodeElement , ElementType , Language , Visibility } ;`
- src/160_layer_utilities.rs → `# [allow (unused_imports)] pub use crate :: cluster_001 :: { build_file_layers , detect_layer , gather_julia_files , julia_entry_paths } ;`
- src/160_layer_utilities.rs → `# [allow (unused_imports)] pub use crate :: cluster_010 :: contains_tools ;`
- src/310_report.rs → `use crate :: cluster_008 :: collect_cluster_plans ;`
- src/310_report.rs → `use crate :: layer_core :: { collect_directory_moves , sort_structural_items } ;`
- src/310_report.rs → `use crate :: utilities :: { compress_path , collect_move_items , write_structural_batches , write_cluster_batches } ;`
- src/310_report.rs → `use crate :: control_flow :: ControlFlowAnalyzer ;`
- src/310_report.rs → `use crate :: dependency :: { LayerGraph , build_directory_entry_map , build_file_dependency_graph , collect_naming_warnings } ;`
- src/310_report.rs → `use crate :: file_ordering :: DirectoryMove ;`
- src/310_report.rs → `use crate :: types :: * ;`
- src/310_report.rs → `use crate :: types :: { ElementType , Language , Visibility } ;`
- src/310_report.rs → `use crate :: refactor_constraints :: RefactorConstraint ;`
- src/310_report.rs → `use crate :: action_validator :: check_move_allowed ;`
- src/140_path_detector.rs → `use crate :: invariant_types :: * ;`
- src/140_path_detector.rs → `use crate :: scc_compressor :: SccCompression ;`
- src/560_correction_plan_generator.rs → `use crate :: correction_plan_types :: { CorrectionPlan , CorrectionStrategy , ErrorTier , RefactorAction , ViolationPrediction , ViolationType , VisibilityPlanOption , } ;`
- src/560_correction_plan_generator.rs → `use crate :: tier_classifier :: classify_tier ;`
- src/440_dead_code_actions.rs → `use crate :: dead_code_types :: { ConfidenceLevel , DeadCodeCategory , RecommendedAction } ;`
- src/460_dead_code_report.rs → `use crate :: dead_code_cli :: DeadCodeRunConfig ;`
- src/460_dead_code_report.rs → `use crate :: dead_code_report_split :: { write_plan_markdown , write_summary_markdown } ;`
- src/460_dead_code_report.rs → `use crate :: dead_code_types :: { DeadCodeItem , DeadCodeReport , DeadCodeSummary } ;`
- src/570_verification_scope_planner.rs → `use crate :: correction_plan_types :: { CorrectionPlan , ErrorTier , RefactorAction } ;`
- src/570_verification_scope_planner.rs → `use crate :: verification_policy_types :: { VerificationCheck , VerificationPolicy , VerificationScope , } ;`
- src/100_structural_detector.rs → `use crate :: invariant_types :: * ;`
- src/100_structural_detector.rs → `use crate :: layer_inference :: infer_layers ;`
- src/100_structural_detector.rs → `use crate :: scc_compressor :: SccCompression ;`
- src/100_structural_detector.rs → `use crate :: types :: { AnalysisResult , ElementType } ;`
- src/530_dead_code_report_split.rs → `use crate :: dead_code_report :: DeadCodeReportWithMeta ;`
- src/530_dead_code_report_split.rs → `use crate :: dead_code_types :: { DeadCodeCategory , DeadCodeItem , RecommendedAction } ;`
- src/120_semantic_detector.rs → `use crate :: invariant_types :: * ;`
- src/120_semantic_detector.rs → `use crate :: types :: { CodeElement , ElementType } ;`
- src/120_semantic_detector.rs → `use crate :: types :: { Language , Visibility } ;`
- src/520_violation_predictor.rs → `use crate :: correction_plan_types :: { RefactorAction , Severity , ViolationPrediction , ViolationType , } ;`
- src/520_violation_predictor.rs → `use crate :: correction_intelligence_report :: { augment_path_coherence_strategies , compute_summary , fill_prediction_confidence , CorrectionIntelligenceReport , IntelligenceState , } ;`
- src/520_violation_predictor.rs → `use crate :: correction_plan_generator :: generate_correction_plan ;`
- src/520_violation_predictor.rs → `use crate :: invariant_types :: InvariantAnalysisResult ;`
- src/520_violation_predictor.rs → `use crate :: rollback_criteria_builder :: build_rollback_criteria ;`
- src/520_violation_predictor.rs → `use crate :: types :: { CallGraphNode , CodeElement } ;`
- src/520_violation_predictor.rs → `use crate :: verification_scope_planner :: plan_verification_scope ;`
- src/520_violation_predictor.rs → `use crate :: action_impact_estimator :: { estimate_impact , AnalysisState as ImpactState } ;`
- src/020_cluster_010.rs → `use crate :: layer_utilities :: allow_analysis_dir ;`
- src/020_cluster_010.rs → `use crate :: layer_utilities :: resolve_source_root ;`
- src/000_cluster_001.rs → `use crate :: cluster_010 :: { gather_rust_files , LayerResolver } ;`
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
- src/000_cluster_001.rs → `use crate :: cluster_001 :: { collect_julia_dependencies , JuliaTarget } ;`
- src/000_cluster_001.rs → `use crate :: dependency :: ReferenceDetail ;`
- src/000_cluster_001.rs → `use crate :: control_flow :: ControlFlowAnalyzer ;`
- src/000_cluster_001.rs → `use crate :: cohesion_analyzer :: FunctionCohesionAnalyzer ;`
- src/000_cluster_001.rs → `use crate :: dependency :: LayerGraph ;`
- src/000_cluster_001.rs → `use crate :: directory_analyzer :: DirectoryAnalyzer ;`
- src/000_cluster_001.rs → `use crate :: dot_exporter :: export_program_cfg_to_path ;`
- src/000_cluster_001.rs → `use crate :: julia_parser :: JuliaAnalyzer ;`
- src/000_cluster_001.rs → `use crate :: report :: ReportGenerator ;`
- src/000_cluster_001.rs → `use crate :: rust_parser :: RustAnalyzer ;`
- src/000_cluster_001.rs → `use crate :: types :: { AnalysisResult , FileOrderingResult } ;`
- src/000_cluster_001.rs → `use crate :: invariant_integrator :: InvariantDetector ;`
- src/000_cluster_001.rs → `use crate :: invariant_reporter ;`
- src/170_invariant_reporter.rs → `use crate :: invariant_types :: * ;`
- src/170_invariant_reporter.rs → `use crate :: refactor_constraints :: RefactorConstraint ;`
- src/280_rust_parser.rs → `use crate :: types :: { AnalysisResult , CfgEdge , CfgNode , CodeElement , ElementType , FunctionCfg , Language , ModuleInfo , NodeType , Visibility , } ;`
- src/180_conscience_graph.rs → `use crate :: invariant_types :: * ;`
- src/060_layer_inference.rs → `use crate :: invariant_types :: LayerInfo ;`
- src/030_refactor_constraints.rs → `use crate :: invariant_types :: * ;`
- src/420_dead_code_classifier.rs → `use crate :: dead_code_call_graph :: CallGraph ;`
- src/211_dead_code_attribute_parser.rs → `use crate :: dead_code_doc_comment_parser :: { extract_doc_markers , item_attrs , item_name , merge_doc_intent , } ;`
- src/211_dead_code_attribute_parser.rs → `use crate :: dead_code_intent :: { check_planned_directory , collect_symbols , merge_intent_sources , planned_directory_intent , DeadCodePolicy , } ;`
- src/211_dead_code_attribute_parser.rs → `use crate :: dead_code_test_boundaries :: has_test_attr ;`
- src/211_dead_code_attribute_parser.rs → `use crate :: dead_code_types :: { IntentMap , IntentMarker , IntentMetadata , IntentSource , IntentTag } ;`

## Julia Layer Graph

No layers discovered.

