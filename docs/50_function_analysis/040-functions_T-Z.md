# Functions T-Z

## Layer: 000_cluster_001.rs

### Rust Functions

#### `temp_dir`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Private
- **Calls:**
  - `std::env::temp_dir`
  - `push`

#### `test_detects_cycles`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Private
- **Calls:**
  - `unwrap`
  - `detects_cycles`

#### `test_generates_canonical_names_and_violations`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Private
- **Calls:**
  - `unwrap`
  - `generates_canonical_names_and_violations`

#### `topo_sort_orders_dependencies`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Private
- **Calls:**
  - `temp_dir`
  - `create_dir_all`
  - `join`
  - `join`
  - `join`
  - `write`
  - `write`
  - `write`
  - `analyze_file_ordering`
  - `clone`
  - `clone`
  - `clone`
  - `Some`
  - `collect`
  - `map`
  - `iter`
  - `clone`
  - `Ok`

#### `topo_sort_within`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `collect`
  - `copied`
  - `iter`
  - `HashMap::new`
  - `insert`
  - `count`
  - `filter`
  - `neighbors_directed`
  - `contains`
  - `insert`
  - `std::collections::VecDeque::new`
  - `unwrap_or`
  - `copied`
  - `get`
  - `push_back`
  - `Vec::new`
  - `pop_front`
  - `push`
  - `neighbors_directed`
  - `contains`
  - `get_mut`
  - `saturating_sub`
  - `push_back`
  - `len`
  - `len`
  - `Err`
  - `Ok`

#### `topological_sort`

- **File:** src/000_cluster_001.rs:0
- **Visibility:** Public
- **Calls:**
  - `node_indices`
  - `index`
  - `count`
  - `neighbors_directed`
  - `VecDeque::new`
  - `node_indices`
  - `index`
  - `push_back`
  - `Vec::new`
  - `pop_front`
  - `push`
  - `neighbors_directed`
  - `index`
  - `saturating_sub`
  - `push_back`
  - `len`
  - `node_count`
  - `Err`
  - `Ok`

## Layer: 000_invariant_types.rs

### Rust Functions

#### `test_confidence_from_strength`

- **File:** src/000_invariant_types.rs:0
- **Visibility:** Private

#### `test_is_blocking`

- **File:** src/000_invariant_types.rs:0
- **Visibility:** Private
- **Calls:**
  - `Invariant::new`
  - `to_string`
  - `to_string`
  - `InvariantKind::Structural`
  - `to_string`
  - `Invariant::new`
  - `to_string`
  - `to_string`
  - `InvariantKind::Semantic`
  - `to_string`

#### `test_stats_calculation`

- **File:** src/000_invariant_types.rs:0
- **Visibility:** Private
- **Calls:**
  - `InvariantAnalysisResult::new`
  - `add_invariant`
  - `Invariant::new`
  - `to_string`
  - `to_string`
  - `InvariantKind::Structural`
  - `to_string`
  - `add_invariant`
  - `Invariant::new`
  - `to_string`
  - `to_string`
  - `InvariantKind::Semantic`
  - `to_string`
  - `update_totals`

## Layer: 005_refactor_constraints.rs

### Rust Functions

#### `test_check_move_allowed_blocking`

- **File:** src/005_refactor_constraints.rs:0
- **Visibility:** Private
- **Calls:**
  - `check_move_allowed`

#### `test_check_move_allowed_non_blocking`

- **File:** src/005_refactor_constraints.rs:0
- **Visibility:** Private
- **Calls:**
  - `check_move_allowed`

#### `test_constraint_is_blocking`

- **File:** src/005_refactor_constraints.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `to_string`
  - `to_string`
  - `to_string`

#### `test_from_invariant_layer_fixed`

- **File:** src/005_refactor_constraints.rs:0
- **Visibility:** Private
- **Calls:**
  - `Invariant::new`
  - `to_string`
  - `to_string`
  - `InvariantKind::Structural`
  - `to_string`
  - `unwrap`
  - `from_invariant`

## Layer: 010_cluster_008.rs

### Rust Functions

#### `topo_sort`

- **File:** src/010_cluster_008.rs:0
- **Visibility:** Private
- **Calls:**
  - `HashMap::new`
  - `or_insert`
  - `entry`
  - `clone`
  - `values`
  - `or_insert`
  - `entry`
  - `clone`
  - `collect`
  - `filter_map`
  - `iter`
  - `Some`
  - `clone`
  - `sort`
  - `make_contiguous`
  - `Vec::new`
  - `pop_front`
  - `push`
  - `clone`
  - `get`
  - `get_mut`
  - `insert_sorted`
  - `clone`
  - `len`
  - `len`
  - `collect`
  - `cloned`
  - `filter`
  - `iter`
  - `contains`
  - `sort`
  - `clone`
  - `extend`
  - `Vec::new`

## Layer: 010_scc_compressor.rs

### Rust Functions

#### `test_scc_compression_cycle`

- **File:** src/010_scc_compressor.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `add_edge`
  - `SccCompression::new`

#### `test_scc_compression_dag`

- **File:** src/010_scc_compressor.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `SccCompression::new`

#### `test_scc_compression_mixed`

- **File:** src/010_scc_compressor.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `add_edge`
  - `add_edge`
  - `SccCompression::new`

## Layer: 020_layer_inference.rs

### Rust Functions

#### `test_detect_layer_violations_none`

- **File:** src/020_layer_inference.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `infer_layers`
  - `detect_layer_violations`

#### `test_layer_inference_diamond`

- **File:** src/020_layer_inference.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `add_edge`
  - `add_edge`
  - `infer_layers`

#### `test_layer_inference_simple_dag`

- **File:** src/020_layer_inference.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `infer_layers`

## Layer: 030_fixpoint_solver.rs

### Rust Functions

#### `test_fixpoint_convergence`

- **File:** src/030_fixpoint_solver.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `HashMap::new`
  - `insert`
  - `to_string`
  - `SymbolicAbstraction::new`
  - `insert`
  - `to_string`
  - `SymbolicAbstraction::new`
  - `propagate_to_fixpoint`

#### `test_fixpoint_simple`

- **File:** src/030_fixpoint_solver.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `HashMap::new`
  - `SymbolicAbstraction::new`
  - `insert`
  - `to_string`
  - `insert`
  - `to_string`
  - `insert`
  - `to_string`
  - `SymbolicAbstraction::new`
  - `insert`
  - `to_string`
  - `SymbolicAbstraction::new`
  - `propagate_to_fixpoint`

#### `test_symbolic_abstraction_merge`

- **File:** src/030_fixpoint_solver.rs:0
- **Visibility:** Private
- **Calls:**
  - `SymbolicAbstraction::new`
  - `Some`
  - `to_string`
  - `insert`
  - `to_string`
  - `Some`
  - `SymbolicAbstraction::new`
  - `insert`
  - `to_string`
  - `Some`
  - `merge`

## Layer: 040_structural_detector.rs

### Rust Functions

#### `test_all_structural_invariants_proven`

- **File:** src/040_structural_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `StructuralDetector::new`
  - `detect_all`
  - `count`
  - `filter`
  - `iter`

#### `test_detect_degree_stable`

- **File:** src/040_structural_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `StructuralDetector::new`
  - `detect_degree_stable`
  - `unwrap`
  - `find`
  - `iter`

#### `test_detect_leaf_root`

- **File:** src/040_structural_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `StructuralDetector::new`
  - `detect_leaf_root`
  - `collect`
  - `filter`
  - `iter`
  - `collect`
  - `filter`
  - `iter`

## Layer: 050_semantic_detector.rs

### Rust Functions

#### `test_detect_idempotent_heuristic`

- **File:** src/050_semantic_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `SemanticDetector::new`
  - `detect_idempotent`

#### `test_detect_pure_function_heuristic`

- **File:** src/050_semantic_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `SemanticDetector::new`
  - `detect_pure_function`

#### `test_detect_type_stable`

- **File:** src/050_semantic_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `SemanticDetector::new`
  - `detect_type_stable`

#### `test_no_pure_for_mutable`

- **File:** src/050_semantic_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `SemanticDetector::new`
  - `detect_pure_function`
  - `count`
  - `filter`
  - `iter`

## Layer: 060_path_detector.rs

### Rust Functions

#### `test_extract_facts_from_path`

- **File:** src/060_path_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `PathDetector::new`
  - `extract_facts_from_path`

#### `test_path_detector_diamond`

- **File:** src/060_path_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `add_edge`
  - `add_edge`
  - `PathDetector::new`
  - `detect_all`
  - `collect`
  - `filter`
  - `iter`
  - `is_empty`

#### `test_path_detector_simple`

- **File:** src/060_path_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `add_edge`
  - `PathDetector::new`
  - `detect_all`

#### `test_path_stats`

- **File:** src/060_path_detector.rs:0
- **Visibility:** Private
- **Calls:**
  - `DiGraph::new`
  - `add_node`
  - `to_string`
  - `add_node`
  - `to_string`
  - `add_edge`
  - `PathDetector::new`
  - `get_stats`

## Layer: 070_invariant_integrator.rs

### Rust Functions

#### `test_detect_all`

- **File:** src/070_invariant_integrator.rs:0
- **Visibility:** Private
- **Calls:**
  - `make_simple_analysis`
  - `InvariantDetector::new`
  - `detect_all`

#### `test_invariant_detector_creation`

- **File:** src/070_invariant_integrator.rs:0
- **Visibility:** Private
- **Calls:**
  - `make_simple_analysis`
  - `InvariantDetector::new`

## Layer: 080_invariant_reporter.rs

### Rust Functions

#### `test_generate_report`

- **File:** src/080_invariant_reporter.rs:0
- **Visibility:** Private
- **Calls:**
  - `Vec::new`
  - `HashMap::new`
  - `join`
  - `std::env::temp_dir`
  - `unwrap`
  - `fs::create_dir_all`
  - `generate_invariant_report`
  - `fs::remove_dir_all`

## Layer: 082_conscience_graph.rs

### Rust Functions

#### `test_generate_stats`

- **File:** src/082_conscience_graph.rs:0
- **Visibility:** Private
- **Calls:**
  - `generate_conscience_stats`

#### `test_strength_emoji`

- **File:** src/082_conscience_graph.rs:0
- **Visibility:** Private
- **Calls:**
  - `make_test_invariant`
  - `InvariantKind::Structural`
  - `make_test_invariant`
  - `InvariantKind::Semantic`
  - `to_string`
  - `make_test_invariant`
  - `InvariantKind::Semantic`

## Layer: 083_action_validator.rs

### Rust Functions

#### `test_check_move_allowed`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Private
- **Calls:**
  - `check_move_allowed`
  - `PathBuf::from`
  - `PathBuf::from`

#### `test_extract_layer`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Private

#### `test_validate_allowed_action`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `PathBuf::from`
  - `PathBuf::from`
  - `validate_action`

#### `test_validate_layer_fixed_constraint`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `PathBuf::from`
  - `PathBuf::from`
  - `validate_action`

#### `test_validate_no_move_constraint`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `PathBuf::from`
  - `PathBuf::from`
  - `validate_action`
  - `unwrap_err`

#### `test_validate_preserve_signature`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_string`
  - `to_string`
  - `to_string`
  - `PathBuf::from`
  - `validate_action`

#### `validate_action`

- **File:** src/083_action_validator.rs:0
- **Visibility:** Public
- **Calls:**
  - `Vec::new`
  - `enumerate`
  - `iter`
  - `push`
  - `extract_layer`
  - `extract_layer`
  - `push`
  - `push`
  - `contains`
  - `push`
  - `is_empty`
  - `Ok`
  - `Err`

## Layer: 085_agent_conscience.rs

### Rust Functions

#### `test_conscience_allows_valid_action`

- **File:** src/085_agent_conscience.rs:0
- **Visibility:** Private
- **Calls:**
  - `make_test_invariant`
  - `AgentConscience::new`
  - `to_string`
  - `PathBuf::from`
  - `PathBuf::from`
  - `check_action`

#### `test_conscience_blocks_invalid_move`

- **File:** src/085_agent_conscience.rs:0
- **Visibility:** Private
- **Calls:**
  - `make_test_invariant`
  - `AgentConscience::new`
  - `to_string`
  - `PathBuf::from`
  - `PathBuf::from`
  - `check_action`

#### `test_conscience_stats`

- **File:** src/085_agent_conscience.rs:0
- **Visibility:** Private
- **Calls:**
  - `AgentConscience::new`
  - `stats`

#### `test_query_allowed_actions`

- **File:** src/085_agent_conscience.rs:0
- **Visibility:** Private
- **Calls:**
  - `make_test_invariant`
  - `AgentConscience::new`
  - `query_allowed_actions`

## Layer: 090_utilities.rs

### Rust Functions

#### `write_cluster_batches`

- **File:** src/090_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push_str`
  - `enumerate`
  - `iter`
  - `push_str`
  - `push_str`
  - `push_str`
  - `Vec::new`
  - `exists`
  - `push_str`
  - `push`
  - `push_str`
  - `compress_path`
  - `as_ref`
  - `to_string_lossy`
  - `push_str`
  - `push`
  - `push`
  - `push_str`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push`

#### `write_structural_batches`

- **File:** src/090_utilities.rs:0
- **Visibility:** Public
- **Calls:**
  - `is_empty`
  - `Vec::new`
  - `HashMap::new`
  - `or_default`
  - `entry`
  - `clone`
  - `is_empty`
  - `push`
  - `clone`
  - `push`
  - `push_str`
  - `push_str`
  - `push_str`
  - `enumerate`
  - `iter`
  - `Vec::new`
  - `unwrap_or`
  - `get`
  - `push_str`
  - `push_str`
  - `push_str`
  - `Vec::new`
  - `exists`
  - `compress_path`
  - `as_ref`
  - `to_string_lossy`
  - `push_str`
  - `push`
  - `unwrap_or`
  - `as_deref`
  - `unwrap_or_else`
  - `map`
  - `as_ref`
  - `compress_path`
  - `as_ref`
  - `to_string_lossy`
  - `to_string`
  - `to_string`
  - `push_str`
  - `push`
  - `clone`
  - `sort`
  - `dedup`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push`
  - `push_str`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push`

## Layer: 160_rust_parser.rs

### Rust Functions

#### `truncate_label`

- **File:** src/160_rust_parser.rs:0
- **Visibility:** Private
- **Calls:**
  - `join`
  - `collect`
  - `split_whitespace`
  - `len`
  - `truncate`
  - `push_str`

## Layer: 180_report.rs

### Rust Functions

#### `visibility_label`

- **File:** src/180_report.rs:0
- **Visibility:** Private

#### `write_baseline_metrics`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `join`
  - `Path::new`
  - `exists`
  - `fs::write`

#### `write_cluster_tips`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`

#### `write_priority_section`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `push_str`
  - `push_str`
  - `push_str`
  - `is_empty`
  - `push_str`
  - `Vec::new`
  - `push_str`
  - `is_empty`
  - `push`
  - `clone`
  - `push`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push_str`

#### `write_structural_tips`

- **File:** src/180_report.rs:0
- **Visibility:** Private
- **Calls:**
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`

## Layer: 191_agent_cli.rs

### Rust Functions

#### `test_load_invariants_empty`

- **File:** src/191_agent_cli.rs:0
- **Visibility:** Private
- **Calls:**
  - `join`
  - `std::env::temp_dir`
  - `unwrap`
  - `std::fs::write`
  - `load_invariants`
  - `std::fs::remove_file`

