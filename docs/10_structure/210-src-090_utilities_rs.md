# Structure Group: src/090_utilities.rs

## File: src/090_utilities.rs

- Layer(s): 090_utilities.rs
- Language coverage: Rust (8)
- Element types: Function (8)
- Total elements: 8

### Elements

- [Rust | Function] `collect_directory_files` (line 0, pub)
  - Signature: `pub fn collect_directory_files (directory : & DirectoryAnalysis , out : & mut Vec < PathBuf >) { out . extend (direct...`
  - Calls: extend, cloned, iter, collect_directory_files
- [Rust | Function] `collect_move_items` (line 0, pub)
  - Signature: `pub fn collect_move_items (placements : & [FunctionPlacement] , utility_names : & BTreeSet < String > , directory : &...`
  - Calls: Vec::new, compute_move_metrics, unwrap_or_else, map, as_ref, compress_path, as_ref, to_string_lossy, to_string, push, String::new, contains, Some, clone, clone, Some, clone, resolve_required_layer_path, compress_path, as_ref, to_string_lossy, compute_move_metrics, push, String::new, Some, clone, Some, clone, contains, Some, clone, Some, Some, clone
- [Rust | Function] `compress_path` (line 0, pub)
  - Signature: `# [doc = " Compress absolute paths to MMSB-relative format"] pub fn compress_path (path : & str) -> String { if let S...`
  - Calls: find, starts_with, to_string, rfind, to_string
- [Rust | Function] `compute_move_metrics` (line 0, pub)
  - Signature: `pub fn compute_move_metrics (placement : & FunctionPlacement ,) -> (usize , usize , usize , usize , Vec < PathBuf > ,...`
  - Calls: sum, map, iter, len, BTreeSet::new, insert, clone, Vec::new, insert, clone, push, clone, Vec::new, insert, clone, push, clone, max, len
- [Rust | Function] `path_common_prefix_len` (line 0, pub)
  - Signature: `pub fn path_common_prefix_len (a : & Path , b : & Path) -> isize { let mut count = 0isize ; for (a_comp , b_comp) in ...`
  - Calls: zip, components, components
- [Rust | Function] `resolve_required_layer_path` (line 0, pub)
  - Signature: `pub fn resolve_required_layer_path (required_layer : & str , current_file : & Path , directory : & DirectoryAnalysis ...`
  - Calls: Vec::new, collect_directory_files, collect, filter, into_iter, unwrap_or, map, and_then, file_name, to_str, is_empty, join, unwrap_or, parent, unwrap_or, parent, unwrap_or, parent, path_common_prefix_len, count, components, Some, unwrap_or_else, join, unwrap_or, parent
- [Rust | Function] `write_cluster_batches` (line 0, pub)
  - Signature: `pub fn write_cluster_batches (content : & mut String , plans : & [ClusterPlan] , root_path : & Path) { if plans . is_...`
  - Calls: is_empty, push_str, push_str, push_str, enumerate, iter, push_str, push_str, push_str, Vec::new, exists, push_str, push, push_str, compress_path, as_ref, to_string_lossy, push_str, push, push, push_str, is_empty, push_str, push_str, push_str, push
- [Rust | Function] `write_structural_batches` (line 0, pub)
  - Signature: `pub fn write_structural_batches (content : & mut String , items : & [PlanItem]) { if items . is_empty () { return ; }...`
  - Calls: is_empty, Vec::new, HashMap::new, or_default, entry, clone, is_empty, push, clone, push, push_str, push_str, push_str, enumerate, iter, Vec::new, unwrap_or, get, push_str, push_str, push_str, Vec::new, exists, compress_path, as_ref, to_string_lossy, push_str, push, unwrap_or, as_deref, unwrap_or_else, map, as_ref, compress_path, as_ref, to_string_lossy, to_string, to_string, push_str, push, clone, sort, dedup, is_empty, push_str, push_str, push, push_str, is_empty, push_str, push_str, push_str, push

