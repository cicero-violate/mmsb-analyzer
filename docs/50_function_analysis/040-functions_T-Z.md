# Functions T-Z

## Layer: 000_cluster_001.rs

### Rust Functions

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

## Layer: 190_action_validator.rs

### Rust Functions

#### `validate_action`

- **File:** src/190_action_validator.rs:0
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

## Layer: 210_utilities.rs

### Rust Functions

#### `write_cluster_batches`

- **File:** src/210_utilities.rs:0
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

- **File:** src/210_utilities.rs:0
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

## Layer: 410_dead_code_entrypoints.rs

### Rust Functions

#### `treat_public_as_entrypoint`

- **File:** src/410_dead_code_entrypoints.rs:0
- **Visibility:** Private
- **Calls:**
  - `unwrap_or`
  - `map`

## Layer: 460_dead_code_report.rs

### Rust Functions

#### `write_outputs`

- **File:** src/460_dead_code_report.rs:0
- **Visibility:** Crate
- **Calls:**
  - `unwrap_or_else`
  - `clone`
  - `join`
  - `parent`
  - `std::fs::create_dir_all`
  - `write_report`
  - `unwrap_or_else`
  - `clone`
  - `join`
  - `parent`
  - `std::fs::create_dir_all`
  - `write_summary_markdown`
  - `unwrap_or_else`
  - `map`
  - `parent`
  - `to_path_buf`
  - `clone`
  - `join`
  - `write_plan_markdown`
  - `Ok`

#### `write_report`

- **File:** src/460_dead_code_report.rs:0
- **Visibility:** Public
- **Calls:**
  - `serde_json::to_string_pretty`
  - `std::fs::write`

## Layer: 530_dead_code_report_split.rs

### Rust Functions

#### `top_items`

- **File:** src/530_dead_code_report_split.rs:0
- **Visibility:** Private
- **Calls:**
  - `to_vec`
  - `sort_by_key`
  - `len`
  - `truncate`

#### `write_plan_markdown`

- **File:** src/530_dead_code_report_split.rs:0
- **Visibility:** Public
- **Calls:**
  - `String::new`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `top_items`
  - `push_str`
  - `is_empty`
  - `push_str`
  - `plan_options`
  - `push_str`
  - `push`
  - `std::fs::write`

#### `write_summary_markdown`

- **File:** src/530_dead_code_report_split.rs:0
- **Visibility:** Public
- **Calls:**
  - `String::new`
  - `push_str`
  - `push_str`
  - `push_str`
  - `push_str`
  - `top_items`
  - `push_str`
  - `is_empty`
  - `push_str`
  - `push_str`
  - `push`
  - `std::fs::write`

## Layer: 610_correction_plan_serializer.rs

### Rust Functions

#### `write_intelligence_outputs_at`

- **File:** src/610_correction_plan_serializer.rs:0
- **Visibility:** Public
- **Calls:**
  - `std::fs::create_dir_all`
  - `unwrap_or_else`
  - `map`
  - `to_path_buf`
  - `join`
  - `parent`
  - `std::fs::create_dir_all`
  - `serialize_correction_plans`
  - `std::fs::write`
  - `serde_json::to_string_pretty`
  - `unwrap_or_else`
  - `map`
  - `to_path_buf`
  - `join`
  - `parent`
  - `std::fs::create_dir_all`
  - `emit_verification_policy`
  - `Ok`

## Layer: 630_correction_intelligence_report.rs

### Rust Functions

#### `write_intelligence_outputs`

- **File:** src/630_correction_intelligence_report.rs:0
- **Visibility:** Public
- **Calls:**
  - `write_intelligence_outputs_at`

