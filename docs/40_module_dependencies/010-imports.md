# Module Imports

## src/000_cluster_001.rs (000_cluster_001.rs)

Module `000_cluster_001`

- `anyhow :: { Context , Result }`
- `crate :: cluster_006 :: layer_prefix_value`
- `crate :: dependency :: analyze_file_ordering`
- `crate :: dependency :: naming_score_for_file`
- `crate :: dependency :: { LayerGraph , ReferenceDetail , UnresolvedDependency }`
- `crate :: file_ordering :: { build_dependency_map , build_entries , build_file_dag , detect_cycles , ordered_by_name , topological_sort , }`
- `crate :: layer_core :: layer_constrained_sort`
- `crate :: layer_utilities :: build_file_layers`
- `crate :: types :: FileOrderingResult`
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

## src/000_invariant_types.rs (000_invariant_types.rs)

Module `000_invariant_types`

- `serde :: { Deserialize , Serialize }`
- `std :: collections :: { HashMap , HashSet }`
- `std :: fmt`
- `super :: *`

## src/005_refactor_constraints.rs (005_refactor_constraints.rs)

Module `005_refactor_constraints`

- `crate :: invariant_types :: *`
- `serde :: { Deserialize , Serialize }`
- `std :: collections :: HashSet`
- `std :: fmt`
- `super :: *`

## src/010_cluster_008.rs (010_cluster_008.rs)

Module `010_cluster_008`

- `anyhow :: Result`
- `crate :: dependency :: { LayerEdge , LayerGraph , ReferenceDetail , UnresolvedDependency , }`
- `crate :: types :: { FileLayerViolation , NodeType }`
- `petgraph :: graph :: DiGraph`
- `petgraph :: visit :: EdgeRef`
- `std :: cmp :: Ordering`
- `std :: collections :: { BTreeMap , BTreeSet , HashMap , VecDeque }`
- `std :: path :: { Path , PathBuf }`

## src/010_scc_compressor.rs (010_scc_compressor.rs)

Module `010_scc_compressor`

- `petgraph :: algo :: tarjan_scc`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `petgraph :: visit :: EdgeRef`
- `std :: collections :: HashMap`
- `super :: *`

## src/020_cluster_010.rs (020_cluster_010.rs)

Module `020_cluster_010`

- `anyhow :: { Context , Result }`
- `crate :: cluster_001 :: { collect_julia_dependencies , JuliaTarget }`
- `crate :: dependency :: ReferenceDetail`
- `crate :: dependency :: RootState`
- `once_cell :: sync :: Lazy`
- `regex :: Regex`
- `std :: collections :: { BTreeMap , BTreeSet , HashMap , HashSet }`
- `std :: fs`
- `std :: path :: { Path , PathBuf }`
- `syn :: ItemUse`
- `syn :: visit :: Visit`
- `walkdir :: WalkDir`

## src/020_layer_inference.rs (020_layer_inference.rs)

Module `020_layer_inference`

- `crate :: invariant_types :: LayerInfo`
- `petgraph :: Direction`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `petgraph :: visit :: EdgeRef`
- `std :: collections :: HashMap`
- `super :: *`

## src/030_cluster_011.rs (030_cluster_011.rs)

Module `030_cluster_011`

- `anyhow :: Result`
- `crate :: types :: ProgramCFG`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: { HashMap , HashSet }`
- `std :: path :: { Path , PathBuf }`

## src/030_fixpoint_solver.rs (030_fixpoint_solver.rs)

Module `030_fixpoint_solver`

- `petgraph :: Direction`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: { HashMap , HashSet }`
- `super :: *`

## src/040_dependency.rs (040_dependency.rs)

Module `040_dependency`

- `std :: collections :: BTreeSet`
- `std :: path :: PathBuf`
- `syn :: UseTree`

## src/040_structural_detector.rs (040_structural_detector.rs)

Module `040_structural_detector`

- `crate :: invariant_types :: *`
- `crate :: layer_inference :: infer_layers`
- `crate :: scc_compressor :: SccCompression`
- `crate :: types :: { AnalysisResult , ElementType }`
- `petgraph :: Direction`
- `petgraph :: algo :: is_cyclic_directed`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: HashMap`
- `std :: collections :: HashSet`
- `super :: *`

## src/050_cluster_006.rs (050_cluster_006.rs)

Module `050_cluster_006`

- `crate :: cluster_008 :: FunctionInfo`
- `once_cell :: sync :: Lazy`
- `regex :: Regex`
- `std :: collections :: { BTreeMap , BTreeSet , HashMap , HashSet }`
- `std :: path :: { Path , PathBuf }`

## src/050_semantic_detector.rs (050_semantic_detector.rs)

Module `050_semantic_detector`

- `crate :: invariant_types :: *`
- `crate :: types :: { CodeElement , ElementType }`
- `crate :: types :: { Language , Visibility }`
- `regex :: Regex`
- `std :: collections :: HashSet`
- `super :: *`

## src/060_layer_core.rs (060_layer_core.rs)

Module `060_layer_core`

- `crate :: cluster_008 :: structural_layer_value`
- `std :: collections :: HashMap`
- `std :: path :: PathBuf`

## src/060_path_detector.rs (060_path_detector.rs)

Module `060_path_detector`

- `crate :: invariant_types :: *`
- `crate :: scc_compressor :: SccCompression`
- `petgraph :: Direction`
- `petgraph :: algo :: all_simple_paths`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: { HashMap , HashSet }`
- `super :: *`

## src/070_invariant_integrator.rs (070_invariant_integrator.rs)

Module `070_invariant_integrator`

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

## src/070_layer_utilities.rs (070_layer_utilities.rs)

Module `070_layer_utilities`

- `anyhow :: { Context , Result }`
- `clap :: Parser`
- `crate :: cohesion_analyzer :: FunctionCohesionAnalyzer`
- `crate :: control_flow :: ControlFlowAnalyzer`
- `crate :: dependency :: LayerGraph`
- `crate :: directory_analyzer :: DirectoryAnalyzer`
- `crate :: dot_exporter :: export_program_cfg_to_path`
- `crate :: invariant_integrator :: InvariantDetector`
- `crate :: invariant_reporter`
- `crate :: julia_parser :: JuliaAnalyzer`
- `crate :: report :: ReportGenerator`
- `crate :: rust_parser :: RustAnalyzer`
- `crate :: types :: { AnalysisResult , FileOrderingResult }`
- `std :: path :: { Path , PathBuf }`
- `walkdir :: WalkDir`

## src/080_invariant_reporter.rs (080_invariant_reporter.rs)

Module `080_invariant_reporter`

- `crate :: invariant_types :: *`
- `crate :: refactor_constraints :: RefactorConstraint`
- `serde_json`
- `std :: collections :: HashMap`
- `std :: fs`
- `std :: path :: Path`
- `super :: *`

## src/082_conscience_graph.rs (082_conscience_graph.rs)

Module `082_conscience_graph`

- `crate :: invariant_types :: *`
- `std :: collections :: HashMap`
- `std :: io :: Write`
- `std :: path :: Path`
- `super :: *`

## src/083_action_validator.rs (083_action_validator.rs)

Module `083_action_validator`

- `crate :: invariant_types :: InvariantStrength`
- `crate :: refactor_constraints :: RefactorConstraint`
- `serde :: { Deserialize , Serialize }`
- `std :: path :: PathBuf`
- `super :: *`

## src/085_agent_conscience.rs (085_agent_conscience.rs)

Module `085_agent_conscience`

- `crate :: action_validator :: { validate_action , AgentAction , ConstraintViolation }`
- `crate :: invariant_types :: { Invariant , InvariantStrength }`
- `crate :: invariant_types :: { InvariantKind , SemanticInvariant }`
- `crate :: invariant_types :: { InvariantKind , StructuralInvariant }`
- `crate :: refactor_constraints :: { from_invariant , RefactorConstraint }`
- `serde :: { Deserialize , Serialize }`
- `std :: path :: { Path , PathBuf }`
- `super :: *`

## src/090_utilities.rs (090_utilities.rs)

Module `090_utilities`

- `crate :: report :: { PlanItem , Priority , ActionKind , ClusterPlan }`
- `crate :: types :: { DirectoryAnalysis , FunctionPlacement , PlacementStatus }`
- `std :: collections :: { BTreeSet , HashMap }`
- `std :: path :: { Path , PathBuf }`

## src/100_types.rs (100_types.rs)

Module `100_types`

- `crate :: invariant_types :: InvariantAnalysisResult`
- `crate :: refactor_constraints :: RefactorConstraint`
- `serde :: { Deserialize , Serialize }`
- `std :: collections :: HashMap`
- `std :: path :: PathBuf`

## src/110_cohesion_analyzer.rs (110_cohesion_analyzer.rs)

Module `110_cohesion_analyzer`

- `anyhow :: Result`
- `crate :: cluster_006 :: compute_cohesion_score`
- `crate :: cluster_008 :: { detect_layer_violation , FunctionInfo }`
- `crate :: types :: { AnalysisResult , ElementType }`
- `crate :: types :: { CallAnalysis , FunctionCluster , FunctionPlacement , PlacementStatus }`
- `std :: collections :: { BTreeMap , HashMap , HashSet }`
- `std :: path :: PathBuf`

## src/120_directory_analyzer.rs (120_directory_analyzer.rs)

Module `120_directory_analyzer`

- `anyhow :: Result`
- `crate :: dependency :: detect_layer`
- `crate :: layer_utilities :: allow_analysis_dir`
- `crate :: types :: DirectoryAnalysis`
- `std :: fs`
- `std :: path :: { Path , PathBuf }`

## src/130_control_flow.rs (130_control_flow.rs)

Module `130_control_flow`

- `crate :: types :: *`
- `crate :: utilities :: compress_path`
- `petgraph :: dot :: Dot`
- `petgraph :: graph :: { DiGraph , NodeIndex }`
- `std :: collections :: HashMap`

## src/140_file_ordering.rs (140_file_ordering.rs)

Module `140_file_ordering`

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

## src/150_julia_parser.rs (150_julia_parser.rs)

Module `150_julia_parser`

- `anyhow :: { anyhow , Context , Result }`
- `crate :: types :: *`
- `once_cell :: sync :: Lazy`
- `regex :: Regex`
- `std :: env`
- `std :: fs`
- `std :: path :: { Path , PathBuf }`
- `std :: process :: { Command , Stdio }`
- `std :: sync :: atomic :: { AtomicBool , Ordering }`

## src/160_rust_parser.rs (160_rust_parser.rs)

Module `160_rust_parser`

- `anyhow :: { Context , Result }`
- `crate :: types :: { AnalysisResult , CfgEdge , CfgNode , CodeElement , ElementType , FunctionCfg , Language , ModuleInfo , NodeType , Visibility , }`
- `std :: fs`
- `std :: path :: { Path , PathBuf }`
- `syn :: visit :: Visit`
- `syn :: { ItemEnum , ItemFn , ItemImpl , ItemMod , ItemStruct , ItemTrait , ItemUse }`

## src/180_report.rs (180_report.rs)

Module `180_report`

- `anyhow :: Result`
- `crate :: action_validator :: check_move_allowed`
- `crate :: cluster_008 :: collect_cluster_plans`
- `crate :: control_flow :: ControlFlowAnalyzer`
- `crate :: dependency :: { LayerGraph , build_directory_entry_map , build_file_dependency_graph , collect_naming_warnings }`
- `crate :: file_ordering :: DirectoryMove`
- `crate :: layer_core :: { collect_directory_moves , sort_structural_items }`
- `crate :: refactor_constraints :: RefactorConstraint`
- `crate :: types :: *`
- `crate :: utilities :: { compress_path , collect_move_items , write_structural_batches , write_cluster_batches }`
- `std :: cmp :: Ordering`
- `std :: collections :: { BTreeMap , BTreeSet , HashMap , HashSet }`
- `std :: fs`
- `std :: path :: { Path , PathBuf }`
- `walkdir :: WalkDir`

## src/190_main.rs (190_main.rs)

Module `190_main`

- `anyhow :: Result`

## src/191_agent_cli.rs (191_agent_cli.rs)

Module `191_agent_cli`

- `anyhow :: Result`
- `clap :: Parser`
- `crate :: action_validator :: AgentAction`
- `crate :: agent_conscience :: AgentConscience`
- `crate :: invariant_types :: Invariant`
- `std :: path :: PathBuf`
- `super :: *`

