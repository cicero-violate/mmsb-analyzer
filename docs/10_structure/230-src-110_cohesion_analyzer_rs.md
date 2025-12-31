# Structure Group: src/110_cohesion_analyzer.rs

## File: src/110_cohesion_analyzer.rs

- Layer(s): 110_cohesion_analyzer.rs
- Language coverage: Rust (16)
- Element types: Function (14), Impl (1), Struct (1)
- Total elements: 16

### Elements

- [Rust | Struct] `FunctionCohesionAnalyzer` (line 0, pub)
  - Signature: `# [derive (Default)] pub struct FunctionCohesionAnalyzer { threshold : f64 , }`
- [Rust | Function] `build_call_analysis` (line 0, priv)
  - Signature: `fn build_call_analysis (func : & FunctionInfo , functions : & [FunctionInfo] , outgoing : & HashMap < usize , usize >...`
  - Calls: BTreeMap::new, or_insert, entry, PathBuf::from, BTreeMap::new, or_insert, entry, PathBuf::from, compute_type_coupling, collect, into_iter, collect, into_iter
- [Rust | Function] `build_call_edges` (line 0, priv)
  - Signature: `fn build_call_edges (result : & AnalysisResult ,) -> (Vec < FunctionInfo > , Vec < HashMap < usize , usize > > , Vec ...`
  - Calls: collect_functions, build_name_map, enumerate, iter, get, or_insert, entry, or_insert, entry
- [Rust | Function] `build_function_layers` (line 0, priv)
  - Signature: `fn build_function_layers (functions : & [FunctionInfo]) -> HashMap < String , String > { let mut map = HashMap :: new...`
  - Calls: HashMap::new, or_insert_with, entry, clone, clone
- [Rust | Function] `build_name_map` (line 0, priv)
  - Signature: `fn build_name_map (functions : & [FunctionInfo]) -> HashMap < String , Vec < usize > > { let mut map : HashMap < Stri...`
  - Calls: HashMap::new, enumerate, iter, push, or_default, entry, clone
- [Rust | Function] `build_type_maps` (line 0, priv)
  - Signature: `fn build_type_maps (result : & AnalysisResult ,) -> (HashMap < String , HashSet < String > > , HashSet < String >) { ...`
  - Calls: HashMap::new, HashSet::new, clone, insert, clone, insert, or_default, entry, clone
- [Rust | Function] `build_undirected_graph` (line 0, priv)
  - Signature: `fn build_undirected_graph (outgoing_counts : & [HashMap < usize , usize >] ,) -> (Vec < Vec < (usize , usize) > > , V...`
  - Calls: len, HashMap::new, enumerate, iter, or_insert, entry, push, push
- [Rust | Function] `collect_functions` (line 0, priv)
  - Signature: `fn collect_functions (result : & AnalysisResult) -> Vec < FunctionInfo > { result . elements . iter () . filter (| el...`
  - Calls: collect, map, filter, iter, clone, clone, clone, clone, clone
- [Rust | Function] `compute_cluster_cohesion` (line 0, priv)
  - Signature: `fn compute_cluster_cohesion (members : & [usize] , outgoing_counts : & [HashMap < usize , usize >] ,) -> f64 { let me...`
  - Calls: collect, copied, iter, get, contains
- [Rust | Function] `compute_type_coupling` (line 0, priv)
  - Signature: `fn compute_type_coupling (func : & FunctionInfo , file_types : & HashMap < String , HashSet < String > > , all_types ...`
  - Calls: extract_identifiers, get, contains, contains
- [Rust | Function] `determine_status` (line 0, priv)
  - Signature: `fn determine_status (call_analysis : & CallAnalysis , cohesion_score : f64 , threshold : f64 , layer_violation : Opti...`
  - Calls: sum, map, iter, to_string
- [Rust | Function] `extract_identifiers` (line 0, priv)
  - Signature: `fn extract_identifiers (text : & str) -> Vec < String > { let mut tokens = Vec :: new () ; let mut current = String :...`
  - Calls: Vec::new, String::new, chars, is_ascii_alphanumeric, push, is_empty, push, clone, clear, is_empty, push
- [Rust | Impl] `impl FunctionCohesionAnalyzer { pub fn new () -> Self { Self { threshold : DEFAULT_THRESHOLD , } } # [allow (dead_code)] pub fn with_threshold (threshold : f64) -> Self { Self { threshold } } pub fn analyze (& self , result : & AnalysisResult) -> Result < Vec < FunctionPlacement > > { let (functions , outgoing_counts , incoming_counts) = build_call_edges (result) ; let file_layers = build_function_layers (& functions) ; let (file_types , all_types) = build_type_maps (result) ; let mut placements = Vec :: new () ; for (idx , func) in functions . iter () . enumerate () { let call_analysis = build_call_analysis (func , & functions , & outgoing_counts [idx] , & incoming_counts [idx] , & file_types , & all_types ,) ; let cohesion_score = compute_cohesion_score (func , & functions , & outgoing_counts [idx] , & file_layers , & call_analysis ,) ; let layer_violation = detect_layer_violation (func , & functions , & outgoing_counts [idx] , & file_layers) ; let placement_status = determine_status (& call_analysis , cohesion_score , self . threshold , layer_violation) ; let suggested_file = match & placement_status { PlacementStatus :: ShouldMove { .. } => suggest_file (& call_analysis) , PlacementStatus :: Orphaned { .. } => None , PlacementStatus :: LayerViolation { .. } => None , PlacementStatus :: Correct => None , } ; placements . push (FunctionPlacement { name : func . name . clone () , signature : func . signature . clone () , current_file : PathBuf :: from (& func . file_path) , suggested_file , placement_status , cohesion_score , call_analysis , }) ; } Ok (placements) } pub fn detect_clusters (& self , result : & AnalysisResult) -> Result < Vec < FunctionCluster > > { let (functions , outgoing_counts , _) = build_call_edges (result) ; let assignments = louvain_communities (& outgoing_counts) ; let mut clusters_map : HashMap < usize , Vec < usize > > = HashMap :: new () ; for (idx , comm) in assignments . into_iter () . enumerate () { clusters_map . entry (comm) . or_default () . push (idx) ; } let mut clusters = Vec :: new () ; for component in clusters_map . values () { if component . len () < 3 { continue ; } let cohesion = compute_cluster_cohesion (component , & outgoing_counts) ; let suggested_file = suggest_cluster_file (component , & functions) ; let members = component . iter () . map (| idx | { format ! ("{}::{}" , functions [* idx] . file_path , functions [* idx] . name) }) . collect () ; clusters . push (FunctionCluster { members , cohesion , suggested_file , }) ; } Ok (clusters) } } . self_ty` (line 0, priv)
- [Rust | Function] `louvain_communities` (line 0, priv)
  - Signature: `fn louvain_communities (outgoing_counts : & [HashMap < usize , usize >]) -> Vec < usize > { let n = outgoing_counts ....`
  - Calls: len, Vec::new, build_undirected_graph, collect, collect, clone, HashMap::new, or_insert, entry, saturating_sub
- [Rust | Function] `suggest_cluster_file` (line 0, priv)
  - Signature: `fn suggest_cluster_file (members : & [usize] , functions : & [FunctionInfo] ,) -> Option < PathBuf > { let mut counts...`
  - Calls: HashMap::new, as_str, or_insert, entry, map, max_by_key, into_iter, PathBuf::from
- [Rust | Function] `suggest_file` (line 0, priv)
  - Signature: `fn suggest_file (call_analysis : & CallAnalysis) -> Option < PathBuf > { let mut best_file : Option < PathBuf > = Non...`
  - Calls: Some, clone, unwrap_or, map, find, iter

