# Module Imports

## src/000_cluster_001.rs (000_cluster_001.rs)

Module `000_cluster_001`

- `anyhow :: { Context , Result }`
- `crate :: cluster_001 :: { collect_julia_dependencies , JuliaTarget }`
- `crate :: cluster_006 :: layer_prefix_value`
- `crate :: cluster_010 :: { gather_rust_files , LayerResolver }`
- `crate :: cohesion_analyzer :: FunctionCohesionAnalyzer`
- `crate :: control_flow :: ControlFlowAnalyzer`
- `crate :: dependency :: LayerGraph`
- `crate :: dependency :: ReferenceDetail`
- `crate :: dependency :: analyze_file_ordering`
- `crate :: dependency :: naming_score_for_file`
- `crate :: dependency :: { LayerGraph , ReferenceDetail , UnresolvedDependency }`
- `crate :: directory_analyzer :: DirectoryAnalyzer`
- `crate :: dot_exporter :: export_program_cfg_to_path`
- `crate :: file_ordering :: { build_dependency_map , build_entries , build_file_dag , detect_cycles , ordered_by_name , topological_sort , }`
- `crate :: invariant_integrator :: InvariantDetector`
- `crate :: invariant_reporter`
- `crate :: julia_parser :: JuliaAnalyzer`
- `crate :: layer_core :: layer_constrained_sort`
- `crate :: layer_utilities :: build_file_layers`
- `crate :: report :: ReportGenerator`
- `crate :: rust_parser :: RustAnalyzer`
- `crate :: types :: FileOrderingResult`
- `crate :: types :: { AnalysisResult , FileOrderingResult }`
- `crate :: utilities :: compress_path`
- `once_cell :: sync :: Lazy`
- `petgraph :: Direction`
- `petgraph :: algo :: tarjan_scc`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `regex :: Regex`
- `std :: collections :: HashMap`
- `std :: collections :: HashSet`
- `std :: collections :: VecDeque`
- `std :: collections :: { BTreeMap , BTreeSet , HashMap , HashSet }`
- `std :: fmt :: Write`
- `std :: fs`
- `std :: fs :: { create_dir_all , write }`
- `std :: path :: { Path , PathBuf }`
- `super :: *`
- `syn :: visit :: Visit`
- `syn :: { ItemUse , UseTree }`
- `walkdir :: WalkDir`

## src/010_cluster_008.rs (010_cluster_008.rs)

Module `010_cluster_008`

- `anyhow :: Result`
- `crate :: dependency :: { LayerEdge , LayerGraph , ReferenceDetail , UnresolvedDependency , }`
- `crate :: types :: { FileLayerViolation , NodeType }`
- `petgraph :: graph :: DiGraph`
- `petgraph :: visit :: EdgeRef`
- `std :: cmp :: Ordering`
- `std :: collections :: HashMap`
- `std :: collections :: { BTreeMap , BTreeSet , HashMap , VecDeque }`
- `std :: path :: PathBuf`
- `std :: path :: { Path , PathBuf }`

## src/020_cluster_010.rs (020_cluster_010.rs)

Module `020_cluster_010`

- `crate :: layer_utilities :: allow_analysis_dir`
- `crate :: layer_utilities :: resolve_source_root`
- `std :: path :: Path`
- `std :: path :: PathBuf`
- `walkdir :: WalkDir`

## src/020_invariant_types.rs (020_invariant_types.rs)

Module `020_invariant_types`

- `serde :: { Deserialize , Serialize }`
- `std :: collections :: { HashMap , HashSet }`
- `std :: fmt`
- `super :: *`

## src/030_refactor_constraints.rs (030_refactor_constraints.rs)

Module `030_refactor_constraints`

- `crate :: invariant_types :: *`
- `serde :: { Deserialize , Serialize }`
- `std :: fmt`
- `super :: *`

## src/040_scc_compressor.rs (040_scc_compressor.rs)

Module `040_scc_compressor`

- `petgraph :: algo :: tarjan_scc`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `petgraph :: visit :: EdgeRef`
- `std :: collections :: HashMap`
- `super :: *`

## src/050_cluster_010.rs (050_cluster_010.rs)

Module `050_cluster_010`

- `anyhow :: { Context , Result }`
- `crate :: dependency :: RootState`
- `crate :: layer_utilities :: resolve_source_root`
- `once_cell :: sync :: Lazy`
- `regex :: Regex`
- `std :: collections :: { BTreeSet , HashMap , HashSet }`
- `std :: fs`
- `std :: path :: { Path , PathBuf }`
- `syn :: ItemUse`
- `syn :: visit :: Visit`
- `walkdir :: WalkDir`

## src/060_layer_inference.rs (060_layer_inference.rs)

Module `060_layer_inference`

- `crate :: invariant_types :: LayerInfo`
- `petgraph :: Direction`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `petgraph :: visit :: EdgeRef`
- `std :: collections :: HashMap`
- `super :: *`

## src/070_cluster_011.rs (070_cluster_011.rs)

Module `070_cluster_011`

- `anyhow :: Result`
- `crate :: types :: ProgramCFG`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: { HashMap , HashSet }`
- `std :: path :: { Path , PathBuf }`

## src/080_fixpoint_solver.rs (080_fixpoint_solver.rs)

Module `080_fixpoint_solver`

- `petgraph :: Direction`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: { HashMap , HashSet }`
- `super :: *`

## src/090_dependency.rs (090_dependency.rs)

Module `090_dependency`

- `std :: collections :: BTreeSet`
- `std :: path :: PathBuf`
- `syn :: UseTree`

## src/100_structural_detector.rs (100_structural_detector.rs)

Module `100_structural_detector`

- `crate :: invariant_types :: *`
- `crate :: layer_inference :: infer_layers`
- `crate :: scc_compressor :: SccCompression`
- `crate :: types :: { AnalysisResult , ElementType }`
- `petgraph :: Direction`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: HashMap`
- `super :: *`

## src/110_cluster_006.rs (110_cluster_006.rs)

Module `110_cluster_006`

- `crate :: cluster_008 :: FunctionInfo`
- `once_cell :: sync :: Lazy`
- `regex :: Regex`
- `std :: collections :: { BTreeMap , BTreeSet , HashMap , HashSet }`
- `std :: path :: { Path , PathBuf }`

## src/120_semantic_detector.rs (120_semantic_detector.rs)

Module `120_semantic_detector`

- `crate :: invariant_types :: *`
- `crate :: types :: { CodeElement , ElementType }`
- `crate :: types :: { Language , Visibility }`
- `regex :: Regex`
- `super :: *`

## src/140_path_detector.rs (140_path_detector.rs)

Module `140_path_detector`

- `crate :: invariant_types :: *`
- `crate :: scc_compressor :: SccCompression`
- `petgraph :: Direction`
- `petgraph :: algo :: all_simple_paths`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: HashSet`
- `super :: *`

## src/150_invariant_integrator.rs (150_invariant_integrator.rs)

Module `150_invariant_integrator`

- `crate :: invariant_types :: *`
- `crate :: layer_inference :: { detect_layer_violations , infer_layers }`
- `crate :: path_detector :: PathDetector`
- `crate :: refactor_constraints :: generate_constraints`
- `crate :: semantic_detector :: SemanticDetector`
- `crate :: structural_detector :: StructuralDetector`
- `crate :: types :: { AnalysisResult , CallGraphNode }`
- `crate :: types :: { CodeElement , ElementType , Language , Visibility }`
- `petgraph :: graph :: DiGraph`
- `std :: collections :: HashMap`
- `super :: *`

## src/160_layer_utilities.rs (160_layer_utilities.rs)

Module `160_layer_utilities`

- `anyhow :: Result`
- `clap :: Parser`
- `crate :: cluster_001 :: run_analysis`
- `crate :: cluster_010 :: gather_rust_files`
- `crate :: dead_code_actions :: recommend_action`
- `crate :: dead_code_attribute_parser :: { detect_test_modules , detect_test_symbols }`
- `crate :: dead_code_call_graph :: { build_call_graph , classify_symbol , is_reachable }`
- `crate :: dead_code_cli :: { DeadCodeRunConfig , is_test_path , merge_intent_map , reason_for_category }`
- `crate :: dead_code_confidence :: { assign_confidence , Evidence }`
- `crate :: dead_code_entrypoints :: { collect_entrypoints , collect_exports , is_public_api }`
- `crate :: dead_code_intent :: detect_intent_signals`
- `crate :: dead_code_report :: { build_report , write_outputs , DeadCodeReportMetadata , DeadCodeReportWithMeta , }`
- `crate :: dead_code_test_boundaries :: TestBoundaries`
- `crate :: dead_code_types :: { DeadCodeCategory , DeadCodeItem }`
- `crate :: types :: { CodeElement , ElementType , Language , Visibility }`
- `std :: collections :: HashMap`
- `std :: path :: { Path , PathBuf }`

## src/170_invariant_reporter.rs (170_invariant_reporter.rs)

Module `170_invariant_reporter`

- `crate :: invariant_types :: *`
- `crate :: refactor_constraints :: RefactorConstraint`
- `serde_json`
- `std :: collections :: HashMap`
- `std :: fs`
- `std :: path :: Path`
- `super :: *`

## src/180_conscience_graph.rs (180_conscience_graph.rs)

Module `180_conscience_graph`

- `crate :: invariant_types :: *`
- `std :: collections :: HashMap`
- `std :: path :: Path`
- `super :: *`

## src/190_action_validator.rs (190_action_validator.rs)

Module `190_action_validator`

- `crate :: invariant_types :: InvariantStrength`
- `crate :: refactor_constraints :: RefactorConstraint`
- `serde :: { Deserialize , Serialize }`
- `std :: path :: PathBuf`
- `super :: *`

## src/200_agent_conscience.rs (200_agent_conscience.rs)

Module `200_agent_conscience`

- `crate :: action_validator :: { validate_action , AgentAction , ConstraintViolation }`
- `crate :: invariant_types :: { Invariant , InvariantStrength }`
- `crate :: invariant_types :: { InvariantKind , SemanticInvariant }`
- `crate :: invariant_types :: { InvariantKind , StructuralInvariant }`
- `crate :: refactor_constraints :: { from_invariant , RefactorConstraint }`
- `serde :: { Deserialize , Serialize }`
- `std :: path :: { Path , PathBuf }`
- `super :: *`

## src/210_utilities.rs (210_utilities.rs)

Module `210_utilities`

- `crate :: report :: { PlanItem , Priority , ActionKind , ClusterPlan }`
- `crate :: types :: { DirectoryAnalysis , FunctionPlacement , PlacementStatus }`
- `std :: collections :: { BTreeSet , HashMap }`
- `std :: path :: { Path , PathBuf }`

## src/211_dead_code_attribute_parser.rs (211_dead_code_attribute_parser.rs)

Module `211_dead_code_attribute_parser`

- `# [doc = " Attribute parsing for dead code intent markers."] use std :: collections :: { HashMap , HashSet }`
- `crate :: dead_code_doc_comment_parser :: { extract_doc_markers , item_attrs , item_name , merge_doc_intent , }`
- `crate :: dead_code_intent :: { check_planned_directory , collect_symbols , merge_intent_sources , planned_directory_intent , DeadCodePolicy , }`
- `crate :: dead_code_test_boundaries :: has_test_attr`
- `crate :: dead_code_types :: { IntentMap , IntentMarker , IntentMetadata , IntentSource , IntentTag }`
- `std :: path :: Path`
- `syn :: { Attribute , Item }`

## src/220_types.rs (220_types.rs)

Module `220_types`

- `crate :: invariant_types :: InvariantAnalysisResult`
- `crate :: refactor_constraints :: RefactorConstraint`
- `serde :: { Deserialize , Serialize }`
- `std :: collections :: HashMap`
- `std :: path :: PathBuf`

## src/230_cohesion_analyzer.rs (230_cohesion_analyzer.rs)

Module `230_cohesion_analyzer`

- `anyhow :: Result`
- `crate :: cluster_006 :: compute_cohesion_score`
- `crate :: cluster_008 :: { detect_layer_violation , FunctionInfo }`
- `crate :: types :: { AnalysisResult , ElementType }`
- `crate :: types :: { CallAnalysis , FunctionCluster , FunctionPlacement , PlacementStatus }`
- `std :: collections :: { BTreeMap , HashMap , HashSet }`
- `std :: path :: PathBuf`

## src/240_directory_analyzer.rs (240_directory_analyzer.rs)

Module `240_directory_analyzer`

- `anyhow :: Result`
- `crate :: dependency :: detect_layer`
- `crate :: layer_utilities :: allow_analysis_dir`
- `crate :: types :: DirectoryAnalysis`
- `std :: fs`
- `std :: path :: { Path , PathBuf }`

## src/250_control_flow.rs (250_control_flow.rs)

Module `250_control_flow`

- `crate :: types :: *`
- `crate :: utilities :: compress_path`
- `petgraph :: dot :: Dot`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: HashMap`

## src/260_file_ordering.rs (260_file_ordering.rs)

Module `260_file_ordering`

- `(crate) use crate :: cluster_001 :: build_entries`
- `(crate) use crate :: cluster_001 :: detect_cycles`
- `(crate) use crate :: cluster_011 :: build_file_dag`
- `anyhow :: Result`
- `crate :: cluster_010 :: extract_dependencies`
- `crate :: dependency :: build_module_map`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `petgraph :: visit :: EdgeRef`
- `rayon :: prelude :: *`
- `std :: collections :: { HashMap , HashSet }`
- `std :: fs`
- `std :: path :: PathBuf`
- `std :: time :: SystemTime`

## src/270_julia_parser.rs (270_julia_parser.rs)

Module `270_julia_parser`

- `anyhow :: { anyhow , Context , Result }`
- `crate :: types :: *`
- `once_cell :: sync :: Lazy`
- `regex :: Regex`
- `std :: env`
- `std :: fs`
- `std :: path :: { Path , PathBuf }`
- `std :: process :: { Command , Stdio }`
- `std :: sync :: atomic :: { AtomicBool , Ordering }`

## src/280_rust_parser.rs (280_rust_parser.rs)

Module `280_rust_parser`

- `anyhow :: { Context , Result }`
- `crate :: types :: { AnalysisResult , CfgEdge , CfgNode , CodeElement , ElementType , FunctionCfg , Language , ModuleInfo , NodeType , Visibility , }`
- `std :: fs`
- `std :: path :: { Path , PathBuf }`
- `syn :: visit :: Visit`
- `syn :: { ItemEnum , ItemFn , ItemImpl , ItemMod , ItemStruct , ItemTrait , ItemUse }`

## src/310_report.rs (310_report.rs)

Module `310_report`

- `anyhow :: Result`
- `crate :: action_validator :: check_move_allowed`
- `crate :: cluster_008 :: collect_cluster_plans`
- `crate :: control_flow :: ControlFlowAnalyzer`
- `crate :: dependency :: { LayerGraph , build_directory_entry_map , build_file_dependency_graph , collect_naming_warnings }`
- `crate :: file_ordering :: DirectoryMove`
- `crate :: layer_core :: { collect_directory_moves , sort_structural_items }`
- `crate :: refactor_constraints :: RefactorConstraint`
- `crate :: types :: *`
- `crate :: types :: { ElementType , Language , Visibility }`
- `crate :: utilities :: { compress_path , collect_move_items , write_structural_batches , write_cluster_batches }`
- `std :: cmp :: Ordering`
- `std :: collections :: HashMap`
- `std :: collections :: { BTreeMap , BTreeSet , HashMap , HashSet }`
- `std :: fs`
- `std :: path :: PathBuf`
- `std :: path :: { Path , PathBuf }`
- `walkdir :: WalkDir`

## src/320_main.rs (320_main.rs)

Module `320_main`

- `anyhow :: Result`

## src/330_agent_cli.rs (330_agent_cli.rs)

Module `330_agent_cli`

- `anyhow :: Result`
- `clap :: Parser`
- `crate :: action_validator :: AgentAction`
- `crate :: agent_conscience :: AgentConscience`
- `crate :: invariant_types :: Invariant`
- `std :: path :: PathBuf`
- `super :: *`

## src/350_dead_code_types.rs (350_dead_code_types.rs)

Module `350_dead_code_types`

- `serde :: { Deserialize , Serialize }`
- `std :: collections :: HashMap`
- `std :: path :: PathBuf`

## src/370_dead_code_doc_comment_parser.rs (370_dead_code_doc_comment_parser.rs)

Module `370_dead_code_doc_comment_parser`

- `crate :: dead_code_types :: { IntentMarker , IntentMap }`
- `std :: collections :: { HashMap , HashSet }`
- `syn :: { Attribute , Item , Meta , MetaNameValue }`

## src/380_dead_code_call_graph.rs (380_dead_code_call_graph.rs)

Module `380_dead_code_call_graph`

- `crate :: dead_code_intent :: DeadCodePolicy`
- `crate :: dead_code_test_boundaries :: TestBoundaries`
- `crate :: dead_code_types :: { DeadCodeCategory , IntentMap }`
- `crate :: types :: { CodeElement , ElementType , Language }`
- `std :: collections :: { HashMap , HashSet , VecDeque }`

## src/390_dead_code_intent.rs (390_dead_code_intent.rs)

Module `390_dead_code_intent`

- `crate :: dead_code_doc_comment_parser :: item_name`
- `crate :: dead_code_types :: { IntentMap , IntentMarker , IntentMetadata , IntentSource , }`
- `std :: collections :: HashMap`
- `std :: path :: { Path , PathBuf }`

## src/400_dead_code_test_boundaries.rs (400_dead_code_test_boundaries.rs)

Module `400_dead_code_test_boundaries`

- `crate :: dead_code_call_graph :: { build_reverse_call_graph , CallGraph }`
- `std :: collections :: { HashSet , VecDeque }`
- `std :: path :: PathBuf`
- `syn :: { Attribute , Item }`

## src/410_dead_code_entrypoints.rs (410_dead_code_entrypoints.rs)

Module `410_dead_code_entrypoints`

- `crate :: dead_code_intent :: DeadCodePolicy`
- `crate :: types :: { CodeElement , ElementType , Visibility }`
- `std :: collections :: HashSet`
- `std :: path :: Path`
- `walkdir :: WalkDir`

## src/420_dead_code_classifier.rs (420_dead_code_classifier.rs)

Module `420_dead_code_classifier`

- `crate :: dead_code_call_graph :: CallGraph`
- `std :: collections :: HashSet`

## src/430_dead_code_confidence.rs (430_dead_code_confidence.rs)

Module `430_dead_code_confidence`

- `crate :: dead_code_types :: { ConfidenceLevel , DeadCodeCategory , DeadCodeItem }`

## src/440_dead_code_actions.rs (440_dead_code_actions.rs)

Module `440_dead_code_actions`

- `crate :: dead_code_types :: { ConfidenceLevel , DeadCodeCategory , RecommendedAction }`

## src/450_correction_plan_types.rs (450_correction_plan_types.rs)

Module `450_correction_plan_types`

- `crate :: types :: Visibility`
- `serde :: { Deserialize , Serialize }`
- `std :: path :: PathBuf`

## src/460_dead_code_report.rs (460_dead_code_report.rs)

Module `460_dead_code_report`

- `anyhow :: Result`
- `crate :: dead_code_cli :: DeadCodeRunConfig`
- `crate :: dead_code_report_split :: { write_plan_markdown , write_summary_markdown }`
- `crate :: dead_code_types :: { DeadCodeItem , DeadCodeReport , DeadCodeSummary }`
- `serde :: { Deserialize , Serialize }`
- `std :: path :: Path`

## src/470_dead_code_filter.rs (470_dead_code_filter.rs)

Module `470_dead_code_filter`

- `crate :: dead_code_report :: DeadCodeReportWithMeta`
- `crate :: dead_code_types :: { DeadCodeCategory }`
- `crate :: types :: CodeElement`
- `std :: collections :: HashSet`

## src/480_verification_policy_types.rs (480_verification_policy_types.rs)

Module `480_verification_policy_types`

- `serde :: { Deserialize , Serialize }`
- `std :: path :: PathBuf`

## src/490_dead_code_cli.rs (490_dead_code_cli.rs)

Module `490_dead_code_cli`

- `crate :: dead_code_intent :: DeadCodePolicy`
- `crate :: dead_code_types :: DeadCodeCategory`
- `std :: collections :: HashMap`
- `std :: path :: { Path , PathBuf }`

## src/500_quality_delta_types.rs (500_quality_delta_types.rs)

Module `500_quality_delta_types`

- `crate :: correction_plan_types :: ViolationType`
- `serde :: { Deserialize , Serialize }`

## src/510_dead_code_policy.rs (510_dead_code_policy.rs)

Module `510_dead_code_policy`

- `crate :: dead_code_intent :: DeadCodePolicy`
- `std :: path :: Path`

## src/520_violation_predictor.rs (520_violation_predictor.rs)

Module `520_violation_predictor`

- `crate :: action_impact_estimator :: { estimate_impact , AnalysisState as ImpactState }`
- `crate :: correction_intelligence_report :: { augment_path_coherence_strategies , compute_summary , fill_prediction_confidence , CorrectionIntelligenceReport , IntelligenceState , }`
- `crate :: correction_plan_generator :: generate_correction_plan`
- `crate :: correction_plan_types :: { RefactorAction , Severity , ViolationPrediction , ViolationType , }`
- `crate :: invariant_types :: InvariantAnalysisResult`
- `crate :: rollback_criteria_builder :: build_rollback_criteria`
- `crate :: types :: { CallGraphNode , CodeElement }`
- `crate :: verification_scope_planner :: plan_verification_scope`
- `std :: collections :: { HashMap , HashSet }`
- `std :: path :: PathBuf`

## src/530_dead_code_report_split.rs (530_dead_code_report_split.rs)

Module `530_dead_code_report_split`

- `crate :: dead_code_report :: DeadCodeReportWithMeta`
- `crate :: dead_code_types :: { DeadCodeCategory , DeadCodeItem , RecommendedAction }`
- `std :: path :: Path`

## src/540_tier_classifier.rs (540_tier_classifier.rs)

Module `540_tier_classifier`

- `crate :: correction_plan_types :: { ErrorTier , Severity , ViolationPrediction , ViolationType }`

## src/550_confidence_scorer.rs (550_confidence_scorer.rs)

Module `550_confidence_scorer`

- `crate :: correction_plan_types :: { ViolationPrediction , ViolationType }`

## src/560_correction_plan_generator.rs (560_correction_plan_generator.rs)

Module `560_correction_plan_generator`

- `crate :: correction_plan_types :: { CorrectionPlan , CorrectionStrategy , ErrorTier , RefactorAction , ViolationPrediction , ViolationType , VisibilityPlanOption , }`
- `crate :: tier_classifier :: classify_tier`

## src/570_verification_scope_planner.rs (570_verification_scope_planner.rs)

Module `570_verification_scope_planner`

- `crate :: correction_plan_types :: { CorrectionPlan , ErrorTier , RefactorAction }`
- `crate :: verification_policy_types :: { VerificationCheck , VerificationPolicy , VerificationScope , }`

## src/580_rollback_criteria_builder.rs (580_rollback_criteria_builder.rs)

Module `580_rollback_criteria_builder`

- `crate :: correction_plan_types :: { CorrectionPlan , ErrorTier , RefactorAction , ViolationType }`
- `crate :: quality_delta_types :: { RollbackCondition , RollbackCriteria }`

## src/590_quality_delta_calculator.rs (590_quality_delta_calculator.rs)

Module `590_quality_delta_calculator`

- `crate :: action_impact_estimator :: { AnalysisState , simulate_action }`
- `crate :: correction_plan_types :: RefactorAction`
- `crate :: quality_delta_types :: QualityDelta`

## src/600_action_impact_estimator.rs (600_action_impact_estimator.rs)

Module `600_action_impact_estimator`

- `crate :: correction_plan_types :: RefactorAction`
- `crate :: quality_delta_calculator :: Metrics`

## src/610_correction_plan_serializer.rs (610_correction_plan_serializer.rs)

Module `610_correction_plan_serializer`

- `# [doc = " Serialize correction plans to JSON values."] use std :: path :: Path`
- `crate :: correction_intelligence_report :: CorrectionIntelligenceReport`
- `crate :: correction_plan_types :: { CorrectionPlan , CorrectionStrategy }`
- `crate :: quality_delta_types :: RollbackCriteria`
- `crate :: verification_policy_emitter :: emit_verification_policy`
- `crate :: verification_policy_types :: { QualityThresholds , VerificationCheck , VerificationPolicy , VerificationScope }`
- `serde_json :: { json , Value }`

## src/620_verification_policy_emitter.rs (620_verification_policy_emitter.rs)

Module `620_verification_policy_emitter`

- `crate :: verification_policy_types :: { QualityThresholds , VerificationCheck , VerificationPolicy , VerificationScope , }`
- `serde_json :: json`
- `std :: path :: Path`

## src/630_correction_intelligence_report.rs (630_correction_intelligence_report.rs)

Module `630_correction_intelligence_report`

- `crate :: correction_plan_types :: { CorrectionPlan , CorrectionStrategy , ErrorTier , RefactorAction , ViolationPrediction , }`
- `crate :: invariant_types :: InvariantAnalysisResult`
- `crate :: quality_delta_calculator :: Metrics`
- `crate :: quality_delta_types :: { QualityDelta , RollbackCriteria }`
- `crate :: types :: { AnalysisResult , CallGraphNode , CodeElement }`
- `regex :: Regex`
- `serde :: { Deserialize , Serialize }`
- `std :: collections :: HashMap`
- `std :: path :: { Path , PathBuf }`
- `std :: { collections :: HashSet , fs }`

