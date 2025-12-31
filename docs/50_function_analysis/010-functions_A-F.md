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

## Layer: 050_cluster_010.rs

### Rust Functions

#### `build_dependency_map`

- **File:** src/050_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `with_context`
  - `extract_dependencies`
  - `insert`
  - `clone`
  - `Ok`

#### `build_module_root_map`

- **File:** src/050_cluster_010.rs:0
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

- **File:** src/050_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `any`
  - `components`
  - `as_os_str`

#### `extract_dependencies`

- **File:** src/050_cluster_010.rs:0
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

- **File:** src/050_cluster_010.rs:0
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

- **File:** src/050_cluster_010.rs:0
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

## Layer: 070_cluster_011.rs

### Rust Functions

#### `build_directory_dag`

- **File:** src/070_cluster_011.rs:0
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

- **File:** src/070_cluster_011.rs:0
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

- **File:** src/070_cluster_011.rs:0
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

- **File:** src/070_cluster_011.rs:0
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

- **File:** src/070_cluster_011.rs:0
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

## Layer: 090_dependency.rs

### Rust Functions

#### `collect_roots`

- **File:** src/090_dependency.rs:0
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

## Layer: 110_cluster_006.rs

### Rust Functions

#### `collect_directory_moves`

- **File:** src/110_cluster_006.rs:0
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

- **File:** src/110_cluster_006.rs:0
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

- **File:** src/110_cluster_006.rs:0
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

## Layer: 160_layer_utilities.rs

### Rust Functions

#### `allow_analysis_dir`

- **File:** src/160_layer_utilities.rs:0
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

## Layer: 170_invariant_reporter.rs

### Rust Functions

#### `export_constraints_json`

- **File:** src/170_invariant_reporter.rs:0
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

- **File:** src/170_invariant_reporter.rs:0
- **Visibility:** Public
- **Calls:**
  - `join`
  - `map_err`
  - `serde_json::to_string_pretty`
  - `std::io::Error::new`
  - `fs::write`
  - `Ok`

## Layer: 190_action_validator.rs

### Rust Functions

#### `extract_layer`

- **File:** src/190_action_validator.rs:0
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

## Layer: 210_utilities.rs

### Rust Functions

#### `collect_directory_files`

- **File:** src/210_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `extend`
  - `cloned`
  - `iter`
  - `collect_directory_files`

#### `collect_move_items`

- **File:** src/210_utilities.rs:0
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

- **File:** src/210_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `find`
  - `starts_with`
  - `to_string`
  - `rfind`
  - `to_string`

#### `compute_move_metrics`

- **File:** src/210_utilities.rs:0
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

## Layer: 211_dead_code_attribute_parser.rs

### Rust Functions

#### `collect_latent_attrs`

- **File:** src/211_dead_code_attribute_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `Vec::new`
  - `is_ident`
  - `path`
  - `parse_nested_meta`
  - `is_ident`
  - `is_ident`
  - `is_ident`
  - `is_ident`
  - `is_ident`
  - `is_ident`
  - `value`
  - `parse`
  - `Some`
  - `value`
  - `is_ident`
  - `value`
  - `parse`
  - `marker_from_str`
  - `value`
  - `Ok`
  - `parse_args`
  - `Some`
  - `value`
  - `push`

#### `detect_intent_signals`

- **File:** src/211_dead_code_attribute_parser.rs:0
- **Visibility:** Public
- **Calls:**
  - `parse_mmsb_latent_attr`
  - `scan_doc_comments`
  - `merge_doc_intent`
  - `planned_directory_intent`
  - `merge_intent_sources`

#### `detect_test_modules`

- **File:** src/211_dead_code_attribute_parser.rs:0
- **Visibility:** Public
- **Calls:**
  - `unwrap_or_default`
  - `std::fs::read_to_string`
  - `syn::parse_file`
  - `HashSet::new`
  - `HashSet::new`
  - `is_cfg_test_item`
  - `insert`
  - `to_string`

#### `detect_test_symbols`

- **File:** src/211_dead_code_attribute_parser.rs:0
- **Visibility:** Public
- **Calls:**
  - `unwrap_or_default`
  - `std::fs::read_to_string`
  - `syn::parse_file`
  - `HashSet::new`
  - `HashSet::new`
  - `has_test_attr`
  - `insert`
  - `to_string`
  - `is_cfg_test_item`
  - `insert`
  - `to_string`
  - `insert`
  - `to_string`

#### `extract_attribute_value`

- **File:** src/211_dead_code_attribute_parser.rs:0
- **Visibility:** Public
- **Calls:**
  - `parse_nested_meta`
  - `is_ident`
  - `value`
  - `parse`
  - `Some`
  - `value`
  - `Ok`

## Layer: 330_agent_cli.rs

### Rust Functions

#### `check_action`

- **File:** src/330_agent_cli.rs:0
- **Visibility:** Private
- **Calls:**
  - `std::fs::read_to_string`
  - `serde_json::from_str`
  - `load_invariants`
  - `AgentConscience::new`
  - `check_action`
  - `serde_json::to_string_pretty`
  - `std::process::exit`

## Layer: 370_dead_code_doc_comment_parser.rs

### Rust Functions

#### `detect_latent_markers`

- **File:** src/370_dead_code_doc_comment_parser.rs:0
- **Visibility:** Public
- **Calls:**
  - `IntentMarker::from_comment`

#### `extract_doc_markers`

- **File:** src/370_dead_code_doc_comment_parser.rs:0
- **Visibility:** Crate
- **Calls:**
  - `Vec::new`
  - `is_ident`
  - `path`
  - `detect_latent_markers`
  - `value`
  - `push`

## Layer: 380_dead_code_call_graph.rs

### Rust Functions

#### `build_call_graph`

- **File:** src/380_dead_code_call_graph.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `or_default`
  - `entry`
  - `clone`
  - `extend`
  - `cloned`
  - `iter`

#### `build_reverse_call_graph`

- **File:** src/380_dead_code_call_graph.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `push`
  - `or_default`
  - `entry`
  - `clone`
  - `clone`

#### `classify_symbol`

- **File:** src/380_dead_code_call_graph.rs:0
- **Visibility:** Public
- **Calls:**
  - `contains_key`
  - `is_test_only`
  - `is_reachable`

#### `compute_reachability`

- **File:** src/380_dead_code_call_graph.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashSet::new`
  - `collect`
  - `cloned`
  - `iter`
  - `pop_front`
  - `insert`
  - `clone`
  - `get`
  - `contains`
  - `push_back`
  - `clone`

## Layer: 390_dead_code_intent.rs

### Rust Functions

#### `check_planned_directory`

- **File:** src/390_dead_code_intent.rs:0
- **Visibility:** Public
- **Calls:**
  - `starts_with`

#### `collect_symbols`

- **File:** src/390_dead_code_intent.rs:0
- **Visibility:** Crate
- **Calls:**
  - `unwrap_or_default`
  - `std::fs::read_to_string`
  - `syn::parse_file`
  - `Vec::new`
  - `collect`
  - `filter_map`
  - `iter`

## Layer: 400_dead_code_test_boundaries.rs

### Rust Functions

#### `find_test_callers`

- **File:** src/400_dead_code_test_boundaries.rs:0
- **Visibility:** Public
- **Calls:**
  - `is_empty`
  - `Vec::new`
  - `build_reverse_call_graph`
  - `Vec::new`
  - `HashSet::new`
  - `collect`
  - `into_iter`
  - `unwrap_or_default`
  - `cloned`
  - `get`
  - `pop_front`
  - `insert`
  - `clone`
  - `contains`
  - `push`
  - `clone`
  - `get`
  - `contains`
  - `push_back`
  - `clone`

## Layer: 410_dead_code_entrypoints.rs

### Rust Functions

#### `collect_entrypoints`

- **File:** src/410_dead_code_entrypoints.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashSet::new`
  - `insert`
  - `clone`
  - `insert`
  - `clone`
  - `treat_public_as_entrypoint`
  - `insert`
  - `clone`

#### `collect_exports`

- **File:** src/410_dead_code_entrypoints.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashSet::new`
  - `join`
  - `filter_map`
  - `into_iter`
  - `WalkDir::new`
  - `path`
  - `is_file`
  - `and_then`
  - `extension`
  - `to_str`
  - `Some`
  - `unwrap_or_default`
  - `std::fs::read_to_string`
  - `syn::parse_file`
  - `collect_use_tree_idents`
  - `insert`
  - `to_string`

#### `collect_use_tree_idents`

- **File:** src/410_dead_code_entrypoints.rs:0
- **Visibility:** Private
- **Calls:**
  - `insert`
  - `to_string`
  - `insert`
  - `to_string`
  - `collect_use_tree_idents`
  - `collect_use_tree_idents`
  - `insert`
  - `to_string`

## Layer: 430_dead_code_confidence.rs

### Rust Functions

#### `assign_confidence`

- **File:** src/430_dead_code_confidence.rs:0
- **Visibility:** Public

## Layer: 460_dead_code_report.rs

### Rust Functions

#### `build_basic_report`

- **File:** src/460_dead_code_report.rs:0
- **Visibility:** Public
- **Calls:**
  - `DeadCodeSummary::from_items`

#### `build_report`

- **File:** src/460_dead_code_report.rs:0
- **Visibility:** Public
- **Calls:**
  - `DeadCodeSummary::from_items`

## Layer: 470_dead_code_filter.rs

### Rust Functions

#### `collect_excluded_symbols`

- **File:** src/470_dead_code_filter.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect`
  - `map`
  - `filter`
  - `iter`
  - `should_exclude_from_analysis`
  - `clone`

#### `filter_dead_code_elements`

- **File:** src/470_dead_code_filter.rs:0
- **Visibility:** Public
- **Calls:**
  - `collect_excluded_symbols`
  - `collect`
  - `cloned`
  - `filter`
  - `iter`
  - `contains`

## Layer: 520_violation_predictor.rs

### Rust Functions

#### `find_callers`

- **File:** src/520_violation_predictor.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashSet::new`
  - `get`
  - `find_element_file`
  - `insert`
  - `collect`
  - `into_iter`

#### `find_element_file`

- **File:** src/520_violation_predictor.rs:0
- **Visibility:** Private
- **Calls:**
  - `map`
  - `find`
  - `iter`
  - `PathBuf::from`

#### `find_reference_files`

- **File:** src/520_violation_predictor.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashSet::new`
  - `any`
  - `iter`
  - `find_element_file`
  - `insert`
  - `collect`
  - `into_iter`

## Layer: 540_tier_classifier.rs

### Rust Functions

#### `classify_tier`

- **File:** src/540_tier_classifier.rs:0
- **Visibility:** Public

## Layer: 550_confidence_scorer.rs

### Rust Functions

#### `compute_confidence`

- **File:** src/550_confidence_scorer.rs:0
- **Visibility:** Public
- **Calls:**
  - `min`

## Layer: 560_correction_plan_generator.rs

### Rust Functions

#### `action_function`

- **File:** src/560_correction_plan_generator.rs:0
- **Visibility:** Private
- **Calls:**
  - `Some`
  - `clone`

#### `action_module_path`

- **File:** src/560_correction_plan_generator.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `display`
  - `to_string`
  - `display`
  - `to_string`
  - `display`
  - `to_string`
  - `display`
  - `to_string`

#### `action_refs`

- **File:** src/560_correction_plan_generator.rs:0
- **Visibility:** Private
- **Calls:**
  - `Some`
  - `clone`
  - `clone`
  - `Some`
  - `to_string`
  - `display`
  - `to_string`
  - `display`

#### `action_symbol`

- **File:** src/560_correction_plan_generator.rs:0
- **Visibility:** Private
- **Calls:**
  - `Some`
  - `clone`
  - `Some`
  - `clone`
  - `Some`
  - `clone`

#### `action_target_layer`

- **File:** src/560_correction_plan_generator.rs:0
- **Visibility:** Private
- **Calls:**
  - `clone`

#### `action_visibility`

- **File:** src/560_correction_plan_generator.rs:0
- **Visibility:** Private
- **Calls:**
  - `Some`
  - `clone`
  - `clone`
  - `clone`
  - `clone`
  - `clone`

#### `average_confidence`

- **File:** src/560_correction_plan_generator.rs:0
- **Visibility:** Private
- **Calls:**
  - `is_empty`
  - `sum`
  - `map`
  - `iter`
  - `len`

#### `estimate_fix_time`

- **File:** src/560_correction_plan_generator.rs:0
- **Visibility:** Private

## Layer: 570_verification_scope_planner.rs

### Rust Functions

#### `action_module`

- **File:** src/570_verification_scope_planner.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `display`
  - `to_string`
  - `display`
  - `to_string`
  - `display`
  - `to_string`
  - `display`
  - `to_string`
  - `display`

#### `affected_files`

- **File:** src/570_verification_scope_planner.rs:0
- **Visibility:** Private

#### `estimate_verification_time`

- **File:** src/570_verification_scope_planner.rs:0
- **Visibility:** Private

## Layer: 580_rollback_criteria_builder.rs

### Rust Functions

#### `build_rollback_criteria`

- **File:** src/580_rollback_criteria_builder.rs:0
- **Visibility:** Public
- **Calls:**
  - `push`
  - `push`
  - `push`
  - `extract_critical_tests`
  - `push`
  - `clone`

#### `extract_critical_tests`

- **File:** src/580_rollback_criteria_builder.rs:0
- **Visibility:** Private
- **Calls:**
  - `Vec::new`

## Layer: 590_quality_delta_calculator.rs

### Rust Functions

#### `calculate_quality_delta`

- **File:** src/590_quality_delta_calculator.rs:0
- **Visibility:** Public
- **Calls:**
  - `to_string`
  - `to_string`
  - `to_string`
  - `action_id`

#### `estimate_impact`

- **File:** src/590_quality_delta_calculator.rs:0
- **Visibility:** Public
- **Calls:**
  - `simulate_action`
  - `calculate_quality_delta`

## Layer: 620_verification_policy_emitter.rs

### Rust Functions

#### `emit_verification_policy`

- **File:** src/620_verification_policy_emitter.rs:0
- **Visibility:** Public
- **Calls:**
  - `std::fs::write`
  - `serde_json::to_string_pretty`
  - `Ok`

## Layer: 630_correction_intelligence_report.rs

### Rust Functions

#### `augment_path_coherence_strategies`

- **File:** src/630_correction_intelligence_report.rs:0
- **Visibility:** Crate
- **Calls:**
  - `module_name_from_path`
  - `module_name_from_path`
  - `unwrap_or`
  - `and_then`
  - `file_name`
  - `to_str`
  - `unwrap_or`
  - `and_then`
  - `file_name`
  - `to_str`
  - `ok`
  - `Regex::new`
  - `ok`
  - `Regex::new`
  - `is_empty`
  - `is_empty`
  - `ok`
  - `Regex::new`
  - `Vec::new`
  - `HashSet::new`
  - `crate::cluster_010::gather_rust_files`
  - `fs::read_to_string`
  - `lines`
  - `is_match`
  - `replace`
  - `clone`
  - `to_string`
  - `clone`
  - `insert`
  - `clone`
  - `push`
  - `is_match`
  - `replace`
  - `clone`
  - `to_string`
  - `clone`
  - `insert`
  - `clone`
  - `push`
  - `is_match`
  - `contains`
  - `replace`
  - `clone`
  - `to_string`
  - `clone`
  - `insert`
  - `clone`
  - `push`
  - `sort_by`
  - `then_with`
  - `then_with`
  - `cmp`
  - `cmp`
  - `cmp`
  - `push`

#### `build_state`

- **File:** src/630_correction_intelligence_report.rs:0
- **Visibility:** Public
- **Generics:** 'a
- **Calls:**
  - `to_path_buf`

#### `compute_summary`

- **File:** src/630_correction_intelligence_report.rs:0
- **Visibility:** Crate
- **Calls:**
  - `len`
  - `is_empty`
  - `len`

#### `default_confidence`

- **File:** src/630_correction_intelligence_report.rs:0
- **Visibility:** Private

#### `fill_prediction_confidence`

- **File:** src/630_correction_intelligence_report.rs:0
- **Visibility:** Crate
- **Calls:**
  - `default_confidence`

#### `filter_path_coherence_report`

- **File:** src/630_correction_intelligence_report.rs:0
- **Visibility:** Public
- **Calls:**
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `enumerate`
  - `iter`
  - `trim_start`
  - `starts_with`
  - `starts_with`
  - `starts_with`
  - `starts_with`
  - `starts_with`
  - `push`
  - `clone`
  - `get`
  - `push`
  - `clone`
  - `get`
  - `push`
  - `clone`
  - `get`
  - `push`
  - `clone`
  - `compute_summary`
  - `clone`
  - `clone`
  - `clone`
  - `len`

#### `filter_visibility_report`

- **File:** src/630_correction_intelligence_report.rs:0
- **Visibility:** Public
- **Calls:**
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `enumerate`
  - `iter`
  - `starts_with`
  - `push`
  - `clone`
  - `get`
  - `push`
  - `clone`
  - `get`
  - `push`
  - `clone`
  - `get`
  - `push`
  - `clone`
  - `compute_summary`
  - `clone`
  - `clone`
  - `clone`
  - `len`

