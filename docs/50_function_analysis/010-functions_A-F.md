# Functions A-F

## Layer: 000_cluster_001.rs

### Rust Functions

#### `analyze_file_ordering`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `unwrap_or`
  - `collect`
  - `cloned`
  - `iter`
  - `crate::cluster_011::build_module_map`
  - `crate::cluster_010::build_dependency_map`
  - `build_file_layers`
  - `crate::layer_core::order_directories`
  - `crate::cluster_011::build_file_dag`
  - `crate::cluster_008::detect_layer_violations`
  - `detect_cycles`
  - `is_empty`
  - `unwrap_or_else`
  - `crate::layer_core::layer_constrained_sort`
  - `unwrap_or_else`
  - `topological_sort`
  - `ordered_by_name`
  - `ordered_by_name`
  - `collect`
  - `map`
  - `into_iter`
  - `clone`
  - `build_entries`
  - `detect_violations`
  - `Ok`

#### `build_directory_entry_map`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `is_empty`
  - `Ok`
  - `HashMap::new`
  - `collect`
  - `cloned`
  - `iter`
  - `crate::cluster_011::build_module_map`
  - `build_dependency_map`
  - `build_file_layers`
  - `build_file_dag`
  - `detect_cycles`
  - `is_empty`
  - `unwrap_or_else`
  - `layer_constrained_sort`
  - `unwrap_or_else`
  - `topological_sort`
  - `ordered_by_name`
  - `ordered_by_name`
  - `collect`
  - `map`
  - `into_iter`
  - `clone`
  - `build_entries`
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `HashMap::new`
  - `insert`
  - `clone`
  - `Ok`

#### `build_entries`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `collect`
  - `map`
  - `enumerate`
  - `iter`
  - `crate::cluster_006::generate_canonical_name`
  - `unwrap_or`
  - `map`
  - `and_then`
  - `file_name`
  - `to_str`
  - `clone`

#### `build_file_layers`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `insert`
  - `clone`
  - `detect_layer`

#### `collect_julia_dependencies`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Crate
- **Calls:**
  - `with_context`
  - `fs::read_to_string`
  - `Vec::new`
  - `captures_iter`
  - `get`
  - `PathBuf::from`
  - `as_str`
  - `unwrap_or_default`
  - `map`
  - `get`
  - `to_string`
  - `trim`
  - `as_str`
  - `push`
  - `JuliaTarget::Include`
  - `captures_iter`
  - `get`
  - `as_str`
  - `to_string`
  - `unwrap_or`
  - `next`
  - `split`
  - `unwrap_or_default`
  - `map`
  - `get`
  - `to_string`
  - `trim`
  - `as_str`
  - `push`
  - `JuliaTarget::Module`
  - `captures_iter`
  - `get`
  - `unwrap_or_default`
  - `map`
  - `get`
  - `to_string`
  - `trim`
  - `as_str`
  - `filter`
  - `map`
  - `split`
  - `as_str`
  - `trim`
  - `is_empty`
  - `to_string`
  - `unwrap_or`
  - `next`
  - `split`
  - `push`
  - `JuliaTarget::Module`
  - `clone`
  - `captures_iter`
  - `get`
  - `as_str`
  - `unwrap_or_default`
  - `map`
  - `get`
  - `to_string`
  - `trim`
  - `as_str`
  - `push`
  - `JuliaTarget::Module`
  - `to_string`
  - `Ok`

#### `collect_naming_warnings`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `any`
  - `components`
  - `as_os_str`
  - `Ok`
  - `build_directory_entry_map`
  - `any`
  - `components`
  - `as_os_str`
  - `get`
  - `naming_score_for_file`
  - `unwrap_or`
  - `map`
  - `as_str`
  - `push`
  - `collect_naming_warnings`
  - `Ok`

#### `collect_roots_from_crate`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `collect_roots_from_crate`
  - `insert`
  - `collect_roots_from_crate`
  - `collect_roots_from_crate`
  - `insert`
  - `to_string`
  - `insert`
  - `to_string`

#### `collect_rust_dependencies`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Private
- **Calls:**
  - `with_context`
  - `fs::read_to_string`
  - `with_context`
  - `syn::parse_file`
  - `UseCollector::default`
  - `visit_file`
  - `Ok`

#### `detect_cycles`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Crate
- **Calls:**
  - `tarjan_scc`
  - `Vec::new`
  - `len`
  - `push`
  - `collect`
  - `map`
  - `into_iter`
  - `clone`
  - `is_empty`
  - `all`
  - `iter`
  - `is_empty`
  - `to_vec`
  - `sort`
  - `push`

#### `detect_layer`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `components`
  - `to_str`
  - `as_os_str`
  - `next`
  - `chars`
  - `is_ascii_digit`
  - `find`
  - `all`
  - `chars`
  - `is_ascii_digit`
  - `to_string`
  - `to_string`

#### `detect_violations`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Crate
- **Calls:**
  - `to_vec`
  - `sort_by`
  - `cmp`
  - `collect`
  - `map`
  - `enumerate`
  - `iter`
  - `clone`
  - `collect`
  - `map`
  - `enumerate`
  - `iter`
  - `clone`
  - `Vec::new`
  - `get`
  - `get`
  - `unwrap_or_default`
  - `map`
  - `get`
  - `collect`
  - `cloned`
  - `filter`
  - `iter`
  - `unwrap_or`
  - `copied`
  - `get`
  - `push`
  - `clone`

#### `detects_cycles`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Private
- **Calls:**
  - `temp_dir`
  - `create_dir_all`
  - `join`
  - `join`
  - `write`
  - `write`
  - `analyze_file_ordering`
  - `clone`
  - `clone`
  - `Some`
  - `Ok`

#### `escape_dot`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Private
- **Calls:**
  - `replace`
  - `replace`
  - `replace`

#### `export_complete_program_dot`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `replace`
  - `replace`
  - `replace`
  - `String::new`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `collect`
  - `iter`
  - `sort_by_key`
  - `as_str`
  - `HashMap::new`
  - `enumerate`
  - `iter`
  - `replace`
  - `crate::cluster_008::cyclomatic_complexity`
  - `insert`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `crate::cluster_008::node_style`
  - `clone`
  - `is_empty`
  - `join`
  - `collect`
  - `map`
  - `iter`
  - `to_string`
  - `unwrap`
  - `unwrap`
  - `Vec::new`
  - `push`
  - `push`
  - `is_empty`
  - `to_string`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `unwrap`
  - `get`
  - `get`
  - `get`
  - `get`
  - `unwrap`
  - `unwrap`
  - `std::fs::write`
  - `Ok`

## Layer: 005_refactor_constraints.rs

### Rust Functions

#### `check_move_allowed`

- **File:** src/005_refactor_constraints.rs:0
- **Visibility:** Public
- **Calls:**
  - `target`
  - `is_blocking`
  - `Err`
  - `Err`
  - `Ok`

#### `from_invariant`

- **File:** src/005_refactor_constraints.rs:0
- **Visibility:** Public
- **Calls:**
  - `Some`
  - `clone`
  - `Some`
  - `clone`
  - `Some`
  - `clone`
  - `to_string`
  - `Some`
  - `clone`
  - `Some`
  - `clone`
  - `to_string`
  - `Some`
  - `clone`
  - `clone`
  - `Some`
  - `clone`
  - `to_string`
  - `Some`
  - `clone`
  - `to_string`
  - `Some`
  - `clone`
  - `Vec::new`
  - `Some`
  - `clone`
  - `collect`
  - `cloned`
  - `iter`

## Layer: 010_cluster_008.rs

### Rust Functions

#### `adjacency_from_edges`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashMap::new`
  - `insert`
  - `or_default`
  - `entry`
  - `clone`
  - `clone`

#### `build_result`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `adjacency_from_edges`
  - `topo_sort`
  - `position`
  - `iter`
  - `remove`
  - `insert`
  - `layer_rank_map`
  - `to_vec`
  - `sort_by`
  - `is_mmsb_main`
  - `is_mmsb_main`
  - `contains`
  - `contains`
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `to_string`
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `to_string`
  - `unwrap_or`
  - `cloned`
  - `get`
  - `len`
  - `unwrap_or`
  - `cloned`
  - `get`
  - `len`
  - `then_with`
  - `then_with`
  - `cmp`
  - `cmp`
  - `cmp`
  - `collect`
  - `map`
  - `into_iter`
  - `is_layer_violation`
  - `collect`
  - `into_iter`
  - `Ok`

#### `cluster_target_path`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `is_core_module_path`
  - `unwrap_or`
  - `and_then`
  - `and_then`
  - `file_stem`
  - `to_str`
  - `layer_prefix_value`
  - `unwrap_or`
  - `and_then`
  - `first`
  - `parent`
  - `join`

#### `collect_cluster_plans`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `Vec::new`
  - `enumerate`
  - `iter`
  - `parse_cluster_members`
  - `clone`
  - `first`
  - `join`
  - `unwrap_or`
  - `parent`
  - `join`
  - `cluster_target_path`
  - `collect`
  - `filter`
  - `into_iter`
  - `len`
  - `push`
  - `sort_by`
  - `then_with`
  - `then_with`
  - `unwrap_or`
  - `partial_cmp`
  - `cmp`
  - `len`
  - `len`
  - `cmp`

#### `compare_dir_layers`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `unwrap_or`
  - `and_then`
  - `file_name`
  - `to_str`
  - `unwrap_or`
  - `and_then`
  - `file_name`
  - `to_str`
  - `unwrap_or`
  - `layer_prefix_value`
  - `unwrap_or`
  - `layer_prefix_value`
  - `then_with`
  - `cmp`
  - `cmp`

#### `compare_path_components`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `collect`
  - `components`
  - `collect`
  - `components`
  - `min`
  - `len`
  - `len`
  - `to_string_lossy`
  - `as_os_str`
  - `to_string_lossy`
  - `as_os_str`
  - `layer_prefix_value`
  - `layer_prefix_value`
  - `cmp`
  - `cmp`
  - `cmp`
  - `len`
  - `len`

#### `cyclomatic_complexity`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `len`
  - `len`

#### `detect_layer_violation`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `clone`
  - `layer_prefix_value`
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `clone`
  - `layer_prefix_value`
  - `Some`
  - `map`

#### `detect_layer_violations`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `Vec::new`
  - `edge_references`
  - `source`
  - `target`
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `to_string`
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `to_string`
  - `layer_prefix_value`
  - `layer_prefix_value`
  - `push`
  - `clone`
  - `clone`

## Layer: 020_cluster_010.rs

### Rust Functions

#### `build_dependency_map`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `with_context`
  - `extract_dependencies`
  - `insert`
  - `clone`
  - `Ok`

#### `build_module_root_map`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `join`
  - `HashMap::new`
  - `is_dir`
  - `fs::read_dir`
  - `path`
  - `contains_tools`
  - `to_string`
  - `trim_end_matches`
  - `to_string`
  - `to_string_lossy`
  - `file_name`
  - `is_dir`
  - `normalize_module_name`
  - `insert`
  - `clone`
  - `unwrap_or`
  - `map`
  - `extension`
  - `insert`
  - `clone`
  - `crate::cluster_001::detect_layer`
  - `Ok`

#### `contains_tools`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `any`
  - `components`
  - `as_os_str`

#### `extract_dependencies`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Crate
- **Calls:**
  - `unwrap_or`
  - `and_then`
  - `extension`
  - `to_str`
  - `extract_rust_dependencies`
  - `extract_julia_dependencies`
  - `Ok`
  - `Vec::new`

#### `extract_julia_dependencies`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `Lazy::new`
  - `unwrap`
  - `Regex::new`
  - `Lazy::new`
  - `unwrap`
  - `Regex::new`
  - `Lazy::new`
  - `unwrap`
  - `Regex::new`
  - `Lazy::new`
  - `unwrap`
  - `Regex::new`
  - `Lazy::new`
  - `unwrap`
  - `Regex::new`
  - `unwrap_or`
  - `next`
  - `split`
  - `resolve_module`
  - `with_context`
  - `fs::read_to_string`
  - `Vec::new`
  - `captures_iter`
  - `get`
  - `as_str`
  - `PathBuf::from`
  - `is_none`
  - `extension`
  - `set_extension`
  - `is_absolute`
  - `unwrap_or`
  - `map`
  - `parent`
  - `join`
  - `crate::cluster_011::resolve_path`
  - `push`
  - `captures_iter`
  - `get`
  - `resolve_module_name`
  - `as_str`
  - `push`
  - `captures_iter`
  - `get`
  - `filter`
  - `map`
  - `split`
  - `as_str`
  - `trim`
  - `is_empty`
  - `resolve_module_name`
  - `push`
  - `captures_iter`
  - `get`
  - `resolve_module_name`
  - `as_str`
  - `push`
  - `captures_iter`
  - `get`
  - `as_str`
  - `starts_with`
  - `resolve_module_name`
  - `push`
  - `Ok`

#### `extract_rust_dependencies`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `crate::dependency::collect_roots`
  - `is_none`
  - `insert`
  - `to_string`
  - `with_context`
  - `fs::read_to_string`
  - `with_context`
  - `syn::parse_file`
  - `UseCollector::default`
  - `visit_file`
  - `Vec::new`
  - `resolve_module`
  - `push`
  - `resolve_module`
  - `push`
  - `Ok`

## Layer: 020_layer_inference.rs

### Rust Functions

#### `detect_layer_violations`

- **File:** src/020_layer_inference.rs:0
- **Visibility:** Public
- **Calls:**
  - `Vec::new`
  - `edge_references`
  - `source`
  - `target`
  - `get`
  - `get`
  - `push`
  - `clone`
  - `clone`

## Layer: 030_cluster_011.rs

### Rust Functions

#### `build_directory_dag`

- **File:** src/030_cluster_011.rs:0
- **Visibility:** Public
- **Calls:**
  - `collect`
  - `map`
  - `filter`
  - `filter_map`
  - `into_iter`
  - `walkdir::WalkDir::new`
  - `ok`
  - `unwrap_or`
  - `map`
  - `and_then`
  - `extension`
  - `path`
  - `to_str`
  - `into_path`
  - `collect`
  - `cloned`
  - `iter`
  - `build_module_map`
  - `crate::cluster_010::build_dependency_map`
  - `build_file_dag`
  - `Ok`

#### `build_file_dag`

- **File:** src/030_cluster_011.rs:0
- **Visibility:** Crate
- **Calls:**
  - `DiGraph::new`
  - `HashMap::new`
  - `add_node`
  - `clone`
  - `insert`
  - `clone`
  - `get`
  - `get`
  - `add_edge`

#### `build_file_dependency_graph`

- **File:** src/030_cluster_011.rs:0
- **Visibility:** Public
- **Calls:**
  - `collect`
  - `cloned`
  - `iter`
  - `build_module_map`
  - `crate::cluster_010::build_dependency_map`
  - `build_file_dag`
  - `Ok`

#### `build_module_map`

- **File:** src/030_cluster_011.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `and_then`
  - `file_stem`
  - `to_str`
  - `crate::cluster_010::normalize_module_name`
  - `insert`
  - `clone`
  - `clone`
  - `and_then`
  - `parent`
  - `file_name`
  - `to_str`
  - `insert`
  - `crate::cluster_010::normalize_module_name`
  - `clone`

#### `export_program_cfg_to_path`

- **File:** src/030_cluster_011.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `Vec::new`
  - `insert`
  - `clone`
  - `clone`
  - `to_string`
  - `unwrap_or`
  - `last`
  - `split`
  - `to_string`
  - `unwrap_or`
  - `last`
  - `split`
  - `contains_key`
  - `contains_key`
  - `push`
  - `join`
  - `std::fs::create_dir_all`
  - `join`
  - `crate::cluster_001::export_complete_program_dot`
  - `as_ref`
  - `to_string_lossy`
  - `join`
  - `to_str`
  - `to_str`
  - `status`
  - `args`
  - `std::process::Command::new`
  - `Ok`

## Layer: 040_dependency.rs

### Rust Functions

#### `collect_roots`

- **File:** src/040_dependency.rs:0
- **Visibility:** Public
- **Calls:**
  - `to_string`
  - `collect_roots`
  - `insert`
  - `insert`
  - `collect_roots`
  - `insert`
  - `to_string`
  - `insert`
  - `to_string`

## Layer: 050_cluster_006.rs

### Rust Functions

#### `collect_directory_moves`

- **File:** src/050_cluster_006.rs:0
- **Visibility:** Public
- **Calls:**
  - `Vec::new`
  - `BTreeMap::new`
  - `join`
  - `parent`
  - `push`
  - `or_default`
  - `entry`
  - `to_path_buf`
  - `clone`
  - `sort_by`
  - `crate::cluster_008::compare_dir_layers`
  - `enumerate`
  - `iter`
  - `and_then`
  - `file_name`
  - `to_str`
  - `strip_numeric_prefix`
  - `join`
  - `push`
  - `clone`

#### `common_root`

- **File:** src/050_cluster_006.rs:0
- **Visibility:** Public
- **Calls:**
  - `iter`
  - `collect`
  - `components`
  - `next`
  - `len`
  - `collect`
  - `components`
  - `len`
  - `PathBuf::new`
  - `take`
  - `into_iter`
  - `push`
  - `as_os_str`
  - `Some`

#### `compute_cohesion_score`

- **File:** src/050_cluster_006.rs:0
- **Visibility:** Public
- **Calls:**
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `clone`
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `clone`
  - `crate::cluster_008::layer_adheres`
  - `clamp`

## Layer: 070_layer_utilities.rs

### Rust Functions

#### `allow_analysis_dir`

- **File:** src/070_layer_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `unwrap_or`
  - `and_then`
  - `file_name`
  - `to_str`
  - `starts_with`
  - `strip_prefix`
  - `any`
  - `components`
  - `unwrap_or`
  - `to_str`
  - `as_os_str`
  - `starts_with`

## Layer: 080_invariant_reporter.rs

### Rust Functions

#### `export_constraints_json`

- **File:** src/080_invariant_reporter.rs:0
- **Visibility:** Public
- **Calls:**
  - `join`
  - `fs::create_dir_all`
  - `join`
  - `map_err`
  - `serde_json::to_string_pretty`
  - `std::io::Error::new`
  - `fs::write`
  - `Ok`

#### `export_json`

- **File:** src/080_invariant_reporter.rs:0
- **Visibility:** Public
- **Calls:**
  - `join`
  - `map_err`
  - `serde_json::to_string_pretty`
  - `std::io::Error::new`
  - `fs::write`
  - `Ok`

## Layer: 083_action_validator.rs

### Rust Functions

#### `check_move_allowed`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Public
- **Calls:**
  - `to_string`
  - `clone`
  - `clone`
  - `validate_action`
  - `Ok`
  - `collect`
  - `map`
  - `iter`
  - `clone`
  - `Err`
  - `join`

#### `extract_layer`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Private
- **Calls:**
  - `and_then`
  - `and_then`
  - `file_name`
  - `to_str`
  - `collect`
  - `split`
  - `is_empty`
  - `ok`
  - `parse`

## Layer: 090_utilities.rs

### Rust Functions

#### `collect_directory_files`

- **File:** src/090_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `extend`
  - `cloned`
  - `iter`
  - `collect_directory_files`

#### `collect_move_items`

- **File:** src/090_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `Vec::new`
  - `compute_move_metrics`
  - `unwrap_or_else`
  - `map`
  - `as_ref`
  - `compress_path`
  - `as_ref`
  - `to_string_lossy`
  - `to_string`
  - `push`
  - `String::new`
  - `contains`
  - `Some`
  - `clone`
  - `clone`
  - `Some`
  - `clone`
  - `resolve_required_layer_path`
  - `compress_path`
  - `as_ref`
  - `to_string_lossy`
  - `compute_move_metrics`
  - `push`
  - `String::new`
  - `Some`
  - `clone`
  - `Some`
  - `clone`
  - `contains`
  - `Some`
  - `clone`
  - `Some`
  - `Some`
  - `clone`

#### `compress_path`

- **File:** src/090_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `find`
  - `starts_with`
  - `to_string`
  - `rfind`
  - `to_string`

#### `compute_move_metrics`

- **File:** src/090_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `sum`
  - `map`
  - `iter`
  - `len`
  - `BTreeSet::new`
  - `insert`
  - `clone`
  - `Vec::new`
  - `insert`
  - `clone`
  - `push`
  - `clone`
  - `Vec::new`
  - `insert`
  - `clone`
  - `push`
  - `clone`
  - `max`
  - `len`

## Layer: 110_cohesion_analyzer.rs

### Rust Functions

#### `build_call_analysis`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `BTreeMap::new`
  - `or_insert`
  - `entry`
  - `PathBuf::from`
  - `BTreeMap::new`
  - `or_insert`
  - `entry`
  - `PathBuf::from`
  - `compute_type_coupling`
  - `collect`
  - `into_iter`
  - `collect`
  - `into_iter`

#### `build_call_edges`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect_functions`
  - `build_name_map`
  - `enumerate`
  - `iter`
  - `get`
  - `or_insert`
  - `entry`
  - `or_insert`
  - `entry`

#### `build_function_layers`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashMap::new`
  - `or_insert_with`
  - `entry`
  - `clone`
  - `clone`

#### `build_name_map`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashMap::new`
  - `enumerate`
  - `iter`
  - `push`
  - `or_default`
  - `entry`
  - `clone`

#### `build_type_maps`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashMap::new`
  - `HashSet::new`
  - `clone`
  - `insert`
  - `clone`
  - `insert`
  - `or_default`
  - `entry`
  - `clone`

#### `build_undirected_graph`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `len`
  - `HashMap::new`
  - `enumerate`
  - `iter`
  - `or_insert`
  - `entry`
  - `push`
  - `push`

#### `collect_functions`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect`
  - `map`
  - `filter`
  - `iter`
  - `clone`
  - `clone`
  - `clone`
  - `clone`
  - `clone`

#### `compute_cluster_cohesion`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect`
  - `copied`
  - `iter`
  - `get`
  - `contains`

#### `compute_type_coupling`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `extract_identifiers`
  - `get`
  - `contains`
  - `contains`

#### `determine_status`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `sum`
  - `map`
  - `iter`
  - `to_string`

#### `extract_identifiers`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `Vec::new`
  - `String::new`
  - `chars`
  - `is_ascii_alphanumeric`
  - `push`
  - `is_empty`
  - `push`
  - `clone`
  - `clear`
  - `is_empty`
  - `push`

## Layer: 150_julia_parser.rs

### Rust Functions

#### `extract_calls_from_lines`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `join`
  - `extract_calls_from_text`

#### `extract_calls_from_text`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `Vec::new`
  - `captures_iter`
  - `get`
  - `as_str`
  - `is_reserved`
  - `push`
  - `to_string`
  - `sort`
  - `dedup`

#### `find_julia_project_dir`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `parent`
  - `exists`
  - `join`
  - `to_path_buf`
  - `parent`
  - `to_path_buf`
  - `unwrap_or_else`
  - `parent`
  - `Path::new`

## Layer: 160_rust_parser.rs

### Rust Functions

#### `expr_snippet`

- **File:** src/160_rust_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `truncate_label`
  - `to_string`

## Layer: 180_report.rs

### Rust Functions

#### `baseline_deltas`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `Vec::new`
  - `get`
  - `push`
  - `get`
  - `push`
  - `get`
  - `push`
  - `get`
  - `push`
  - `get`
  - `push`

#### `cluster_priority`

- **File:** src/180_report.rs:0
- **Visibility:** Private

#### `collect_cluster_items`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect`
  - `map`
  - `iter`
  - `cluster_priority`
  - `Vec::new`
  - `Some`
  - `clone`
  - `Vec::new`
  - `len`

#### `collect_directories`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Generics:** 'a
- **Calls:**
  - `push`
  - `collect_directories`

#### `collect_rename_items`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `BTreeSet::new`
  - `insert`
  - `clone`
  - `collect`
  - `map`
  - `filter`
  - `iter`
  - `clone`
  - `unwrap_or_else`
  - `map`
  - `parent`
  - `join`
  - `PathBuf::from`
  - `contains`
  - `Vec::new`
  - `Some`
  - `clone`
  - `Some`
  - `clone`
  - `Vec::new`

#### `collect_size_warnings`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `len`
  - `push`
  - `fs::read_to_string`
  - `count`
  - `lines`
  - `push`
  - `collect_size_warnings`

#### `collect_symbol_references`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashMap::new`
  - `join`
  - `filter_map`
  - `into_iter`
  - `WalkDir::new`
  - `ok`
  - `path`
  - `is_file`
  - `and_then`
  - `extension`
  - `to_str`
  - `Some`
  - `fs::read_to_string`
  - `lines`
  - `contains`
  - `parse_use_symbols`
  - `insert`
  - `or_insert_with`
  - `entry`
  - `to_path_buf`
  - `contains`
  - `scan_crate_paths`
  - `insert`
  - `or_insert_with`
  - `entry`
  - `to_path_buf`

#### `collect_utility_candidates`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `BTreeSet::new`
  - `len`
  - `insert`
  - `collect`
  - `into_iter`

#### `compute_directory_cohesion`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `map`
  - `parent`
  - `to_path_buf`
  - `unwrap_or`
  - `and_then`
  - `as_ref`
  - `map`
  - `parent`

#### `compute_ordering_correctness`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `len`
  - `saturating_sub`
  - `len`
  - `len`

#### `directory_moves_to_plan`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect`
  - `map`
  - `into_iter`
  - `Vec::new`
  - `Some`
  - `clone`
  - `Some`
  - `clone`
  - `Vec::new`

#### `display_path`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `unwrap_or`
  - `strip_prefix`
  - `to_string`
  - `to_string_lossy`

#### `filter_orphaned`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Generics:** 'a
- **Calls:**
  - `collect_symbol_references`
  - `unwrap_or_default`
  - `map`
  - `as_deref`
  - `load_cargo_warnings`
  - `Vec::new`
  - `Vec::new`
  - `filter`
  - `iter`
  - `is_entrypoint_main`
  - `is_public_function`
  - `referenced_elsewhere`
  - `is_dead_code_candidate`
  - `push`
  - `push`

#### `function_bucket_label`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `unwrap_or`
  - `map`
  - `find`
  - `chars`
  - `is_ascii_alphabetic`
  - `to_ascii_uppercase`

## Layer: 191_agent_cli.rs

### Rust Functions

#### `check_action`

- **File:** src/191_agent_cli.rs:0
- **Visibility:** Private
- **Calls:**
  - `std::fs::read_to_string`
  - `serde_json::from_str`
  - `load_invariants`
  - `AgentConscience::new`
  - `check_action`
  - `serde_json::to_string_pretty`
  - `std::process::exit`

