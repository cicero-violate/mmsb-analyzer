# Structure Group: src/070_layer_utilities.rs

## File: src/070_layer_utilities.rs

- Layer(s): 070_layer_utilities.rs
- Language coverage: Rust (6)
- Element types: Function (5), Struct (1)
- Total elements: 6

### Elements

- [Rust | Struct] `Args` (line 0, priv)
  - Signature: `# [derive (Parser , Debug)] # [command (name = "mmsb-analyzer")] # [command (about = "MMSB Intelligence Substrate Ana...`
- [Rust | Function] `allow_analysis_dir` (line 0, pub)
  - Signature: `# [doc = " Checks if a directory should be included in analysis"] pub fn allow_analysis_dir (root : & Path , dir : & ...`
  - Calls: unwrap_or, and_then, file_name, to_str, starts_with, strip_prefix, any, components, unwrap_or, to_str, as_os_str, starts_with
- [Rust | Function] `gather_rust_files` (line 0, pub)
  - Signature: `pub fn gather_rust_files (root : & Path) -> Vec < PathBuf > { use walkdir :: WalkDir ; let src_root = resolve_source_...`
  - Calls: resolve_source_root, collect, map, filter, filter, filter_map, filter_entry, into_iter, WalkDir::new, depth, is_dir, file_type, allow_analysis_dir, path, ok, map_or, extension, path, unwrap_or, strip_prefix, path, path, count, components, starts_with, path, join, into_path
- [Rust | Function] `main` (line 0, pub)
  - Signature: `pub fn main () -> Result < () > { let args = Args :: parse () ; let root_path = std :: env :: current_dir () ? . join...`
  - Calls: Args::parse, canonicalize, join, std::env::current_dir, unwrap_or_else, canonicalize, join, std::env::current_dir, join, unwrap, std::env::current_dir, ok, std::fs::create_dir_all, unwrap_or, canonicalize, run_analysis
- [Rust | Function] `resolve_source_root` (line 0, pub)
  - Signature: `# [doc = " Resolves the source root directory from a given root path"] pub fn resolve_source_root (root : & Path) -> ...`
  - Calls: join, exists, is_dir, to_path_buf
- [Rust | Function] `run_analysis` (line 0, pub)
  - Signature: `pub fn run_analysis (root_path : & Path , output_path : & Path , verbose : bool , skip_julia : bool ,) -> Result < ()...`
  - Calls: join, RustAnalyzer::new, to_string, to_string_lossy, AnalysisResult::new, gather_rust_files, context, crate::dependency::order_rust_files_by_dependency, context, crate::dependency::analyze_file_ordering, Vec::new, Vec::new, Vec::new, Vec::new, Vec::new, analyze_file, merge, Vec::new, Vec::new, Vec::new, Vec::new, gather_julia_files, context, crate::dependency::order_julia_files_by_dependency, exists, JuliaAnalyzer::new, to_path_buf, clone, join, analyze_file, merge, ControlFlowAnalyzer::new, build_call_graph, InvariantDetector::new, detect_all, InvariantDetector::new, generate_constraints, FunctionCohesionAnalyzer::new, analyze, detect_clusters, DirectoryAnalyzer::new, to_path_buf, analyze, ReportGenerator::new, to_string, to_string_lossy, context, generate_all, export_program_cfg_to_path, call_edges, context, invariant_reporter::generate_invariant_report, context, invariant_reporter::export_constraints_json, Ok

