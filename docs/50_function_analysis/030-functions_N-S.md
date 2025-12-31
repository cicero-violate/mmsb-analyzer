# Functions N-S

## Layer: 000_cluster_001.rs

### Rust Functions

#### `naming_score_for_file`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `to_string_lossy`
  - `file_name`
  - `to_string_lossy`
  - `file_stem`
  - `len`
  - `len`
  - `any`
  - `chars`
  - `is_uppercase`
  - `all`
  - `chars`
  - `is_ascii_lowercase`
  - `is_ascii_digit`
  - `contains`
  - `as_str`
  - `as_ref`
  - `fs::read_to_string`
  - `HashMap::new`
  - `Regex::new`
  - `captures_iter`
  - `get`
  - `to_lowercase`
  - `as_str`
  - `or_insert`
  - `entry`
  - `collect`
  - `into_iter`
  - `sort_by`
  - `cmp`
  - `collect`
  - `map`
  - `take`
  - `into_iter`
  - `collect`
  - `filter`
  - `map`
  - `split`
  - `to_lowercase`
  - `is_empty`
  - `all`
  - `chars`
  - `is_ascii_digit`
  - `count`
  - `filter`
  - `iter`
  - `any`
  - `iter`
  - `Some`

#### `order_rust_files_by_dependency`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `crate::cluster_010::build_module_root_map`
  - `rust_entry_paths`
  - `HashMap::new`
  - `BTreeSet::new`
  - `BTreeMap::new`
  - `Vec::new`
  - `detect_layer`
  - `insert`
  - `clone`
  - `insert`
  - `clone`
  - `clone`
  - `with_context`
  - `collect_rust_dependencies`
  - `get`
  - `insert`
  - `clone`
  - `insert`
  - `or_default`
  - `entry`
  - `clone`
  - `clone`
  - `clone`
  - `clone`
  - `push`
  - `clone`
  - `clone`
  - `crate::cluster_008::build_result`

#### `ordered_by_name`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `to_vec`
  - `sort`
  - `collect`
  - `filter_map`
  - `into_iter`
  - `copied`
  - `get`

#### `rust_entry_paths`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `crate::layer_utilities::resolve_source_root`
  - `collect`
  - `filter`
  - `map`
  - `iter`
  - `join`
  - `exists`

## Layer: 010_cluster_008.rs

### Rust Functions

#### `node_style`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public

#### `parse_cluster_members`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `collect`
  - `filter_map`
  - `iter`
  - `rsplit_once`
  - `Some`
  - `PathBuf::from`
  - `to_string`

#### `structural_layer_value`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Crate
- **Calls:**
  - `unwrap_or`
  - `and_then`
  - `as_ref`
  - `layer_prefix_value`

## Layer: 020_cluster_010.rs

### Rust Functions

#### `normalize_module_name`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `find`
  - `all`
  - `chars`
  - `is_ascii_digit`
  - `to_string`
  - `to_string`

#### `order_julia_files_by_dependency`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `BTreeSet::new`
  - `BTreeMap::new`
  - `Vec::new`
  - `LayerResolver::build`
  - `crate::cluster_001::julia_entry_paths`
  - `crate::cluster_001::detect_layer`
  - `insert`
  - `clone`
  - `insert`
  - `clone`
  - `clone`
  - `with_context`
  - `collect_julia_dependencies`
  - `is_absolute`
  - `clone`
  - `unwrap_or`
  - `map`
  - `parent`
  - `join`
  - `clone`
  - `exists`
  - `crate::cluster_001::detect_layer`
  - `insert`
  - `clone`
  - `insert`
  - `or_default`
  - `entry`
  - `clone`
  - `clone`
  - `clone`
  - `clone`
  - `push`
  - `clone`
  - `clone`
  - `resolve_module`
  - `insert`
  - `clone`
  - `insert`
  - `or_default`
  - `entry`
  - `clone`
  - `clone`
  - `clone`
  - `clone`
  - `push`
  - `clone`
  - `clone`
  - `crate::cluster_008::build_result`

#### `resolve_module`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Public
- **Calls:**
  - `normalize_module_name`
  - `get`
  - `Some`
  - `clone`
  - `or_else`
  - `or_else`
  - `map`
  - `find`
  - `iter`
  - `clone`
  - `map`
  - `find`
  - `iter`
  - `starts_with`
  - `as_str`
  - `clone`
  - `crate::cluster_011::resolve_path`
  - `PathBuf::from`

#### `resolve_module_name`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Private
- **Calls:**
  - `unwrap_or`
  - `next`
  - `split`
  - `resolve_module`

#### `resolve_source_root`

- **File:** src/020_cluster_010.rs:0
- **Visibility:** Private
- **Calls:**
  - `join`
  - `exists`
  - `is_dir`
  - `to_path_buf`

## Layer: 030_cluster_011.rs

### Rust Functions

#### `resolve_path`

- **File:** src/030_cluster_011.rs:0
- **Visibility:** Public
- **Calls:**
  - `contains`
  - `Some`
  - `to_path_buf`
  - `and_then`
  - `file_stem`
  - `to_str`
  - `crate::cluster_010::normalize_module_name`
  - `get`
  - `Some`
  - `clone`

## Layer: 030_fixpoint_solver.rs

### Rust Functions

#### `propagate_to_fixpoint`

- **File:** src/030_fixpoint_solver.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `node_indices`
  - `insert`
  - `clone`
  - `HashMap::new`
  - `get`
  - `insert`
  - `clone`
  - `clone`
  - `node_indices`
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `neighbors_directed`
  - `get`
  - `merge`
  - `get`
  - `approx_eq`
  - `insert`
  - `HashMap::new`
  - `Vec::new`
  - `clone`
  - `get`
  - `approx_eq`
  - `push`
  - `clone`
  - `insert`
  - `clone`

## Layer: 050_cluster_006.rs

### Rust Functions

#### `order_directories`

- **File:** src/050_cluster_006.rs:0
- **Visibility:** Public
- **Calls:**
  - `common_root`
  - `HashSet::new`
  - `map`
  - `parent`
  - `starts_with`
  - `insert`
  - `clone`
  - `map`
  - `parent`
  - `collect`
  - `into_iter`
  - `sort_by`
  - `crate::cluster_008::compare_path_components`
  - `HashMap::new`
  - `enumerate`
  - `iter`
  - `insert`
  - `clone`
  - `map`
  - `parent`
  - `get`
  - `map`
  - `parent`
  - `get`
  - `insert`
  - `collect`
  - `filter_map`
  - `enumerate`
  - `iter`
  - `Some`
  - `Vec::with_capacity`
  - `len`
  - `next`
  - `iter`
  - `remove`
  - `push`
  - `clone`
  - `clone`
  - `insert`
  - `len`
  - `len`
  - `enumerate`
  - `iter`
  - `push`
  - `clone`

#### `strip_numeric_prefix`

- **File:** src/050_cluster_006.rs:0
- **Visibility:** Crate
- **Calls:**
  - `Lazy::new`
  - `unwrap`
  - `Regex::new`
  - `unwrap_or`
  - `map`
  - `and_then`
  - `captures`
  - `get`
  - `as_str`

## Layer: 060_layer_core.rs

### Rust Functions

#### `sort_structural_items`

- **File:** src/060_layer_core.rs:0
- **Visibility:** Public
- **Calls:**
  - `len`
  - `len`
  - `HashMap::new`
  - `enumerate`
  - `iter`
  - `push`
  - `or_default`
  - `entry`
  - `clone`
  - `structural_layer_value`
  - `structural_layer_value`
  - `Some`
  - `Some`
  - `Some`
  - `Some`
  - `push`
  - `enumerate`
  - `iter`
  - `get`
  - `push`
  - `Vec::with_capacity`
  - `collect`
  - `filter`
  - `is_empty`
  - `sort_by`
  - `structural_cmp`
  - `remove`
  - `push`
  - `saturating_sub`
  - `push`
  - `len`
  - `sort_by`
  - `Vec::with_capacity`
  - `push`
  - `clone`

#### `structural_cmp`

- **File:** src/060_layer_core.rs:0
- **Visibility:** Public
- **Calls:**
  - `structural_layer_value`
  - `structural_layer_value`
  - `structural_layer_value`
  - `structural_layer_value`
  - `saturating_mul`
  - `saturating_mul`
  - `then_with`
  - `then_with`
  - `then_with`
  - `then_with`
  - `then_with`
  - `cmp`
  - `cmp`
  - `cmp`
  - `cmp`
  - `cmp`
  - `cmp`

## Layer: 070_layer_utilities.rs

### Rust Functions

#### `resolve_source_root`

- **File:** src/070_layer_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `join`
  - `exists`
  - `is_dir`
  - `to_path_buf`

#### `run_analysis`

- **File:** src/070_layer_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `join`
  - `RustAnalyzer::new`
  - `to_string`
  - `to_string_lossy`
  - `AnalysisResult::new`
  - `gather_rust_files`
  - `context`
  - `crate::dependency::order_rust_files_by_dependency`
  - `context`
  - `crate::dependency::analyze_file_ordering`
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `analyze_file`
  - `merge`
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `Vec::new`
  - `gather_julia_files`
  - `context`
  - `crate::dependency::order_julia_files_by_dependency`
  - `exists`
  - `JuliaAnalyzer::new`
  - `to_path_buf`
  - `clone`
  - `join`
  - `analyze_file`
  - `merge`
  - `ControlFlowAnalyzer::new`
  - `build_call_graph`
  - `InvariantDetector::new`
  - `detect_all`
  - `InvariantDetector::new`
  - `generate_constraints`
  - `FunctionCohesionAnalyzer::new`
  - `analyze`
  - `detect_clusters`
  - `DirectoryAnalyzer::new`
  - `to_path_buf`
  - `analyze`
  - `ReportGenerator::new`
  - `to_string`
  - `to_string_lossy`
  - `context`
  - `generate_all`
  - `export_program_cfg_to_path`
  - `call_edges`
  - `context`
  - `invariant_reporter::generate_invariant_report`
  - `context`
  - `invariant_reporter::export_constraints_json`
  - `Ok`

## Layer: 082_conscience_graph.rs

### Rust Functions

#### `strength_emoji`

- **File:** src/082_conscience_graph.rs:0
- **Visibility:** Private

## Layer: 090_utilities.rs

### Rust Functions

#### `path_common_prefix_len`

- **File:** src/090_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `zip`
  - `components`
  - `components`

#### `resolve_required_layer_path`

- **File:** src/090_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `Vec::new`
  - `collect_directory_files`
  - `collect`
  - `filter`
  - `into_iter`
  - `unwrap_or`
  - `map`
  - `and_then`
  - `file_name`
  - `to_str`
  - `is_empty`
  - `join`
  - `unwrap_or`
  - `parent`
  - `unwrap_or`
  - `parent`
  - `unwrap_or`
  - `parent`
  - `path_common_prefix_len`
  - `count`
  - `components`
  - `Some`
  - `unwrap_or_else`
  - `join`
  - `unwrap_or`
  - `parent`

## Layer: 110_cohesion_analyzer.rs

### Rust Functions

#### `suggest_cluster_file`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashMap::new`
  - `as_str`
  - `or_insert`
  - `entry`
  - `map`
  - `max_by_key`
  - `into_iter`
  - `PathBuf::from`

#### `suggest_file`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `Some`
  - `clone`
  - `unwrap_or`
  - `map`
  - `find`
  - `iter`

## Layer: 120_directory_analyzer.rs

### Rust Functions

#### `should_skip_path`

- **File:** src/120_directory_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `file_name`

## Layer: 130_control_flow.rs

### Rust Functions

#### `sanitize_identifier`

- **File:** src/130_control_flow.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect`
  - `map`
  - `chars`
  - `is_ascii_alphanumeric`

## Layer: 140_file_ordering.rs

### Rust Functions

#### `parallel_build_file_dag`

- **File:** src/140_file_ordering.rs:0
- **Visibility:** Public
- **Calls:**
  - `collect`
  - `map`
  - `par_iter`
  - `crate::dependency::build_directory_dag`
  - `DiGraph::new`
  - `HashMap::new`
  - `node_indices`
  - `clone`
  - `or_insert_with`
  - `entry`
  - `clone`
  - `add_node`
  - `edge_indices`
  - `edge_endpoints`
  - `clone`
  - `clone`
  - `expect`
  - `get`
  - `expect`
  - `get`
  - `add_edge`
  - `Ok`

## Layer: 150_julia_parser.rs

### Rust Functions

#### `paren_balance`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `chars`

#### `parse_module_name`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `starts_with`
  - `map`
  - `nth`
  - `split_whitespace`
  - `to_string`
  - `trim_end_matches`
  - `starts_with`
  - `map`
  - `nth`
  - `split_whitespace`
  - `to_string`
  - `trim_end_matches`

#### `parse_struct_name`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `starts_with`
  - `starts_with`
  - `starts_with`
  - `collect`
  - `split_whitespace`
  - `map`
  - `get`
  - `to_string`
  - `trim_end_matches`
  - `unwrap_or`
  - `next`
  - `split`

#### `relativize_path`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `strip_prefix`
  - `to_string`
  - `to_string_lossy`
  - `to_string`
  - `to_string_lossy`

#### `resolve_julia_binary`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `env::var`
  - `PathBuf::from`
  - `exists`
  - `env::var`
  - `join`
  - `Path::new`
  - `fs::read_dir`
  - `flatten`
  - `path`
  - `is_dir`
  - `and_then`
  - `file_name`
  - `to_str`
  - `starts_with`
  - `join`
  - `exists`
  - `PathBuf::from`
  - `exists`
  - `PathBuf::from`

#### `slugify_relative`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `unwrap_or`
  - `strip_prefix`
  - `join`
  - `collect`
  - `map`
  - `components`
  - `replace`
  - `to_string_lossy`
  - `as_os_str`

## Layer: 160_rust_parser.rs

### Rust Functions

#### `pat_snippet`

- **File:** src/160_rust_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `truncate_label`
  - `to_string`

#### `relativize_path`

- **File:** src/160_rust_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `strip_prefix`
  - `to_string`
  - `to_string_lossy`
  - `to_string`
  - `to_string_lossy`

## Layer: 180_report.rs

### Rust Functions

#### `normalize_use_stmt`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `replace`
  - `join`
  - `collect`
  - `split_whitespace`
  - `find`
  - `truncate`
  - `to_string`
  - `trim`
  - `starts_with`
  - `find`
  - `to_string`
  - `trim`
  - `strip_prefix`
  - `to_string`
  - `trim`

#### `parse_dead_code_warnings`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashMap::new`
  - `peekable`
  - `lines`
  - `next`
  - `trim`
  - `starts_with`
  - `find`
  - `len`
  - `find`
  - `contains`
  - `peek`
  - `trim`
  - `find`
  - `find`
  - `Some`
  - `PathBuf::from`
  - `extend`
  - `or_insert_with`
  - `entry`
  - `to_string`

#### `parse_use_symbols`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `Vec::new`
  - `find`
  - `trim`
  - `find`
  - `trim`
  - `unwrap_or`
  - `strip_prefix`
  - `unwrap_or`
  - `strip_prefix`
  - `find`
  - `unwrap_or`
  - `rfind`
  - `len`
  - `split`
  - `trim`
  - `is_empty`
  - `trim`
  - `unwrap_or`
  - `next`
  - `split`
  - `unwrap_or`
  - `next`
  - `rsplit`
  - `is_empty`
  - `push`
  - `to_string`
  - `trim`
  - `unwrap_or`
  - `next`
  - `rsplit`
  - `is_empty`
  - `push`
  - `to_string`

#### `path_matches`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `ends_with`
  - `ends_with`

#### `placement_status_label`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `to_string`
  - `to_string`
  - `to_string`

#### `placement_status_notes`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `String::new`

#### `prefix_key_from_path`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `unwrap_or`
  - `strip_prefix`
  - `is_empty`
  - `to_string`
  - `collect`
  - `split`
  - `len`
  - `to_string`
  - `len`
  - `to_string`

#### `referenced_elsewhere`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `get`
  - `any`
  - `iter`
  - `path_matches`

#### `render_mermaid_graph`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `String::from`
  - `HashMap::new`
  - `node_indices`
  - `unwrap_or`
  - `and_then`
  - `file_name`
  - `to_str`
  - `insert`
  - `index`
  - `clone`
  - `push_str`
  - `edge_indices`
  - `edge_endpoints`
  - `get`
  - `index`
  - `get`
  - `index`
  - `push_str`
  - `push_str`

#### `sanitize_mermaid_id`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect`
  - `map`
  - `chars`
  - `is_ascii_alphanumeric`

#### `sanitize_mermaid_label`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `replace`
  - `replace`

#### `scan_crate_paths`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `Vec::new`
  - `find`
  - `len`
  - `chars`
  - `is_ascii_alphanumeric`
  - `len_utf8`
  - `next`
  - `rsplit`
  - `is_empty`
  - `push`
  - `to_string`

#### `short_signature`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `join`
  - `collect`
  - `split_whitespace`
  - `len`
  - `collect`
  - `take`
  - `chars`
  - `push_str`

#### `slugify_file_path`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_lowercase`
  - `replace`
  - `replace`
  - `trim_start_matches`

#### `slugify_key`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect`
  - `map`
  - `chars`
  - `is_ascii_alphanumeric`
  - `to_ascii_lowercase`

#### `slugify_path`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `String::new`
  - `components`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `replace`
  - `to_string_lossy`
  - `as_os_str`
  - `is_empty`
  - `to_string`

#### `sort_cluster_items`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `sort_by`
  - `then_with`
  - `then_with`
  - `unwrap_or`
  - `partial_cmp`
  - `cmp`
  - `cmp`

#### `sort_plan_items`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `sort_by`
  - `then_with`
  - `cmp`
  - `cmp`

## Layer: 191_agent_cli.rs

### Rust Functions

#### `query_function`

- **File:** src/191_agent_cli.rs:0
- **Visibility:** Private
- **Calls:**
  - `load_invariants`
  - `AgentConscience::new`
  - `query_allowed_actions`
  - `serde_json::to_string_pretty`
  - `Ok`

#### `run_agent_cli`

- **File:** src/191_agent_cli.rs:0
- **Visibility:** Public
- **Calls:**
  - `AgentCli::parse`
  - `check_action`
  - `query_function`
  - `list_invariants`
  - `show_stats`
  - `Ok`

#### `show_stats`

- **File:** src/191_agent_cli.rs:0
- **Visibility:** Private
- **Calls:**
  - `load_invariants`
  - `AgentConscience::new`
  - `stats`
  - `Ok`

