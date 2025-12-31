# Functions G-M

## Layer: 000_cluster_001.rs

### Rust Functions

#### `gather_julia_files`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `crate::layer_utilities::resolve_source_root`
  - `collect`
  - `map`
  - `filter`
  - `filter`
  - `filter_map`
  - `filter_entry`
  - `into_iter`
  - `WalkDir::new`
  - `depth`
  - `is_dir`
  - `file_type`
  - `crate::layer_utilities::allow_analysis_dir`
  - `path`
  - `ok`
  - `map_or`
  - `extension`
  - `path`
  - `unwrap_or`
  - `strip_prefix`
  - `path`
  - `path`
  - `count`
  - `components`
  - `starts_with`
  - `path`
  - `join`
  - `into_path`

#### `generates_canonical_names_and_violations`

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

#### `julia_entry_paths`

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

#### `layer_constrained_sort`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `BTreeMap::new`
  - `node_indices`
  - `unwrap_or_else`
  - `cloned`
  - `get`
  - `to_string`
  - `unwrap_or`
  - `layer_prefix_value`
  - `push`
  - `or_default`
  - `entry`
  - `Vec::new`
  - `topo_sort_within`
  - `extend`
  - `Ok`

## Layer: 005_refactor_constraints.rs

### Rust Functions

#### `generate_constraints`

- **File:** src/005_refactor_constraints.rs:0
- **Visibility:** Public
- **Calls:**
  - `collect`
  - `filter_map`
  - `iter`

## Layer: 010_cluster_008.rs

### Rust Functions

#### `insert_sorted`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Private
- **Calls:**
  - `len`
  - `insert`
  - `clone`
  - `push_back`

#### `is_core_module_path`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `and_then`
  - `file_stem`
  - `to_str`
  - `starts_with`
  - `starts_with`

#### `is_layer_violation`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `layer_prefix_value`
  - `layer_prefix_value`

#### `is_mmsb_main`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Private
- **Calls:**
  - `unwrap_or`
  - `map`
  - `and_then`
  - `file_name`
  - `to_str`

#### `layer_adheres`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Public
- **Calls:**
  - `layer_prefix_value`
  - `layer_prefix_value`

#### `layer_prefix_value`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Private
- **Calls:**
  - `chars`
  - `String::new`
  - `next`
  - `is_ascii_digit`
  - `push`
  - `is_empty`
  - `ok`
  - `parse`

#### `layer_rank_map`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashMap::new`
  - `enumerate`
  - `iter`
  - `insert`
  - `clone`

## Layer: 020_layer_inference.rs

### Rust Functions

#### `infer_layers`

- **File:** src/020_layer_inference.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `HashMap::new`
  - `node_indices`
  - `count`
  - `neighbors_directed`
  - `insert`
  - `node_indices`
  - `contains_key`
  - `collect`
  - `neighbors_directed`
  - `all`
  - `iter`
  - `contains_key`
  - `is_empty`
  - `unwrap_or`
  - `copied`
  - `max`
  - `filter_map`
  - `iter`
  - `get`
  - `insert`
  - `unwrap_or`
  - `copied`
  - `max`
  - `values`
  - `node_indices`
  - `contains_key`
  - `insert`
  - `clone`
  - `collect`
  - `map`
  - `neighbors_directed`
  - `clone`
  - `is_empty`
  - `max`
  - `filter_map`
  - `iter`
  - `copied`
  - `and_then`
  - `find`
  - `node_indices`
  - `get`
  - `insert`
  - `clone`

## Layer: 050_cluster_006.rs

### Rust Functions

#### `generate_canonical_name`

- **File:** src/050_cluster_006.rs:0
- **Visibility:** Public
- **Calls:**
  - `unwrap_or`
  - `and_then`
  - `file_stem`
  - `to_str`
  - `unwrap_or`
  - `and_then`
  - `extension`
  - `to_str`
  - `strip_numeric_prefix`
  - `is_empty`

#### `layer_prefix_value`

- **File:** src/050_cluster_006.rs:0
- **Visibility:** Public
- **Calls:**
  - `chars`
  - `String::new`
  - `next`
  - `is_ascii_digit`
  - `push`
  - `is_empty`
  - `ok`
  - `parse`

## Layer: 050_semantic_detector.rs

### Rust Functions

#### `make_function`

- **File:** src/050_semantic_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `to_string`
  - `to_string`
  - `Vec::new`
  - `to_string`

## Layer: 070_invariant_integrator.rs

### Rust Functions

#### `make_simple_analysis`

- **File:** src/070_invariant_integrator.rs:0
- **Visibility:** Private
- **Calls:**
  - `AnalysisResult::new`
  - `add_element`
  - `to_string`
  - `to_string`
  - `to_string`
  - `Vec::new`
  - `to_string`
  - `Vec::new`
  - `HashMap::new`
  - `insert`
  - `to_string`
  - `to_string`
  - `to_string`
  - `Vec::new`
  - `Vec::new`

## Layer: 070_layer_utilities.rs

### Rust Functions

#### `gather_rust_files`

- **File:** src/070_layer_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `resolve_source_root`
  - `collect`
  - `map`
  - `filter`
  - `filter`
  - `filter_map`
  - `filter_entry`
  - `into_iter`
  - `WalkDir::new`
  - `depth`
  - `is_dir`
  - `file_type`
  - `allow_analysis_dir`
  - `path`
  - `ok`
  - `map_or`
  - `extension`
  - `path`
  - `unwrap_or`
  - `strip_prefix`
  - `path`
  - `path`
  - `count`
  - `components`
  - `starts_with`
  - `path`
  - `join`
  - `into_path`

#### `main`

- **File:** src/070_layer_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `Args::parse`
  - `canonicalize`
  - `join`
  - `std::env::current_dir`
  - `unwrap_or_else`
  - `canonicalize`
  - `join`
  - `std::env::current_dir`
  - `join`
  - `unwrap`
  - `std::env::current_dir`
  - `ok`
  - `std::fs::create_dir_all`
  - `unwrap_or`
  - `canonicalize`
  - `run_analysis`

## Layer: 080_invariant_reporter.rs

### Rust Functions

#### `generate_invariant_report`

- **File:** src/080_invariant_reporter.rs:0
- **Visibility:** Public
- **Calls:**
  - `join`
  - `fs::create_dir_all`
  - `join`
  - `String::new`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `collect`
  - `filter`
  - `iter`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `collect`
  - `filter`
  - `iter`
  - `is_empty`
  - `push_str`
  - `take`
  - `iter`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `len`
  - `push_str`
  - `push_str`
  - `push_str`
  - `collect`
  - `filter`
  - `iter`
  - `is_empty`
  - `push_str`
  - `take`
  - `iter`
  - `push_str`
  - `len`
  - `push_str`
  - `push_str`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `collect`
  - `filter`
  - `iter`
  - `sort_by_key`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `collect`
  - `iter`
  - `sort_by_key`
  - `take`
  - `iter`
  - `push_str`
  - `len`
  - `push_str`
  - `push_str`
  - `fs::write`
  - `export_json`
  - `join`
  - `crate::conscience_graph::generate_conscience_map`
  - `Ok`

## Layer: 082_conscience_graph.rs

### Rust Functions

#### `generate_conscience_map`

- **File:** src/082_conscience_graph.rs:0
- **Visibility:** Public
- **Calls:**
  - `String::new`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `HashMap::new`
  - `push`
  - `or_default`
  - `entry`
  - `clone`
  - `len`
  - `count`
  - `filter`
  - `values`
  - `any`
  - `iter`
  - `is_blocking`
  - `push_str`
  - `push_str`
  - `collect`
  - `into_iter`
  - `sort_by_key`
  - `count`
  - `filter`
  - `iter`
  - `is_blocking`
  - `push_str`
  - `push_str`
  - `count`
  - `filter`
  - `iter`
  - `is_blocking`
  - `len`
  - `push_str`
  - `first`
  - `is_empty`
  - `push_str`
  - `filter`
  - `iter`
  - `is_blocking`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `std::fs::write`
  - `Ok`

#### `generate_conscience_stats`

- **File:** src/082_conscience_graph.rs:0
- **Visibility:** Public
- **Calls:**
  - `HashMap::new`
  - `push`
  - `or_default`
  - `entry`
  - `clone`
  - `len`
  - `count`
  - `filter`
  - `values`
  - `any`
  - `iter`
  - `is_blocking`
  - `count`
  - `filter`
  - `iter`
  - `count`
  - `filter`
  - `iter`
  - `count`
  - `filter`
  - `iter`
  - `len`
  - `count`
  - `filter`
  - `iter`
  - `is_blocking`

#### `kind_name`

- **File:** src/082_conscience_graph.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `to_string`
  - `to_string`
  - `to_string`
  - `to_string`
  - `to_string`
  - `to_string`
  - `to_string`
  - `to_string`

#### `make_test_invariant`

- **File:** src/082_conscience_graph.rs:0
- **Visibility:** Private
- **Calls:**
  - `Invariant::new`
  - `to_string`
  - `to_string`
  - `to_string`

## Layer: 083_action_validator.rs

### Rust Functions

#### `matches_function`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Private
- **Calls:**
  - `target`

## Layer: 085_agent_conscience.rs

### Rust Functions

#### `make_test_invariant`

- **File:** src/085_agent_conscience.rs:0
- **Visibility:** Private
- **Calls:**
  - `Invariant::new`
  - `to_string`
  - `InvariantKind::Structural`

## Layer: 110_cohesion_analyzer.rs

### Rust Functions

#### `louvain_communities`

- **File:** src/110_cohesion_analyzer.rs:0
- **Visibility:** Private
- **Calls:**
  - `len`
  - `Vec::new`
  - `build_undirected_graph`
  - `collect`
  - `collect`
  - `clone`
  - `HashMap::new`
  - `or_insert`
  - `entry`
  - `saturating_sub`

## Layer: 120_directory_analyzer.rs

### Rust Functions

#### `is_source_file`

- **File:** src/120_directory_analyzer.rs:0
- **Visibility:** Private

## Layer: 150_julia_parser.rs

### Rust Functions

#### `is_reserved`

- **File:** src/150_julia_parser.rs:0
- **Visibility:** Private

## Layer: 180_report.rs

### Rust Functions

#### `group_key_cmp`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `cmp`

#### `is_dead_code_candidate`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `get`
  - `is_empty`
  - `any`
  - `iter`
  - `path_matches`

#### `is_entrypoint_main`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `ends_with`
  - `Path::new`

#### `is_public_function`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `fs::read_to_string`
  - `lines`
  - `find`
  - `trim_start`
  - `Some`
  - `starts_with`

#### `language_label`

- **File:** src/180_report.rs:0
- **Visibility:** Private

#### `load_baseline_metrics`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `join`
  - `Path::new`
  - `fs::read_to_string`
  - `HashMap::new`
  - `lines`
  - `trim`
  - `is_empty`
  - `starts_with`
  - `split_once`
  - `to_string`
  - `trim`
  - `trim`
  - `parse`
  - `insert`
  - `Some`

#### `load_cargo_warnings`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `join`
  - `Path::new`
  - `exists`
  - `ok`
  - `fs::read_to_string`

#### `load_report_config`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `join`
  - `Path::new`
  - `ReportConfig::defaults`
  - `fs::read_to_string`
  - `lines`
  - `trim`
  - `is_empty`
  - `starts_with`
  - `split_once`
  - `trim`
  - `trim_matches`
  - `trim`
  - `parse`
  - `parse`
  - `is_empty`
  - `to_string`
  - `parse`

## Layer: 190_main.rs

### Rust Functions

#### `main`

- **File:** src/190_main.rs:0
- **Visibility:** Private
- **Calls:**
  - `collect`
  - `std::env::args`
  - `len`
  - `agent_cli::run_agent_cli`
  - `crate::layer_utilities::main`

## Layer: 191_agent_cli.rs

### Rust Functions

#### `list_invariants`

- **File:** src/191_agent_cli.rs:0
- **Visibility:** Private
- **Calls:**
  - `load_invariants`
  - `collect`
  - `cloned`
  - `filter`
  - `iter`
  - `is_blocking`
  - `serde_json::to_string_pretty`
  - `Ok`

#### `load_invariants`

- **File:** src/191_agent_cli.rs:0
- **Visibility:** Private
- **Calls:**
  - `std::fs::read_to_string`
  - `Ok`
  - `serde_json::from_str`

