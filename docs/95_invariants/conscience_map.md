# Conscience Map

## Overview

This map shows which functions are protected by mechanical constraints.
Functions with blocking invariants cannot be refactored without violating proven properties.

**Total Functions**: 229

**Protected Functions**: 229 (100.0%)

---

## Functions by Protection Level

### `relativize_path` (81% protected)

**File**: `src/160_rust_parser.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(176, size=1)**: Member of SCC 176 (size: 1)
- 🔒 ✓ **SCC(181, size=1)**: Member of SCC 181 (size: 1)

### `layer_prefix_value` (80% protected)

**File**: `src/050_cluster_006.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **DegreeStable(in=15, out=0)**: Degree: in=15, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(4, size=1)**: Member of SCC 4 (size: 1)
- 🔒 ✓ **SCC(57, size=1)**: Member of SCC 57 (size: 1)

### `resolve_source_root` (80% protected)

**File**: `src/070_layer_utilities.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(75, size=1)**: Member of SCC 75 (size: 1)
- 🔒 ✓ **SCC(115, size=1)**: Member of SCC 115 (size: 1)

### `make_test_invariant` (80% protected)

**File**: `src/085_agent_conscience.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **DegreeStable(in=6, out=0)**: Degree: in=6, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(128, size=1)**: Member of SCC 128 (size: 1)
- 🔒 ✓ **SCC(130, size=1)**: Member of SCC 130 (size: 1)

### `main` (80% protected)

**File**: `src/190_main.rs`

- 🔒 ✓ **LayerFixed(3)**: Layer 3 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(118, size=1)**: Member of SCC 118 (size: 1)
- 🔒 ✓ **SCC(230, size=1)**: Member of SCC 230 (size: 1)

### `check_move_allowed` (77% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **DegreeStable(in=3, out=1)**: Degree: in=3, out=1 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(41, size=1)**: Member of SCC 41 (size: 1)
- 🔒 ✓ **SCC(46, size=1)**: Member of SCC 46 (size: 1)

### `detect_layer_violations` (77% protected)

**File**: `src/020_layer_inference.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(62, size=1)**: Member of SCC 62 (size: 1)
- 🔒 ✓ **SCC(78, size=1)**: Member of SCC 78 (size: 1)

### `suggest_file` (83% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(162, size=1)**: Member of SCC 162 (size: 1)

### `slugify_key` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(220, size=1)**: Member of SCC 220 (size: 1)

### `short_signature` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(226, size=1)**: Member of SCC 226 (size: 1)

### `test_detect_degree_stable` (100% protected)

**File**: `src/040_structural_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(93, size=1)**: Member of SCC 93 (size: 1)

### `write_priority_section` (100% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(191, size=1)**: Member of SCC 191 (size: 1)

### `compute_cluster_cohesion` (71% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(163, size=1)**: Member of SCC 163 (size: 1)

### `compute_ordering_correctness` (71% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(217, size=1)**: Member of SCC 217 (size: 1)

### `test_path_stats` (100% protected)

**File**: `src/060_path_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(111, size=1)**: Member of SCC 111 (size: 1)

### `test_confidence_from_strength` (100% protected)

**File**: `src/000_invariant_types.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(37, size=1)**: Member of SCC 37 (size: 1)

### `node_style` (83% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(68, size=1)**: Member of SCC 68 (size: 1)

### `resolve_path` (83% protected)

**File**: `src/030_cluster_011.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(83, size=1)**: Member of SCC 83 (size: 1)

### `is_source_file` (71% protected)

**File**: `src/120_directory_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(167, size=1)**: Member of SCC 167 (size: 1)

### `baseline_deltas` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(212, size=1)**: Member of SCC 212 (size: 1)

### `sort_plan_items` (100% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(194, size=1)**: Member of SCC 194 (size: 1)

### `collect_directories` (100% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(214, size=1)**: Member of SCC 214 (size: 1)

### `render_mermaid_graph` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(216, size=1)**: Member of SCC 216 (size: 1)

### `test_constraint_is_blocking` (100% protected)

**File**: `src/005_refactor_constraints.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(49, size=1)**: Member of SCC 49 (size: 1)

### `test_scc_compression_dag` (100% protected)

**File**: `src/010_scc_compressor.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(70, size=1)**: Member of SCC 70 (size: 1)

### `slugify_relative` (83% protected)

**File**: `src/150_julia_parser.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(171, size=1)**: Member of SCC 171 (size: 1)

### `test_scc_compression_mixed` (100% protected)

**File**: `src/010_scc_compressor.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(72, size=1)**: Member of SCC 72 (size: 1)

### `sanitize_mermaid_label` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(229, size=1)**: Member of SCC 229 (size: 1)

### `write_cluster_tips` (100% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(193, size=1)**: Member of SCC 193 (size: 1)

### `collect_roots_from_crate` (100% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(28, size=1)**: Member of SCC 28 (size: 1)

### `sort_cluster_items` (100% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(195, size=1)**: Member of SCC 195 (size: 1)

### `compute_directory_cohesion` (71% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(218, size=1)**: Member of SCC 218 (size: 1)

### `visibility_label` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(225, size=1)**: Member of SCC 225 (size: 1)

### `export_complete_program_dot` (83% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(33, size=1)**: Member of SCC 33 (size: 1)

### `escape_dot` (83% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(34, size=1)**: Member of SCC 34 (size: 1)

### `collect_rename_items` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(188, size=1)**: Member of SCC 188 (size: 1)

### `matches_function` (83% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(132, size=1)**: Member of SCC 132 (size: 1)

### `test_path_detector_simple` (100% protected)

**File**: `src/060_path_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(108, size=1)**: Member of SCC 108 (size: 1)

### `strength_emoji` (83% protected)

**File**: `src/082_conscience_graph.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(125, size=1)**: Member of SCC 125 (size: 1)

### `test_detect_leaf_root` (100% protected)

**File**: `src/040_structural_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(92, size=1)**: Member of SCC 92 (size: 1)

### `cyclomatic_complexity` (83% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(69, size=1)**: Member of SCC 69 (size: 1)

### `test_scc_compression_cycle` (100% protected)

**File**: `src/010_scc_compressor.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(71, size=1)**: Member of SCC 71 (size: 1)

### `paren_balance` (83% protected)

**File**: `src/150_julia_parser.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(180, size=1)**: Member of SCC 180 (size: 1)

### `normalize_use_stmt` (71% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(227, size=1)**: Member of SCC 227 (size: 1)

### `test_stats_calculation` (100% protected)

**File**: `src/000_invariant_types.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(39, size=1)**: Member of SCC 39 (size: 1)

### `language_label` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(224, size=1)**: Member of SCC 224 (size: 1)

### `parse_module_name` (71% protected)

**File**: `src/150_julia_parser.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(174, size=1)**: Member of SCC 174 (size: 1)

### `slugify_path` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(215, size=1)**: Member of SCC 215 (size: 1)

### `placement_status_label` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(186, size=1)**: Member of SCC 186 (size: 1)

### `test_extract_layer` (100% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(133, size=1)**: Member of SCC 133 (size: 1)

### `test_no_pure_for_mutable` (83% protected)

**File**: `src/050_semantic_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(105, size=1)**: Member of SCC 105 (size: 1)

### `prefix_key_from_path` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(219, size=1)**: Member of SCC 219 (size: 1)

### `test_detect_pure_function_heuristic` (83% protected)

**File**: `src/050_semantic_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(103, size=1)**: Member of SCC 103 (size: 1)

### `kind_name` (83% protected)

**File**: `src/082_conscience_graph.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(126, size=1)**: Member of SCC 126 (size: 1)

### `determine_status` (83% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(161, size=1)**: Member of SCC 161 (size: 1)

### `placement_status_notes` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(187, size=1)**: Member of SCC 187 (size: 1)

### `sanitize_identifier` (83% protected)

**File**: `src/130_control_flow.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(169, size=1)**: Member of SCC 169 (size: 1)

### `find_julia_project_dir` (83% protected)

**File**: `src/150_julia_parser.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(173, size=1)**: Member of SCC 173 (size: 1)

### `make_function` (83% protected)

**File**: `src/050_semantic_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(101, size=1)**: Member of SCC 101 (size: 1)

### `resolve_julia_binary` (83% protected)

**File**: `src/150_julia_parser.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(172, size=1)**: Member of SCC 172 (size: 1)

### `generate_conscience_map` (83% protected)

**File**: `src/082_conscience_graph.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(124, size=1)**: Member of SCC 124 (size: 1)

### `load_baseline_metrics` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(211, size=1)**: Member of SCC 211 (size: 1)

### `collect_roots` (100% protected)

**File**: `src/040_dependency.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(91, size=1)**: Member of SCC 91 (size: 1)

### `should_skip_path` (83% protected)

**File**: `src/120_directory_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(168, size=1)**: Member of SCC 168 (size: 1)

### `collect_utility_candidates` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(189, size=1)**: Member of SCC 189 (size: 1)

### `sanitize_mermaid_id` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(228, size=1)**: Member of SCC 228 (size: 1)

### `parse_dead_code_warnings` (71% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(199, size=1)**: Member of SCC 199 (size: 1)

### `test_all_structural_invariants_proven` (100% protected)

**File**: `src/040_structural_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(94, size=1)**: Member of SCC 94 (size: 1)

### `write_structural_tips` (100% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(192, size=1)**: Member of SCC 192 (size: 1)

### `compute_cohesion_score` (71% protected)

**File**: `src/050_cluster_006.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(100, size=1)**: Member of SCC 100 (size: 1)

### `function_bucket_label` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(222, size=1)**: Member of SCC 222 (size: 1)

### `julia_entry_paths` (83% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(31, size=1)**: Member of SCC 31 (size: 1)

### `display_path` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(185, size=1)**: Member of SCC 185 (size: 1)

### `test_detect_type_stable` (83% protected)

**File**: `src/050_semantic_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(102, size=1)**: Member of SCC 102 (size: 1)

### `test_detect_idempotent_heuristic` (100% protected)

**File**: `src/050_semantic_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(104, size=1)**: Member of SCC 104 (size: 1)

### `test_extract_facts_from_path` (100% protected)

**File**: `src/060_path_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(110, size=1)**: Member of SCC 110 (size: 1)

### `build_function_layers` (83% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(156, size=1)**: Member of SCC 156 (size: 1)

### `group_key_cmp` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(221, size=1)**: Member of SCC 221 (size: 1)

### `test_is_blocking` (100% protected)

**File**: `src/000_invariant_types.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(38, size=1)**: Member of SCC 38 (size: 1)

### `test_symbolic_abstraction_merge` (100% protected)

**File**: `src/030_fixpoint_solver.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(88, size=1)**: Member of SCC 88 (size: 1)

### `write_baseline_metrics` (100% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(213, size=1)**: Member of SCC 213 (size: 1)

### `suggest_cluster_file` (83% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(164, size=1)**: Member of SCC 164 (size: 1)

### `parse_struct_name` (71% protected)

**File**: `src/150_julia_parser.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(175, size=1)**: Member of SCC 175 (size: 1)

### `export_constraints_json` (83% protected)

**File**: `src/080_invariant_reporter.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(122, size=1)**: Member of SCC 122 (size: 1)

### `parallel_build_file_dag` (83% protected)

**File**: `src/140_file_ordering.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(170, size=1)**: Member of SCC 170 (size: 1)

### `directory_moves_to_plan` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(190, size=1)**: Member of SCC 190 (size: 1)

### `build_type_maps` (83% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(157, size=1)**: Member of SCC 157 (size: 1)

### `load_report_config` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(209, size=1)**: Member of SCC 209 (size: 1)

### `test_path_detector_diamond` (100% protected)

**File**: `src/060_path_detector.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(109, size=1)**: Member of SCC 109 (size: 1)

### `test_conscience_stats` (100% protected)

**File**: `src/085_agent_conscience.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(143, size=1)**: Member of SCC 143 (size: 1)

### `slugify_file_path` (83% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(223, size=1)**: Member of SCC 223 (size: 1)

### `collect_size_warnings` (100% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=0)**: Degree: in=0, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(210, size=1)**: Member of SCC 210 (size: 1)

### `adjacency_from_edges` (80% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(55, size=1)**: Member of SCC 55 (size: 1)

### `build_module_map` (80% protected)

**File**: `src/030_cluster_011.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(82, size=1)**: Member of SCC 82 (size: 1)

### `test_validate_layer_fixed_constraint` (100% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(135, size=1)**: Member of SCC 135 (size: 1)

### `detect_cycles` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(6, size=1)**: Member of SCC 6 (size: 1)

### `test_check_move_allowed_blocking` (100% protected)

**File**: `src/005_refactor_constraints.rs`

- 🔒 ✓ **LayerFixed(3)**: Layer 3 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(47, size=1)**: Member of SCC 47 (size: 1)

### `test_generate_stats` (100% protected)

**File**: `src/082_conscience_graph.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(129, size=1)**: Member of SCC 129 (size: 1)

### `infer_layers` (80% protected)

**File**: `src/020_layer_inference.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=3, out=0)**: Degree: in=3, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(77, size=1)**: Member of SCC 77 (size: 1)

### `is_public_function` (66% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(203, size=1)**: Member of SCC 203 (size: 1)

### `test_strength_emoji` (100% protected)

**File**: `src/082_conscience_graph.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=3)**: Degree: in=0, out=3 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(131, size=1)**: Member of SCC 131 (size: 1)

### `test_conscience_allows_valid_action` (100% protected)

**File**: `src/085_agent_conscience.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(142, size=1)**: Member of SCC 142 (size: 1)

### `load_cargo_warnings` (80% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(198, size=1)**: Member of SCC 198 (size: 1)

### `build_call_analysis` (80% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(160, size=1)**: Member of SCC 160 (size: 1)

### `collect_cluster_items` (80% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(197, size=1)**: Member of SCC 197 (size: 1)

### `pat_snippet` (80% protected)

**File**: `src/160_rust_parser.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(184, size=1)**: Member of SCC 184 (size: 1)

### `run_agent_cli` (80% protected)

**File**: `src/191_agent_cli.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=4)**: Degree: in=0, out=4 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(234, size=1)**: Member of SCC 234 (size: 1)

### `is_entrypoint_main` (66% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(205, size=1)**: Member of SCC 205 (size: 1)

### `order_directories` (80% protected)

**File**: `src/050_cluster_006.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(96, size=1)**: Member of SCC 96 (size: 1)

### `path_common_prefix_len` (80% protected)

**File**: `src/090_utilities.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(147, size=1)**: Member of SCC 147 (size: 1)

### `write_cluster_batches` (100% protected)

**File**: `src/090_utilities.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(152, size=1)**: Member of SCC 152 (size: 1)

### `strip_numeric_prefix` (80% protected)

**File**: `src/050_cluster_006.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(97, size=1)**: Member of SCC 97 (size: 1)

### `collect_cluster_plans` (80% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(67, size=1)**: Member of SCC 67 (size: 1)

### `topo_sort_orders_dependencies` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(3)**: Layer 3 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(25, size=1)**: Member of SCC 25 (size: 1)

### `collect_directory_moves` (80% protected)

**File**: `src/050_cluster_006.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(99, size=1)**: Member of SCC 99 (size: 1)

### `extract_layer` (80% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(44, size=1)**: Member of SCC 44 (size: 1)

### `test_validate_allowed_action` (100% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(137, size=1)**: Member of SCC 137 (size: 1)

### `test_detect_all` (100% protected)

**File**: `src/070_invariant_integrator.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(114, size=1)**: Member of SCC 114 (size: 1)

### `louvain_communities` (80% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(166, size=1)**: Member of SCC 166 (size: 1)

### `ordered_by_name` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=4, out=0)**: Degree: in=4, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(1, size=1)**: Member of SCC 1 (size: 1)

### `collect_naming_warnings` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(7)**: Layer 7 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(19, size=1)**: Member of SCC 19 (size: 1)

### `cluster_priority` (80% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(196, size=1)**: Member of SCC 196 (size: 1)

### `collect_rust_dependencies` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(27, size=1)**: Member of SCC 27 (size: 1)

### `export_json` (80% protected)

**File**: `src/080_invariant_reporter.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(120, size=1)**: Member of SCC 120 (size: 1)

### `generate_constraints` (80% protected)

**File**: `src/005_refactor_constraints.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(42, size=1)**: Member of SCC 42 (size: 1)

### `normalize_module_name` (66% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(10, size=1)**: Member of SCC 10 (size: 1)

### `test_validate_no_move_constraint` (100% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(134, size=1)**: Member of SCC 134 (size: 1)

### `generate_conscience_stats` (80% protected)

**File**: `src/082_conscience_graph.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(127, size=1)**: Member of SCC 127 (size: 1)

### `naming_score_for_file` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(18, size=1)**: Member of SCC 18 (size: 1)

### `test_from_invariant_layer_fixed` (100% protected)

**File**: `src/005_refactor_constraints.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(43, size=1)**: Member of SCC 43 (size: 1)

### `compress_path` (80% protected)

**File**: `src/090_utilities.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=5, out=0)**: Degree: in=5, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(145, size=1)**: Member of SCC 145 (size: 1)

### `build_undirected_graph` (80% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(165, size=1)**: Member of SCC 165 (size: 1)

### `extract_calls_from_lines` (80% protected)

**File**: `src/150_julia_parser.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(179, size=1)**: Member of SCC 179 (size: 1)

### `rust_entry_paths` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(26, size=1)**: Member of SCC 26 (size: 1)

### `build_entries` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(0, size=1)**: Member of SCC 0 (size: 1)

### `test_validate_preserve_signature` (80% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(136, size=1)**: Member of SCC 136 (size: 1)

### `propagate_to_fixpoint` (80% protected)

**File**: `src/030_fixpoint_solver.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(87, size=1)**: Member of SCC 87 (size: 1)

### `generate_canonical_name` (80% protected)

**File**: `src/050_cluster_006.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(98, size=1)**: Member of SCC 98 (size: 1)

### `build_file_dependency_graph` (80% protected)

**File**: `src/030_cluster_011.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(85, size=1)**: Member of SCC 85 (size: 1)

### `topological_sort` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(2, size=1)**: Member of SCC 2 (size: 1)

### `order_julia_files_by_dependency` (80% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(76, size=1)**: Member of SCC 76 (size: 1)

### `make_simple_analysis` (80% protected)

**File**: `src/070_invariant_integrator.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(112, size=1)**: Member of SCC 112 (size: 1)

### `collect_functions` (80% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(153, size=1)**: Member of SCC 153 (size: 1)

### `test_fixpoint_simple` (100% protected)

**File**: `src/030_fixpoint_solver.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(89, size=1)**: Member of SCC 89 (size: 1)

### `build_result` (80% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=6)**: Degree: in=0, out=6 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(56, size=1)**: Member of SCC 56 (size: 1)

### `filter_orphaned` (80% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=6)**: Degree: in=0, out=6 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(208, size=1)**: Member of SCC 208 (size: 1)

### `test_layer_inference_simple_dag` (100% protected)

**File**: `src/020_layer_inference.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(79, size=1)**: Member of SCC 79 (size: 1)

### `build_call_edges` (80% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(155, size=1)**: Member of SCC 155 (size: 1)

### `truncate_label` (80% protected)

**File**: `src/160_rust_parser.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(182, size=1)**: Member of SCC 182 (size: 1)

### `build_module_root_map` (80% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(74, size=1)**: Member of SCC 74 (size: 1)

### `compare_path_components` (80% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(59, size=1)**: Member of SCC 59 (size: 1)

### `test_layer_inference_diamond` (100% protected)

**File**: `src/020_layer_inference.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(80, size=1)**: Member of SCC 80 (size: 1)

### `extract_identifiers` (80% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(158, size=1)**: Member of SCC 158 (size: 1)

### `temp_dir` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=3, out=0)**: Degree: in=3, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(20, size=1)**: Member of SCC 20 (size: 1)

### `collect_julia_dependencies` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(30, size=1)**: Member of SCC 30 (size: 1)

### `is_reserved` (66% protected)

**File**: `src/150_julia_parser.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(177, size=1)**: Member of SCC 177 (size: 1)

### `is_core_module_path` (66% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(65, size=1)**: Member of SCC 65 (size: 1)

### `build_directory_dag` (80% protected)

**File**: `src/030_cluster_011.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(84, size=1)**: Member of SCC 84 (size: 1)

### `test_check_move_allowed` (100% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(3)**: Layer 3 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(138, size=1)**: Member of SCC 138 (size: 1)

### `test_invariant_detector_creation` (100% protected)

**File**: `src/070_invariant_integrator.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(113, size=1)**: Member of SCC 113 (size: 1)

### `write_structural_batches` (100% protected)

**File**: `src/090_utilities.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(151, size=1)**: Member of SCC 151 (size: 1)

### `is_mmsb_main` (66% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(51, size=1)**: Member of SCC 51 (size: 1)

### `test_generates_canonical_names_and_violations` (100% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(4)**: Layer 4 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(36, size=1)**: Member of SCC 36 (size: 1)

### `layer_adheres` (80% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(60, size=1)**: Member of SCC 60 (size: 1)

### `test_generate_report` (100% protected)

**File**: `src/080_invariant_reporter.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(123, size=1)**: Member of SCC 123 (size: 1)

### `test_conscience_blocks_invalid_move` (100% protected)

**File**: `src/085_agent_conscience.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(141, size=1)**: Member of SCC 141 (size: 1)

### `collect_directory_files` (100% protected)

**File**: `src/090_utilities.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(146, size=1)**: Member of SCC 146 (size: 1)

### `parse_use_symbols` (66% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(200, size=1)**: Member of SCC 200 (size: 1)

### `layer_rank_map` (80% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(52, size=1)**: Member of SCC 52 (size: 1)

### `order_rust_files_by_dependency` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=3)**: Degree: in=0, out=3 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(29, size=1)**: Member of SCC 29 (size: 1)

### `test_detects_cycles` (100% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(4)**: Layer 4 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(35, size=1)**: Member of SCC 35 (size: 1)

### `compute_move_metrics` (66% protected)

**File**: `src/090_utilities.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(149, size=1)**: Member of SCC 149 (size: 1)

### `expr_snippet` (80% protected)

**File**: `src/160_rust_parser.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(183, size=1)**: Member of SCC 183 (size: 1)

### `load_invariants` (80% protected)

**File**: `src/191_agent_cli.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=5, out=0)**: Degree: in=5, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(139, size=1)**: Member of SCC 139 (size: 1)

### `parse_cluster_members` (66% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(64, size=1)**: Member of SCC 64 (size: 1)

### `test_load_invariants_empty` (100% protected)

**File**: `src/191_agent_cli.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(235, size=1)**: Member of SCC 235 (size: 1)

### `contains_tools` (80% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(73, size=1)**: Member of SCC 73 (size: 1)

### `compare_dir_layers` (80% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(58, size=1)**: Member of SCC 58 (size: 1)

### `gather_julia_files` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(32, size=1)**: Member of SCC 32 (size: 1)

### `test_fixpoint_convergence` (100% protected)

**File**: `src/030_fixpoint_solver.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(90, size=1)**: Member of SCC 90 (size: 1)

### `test_query_allowed_actions` (100% protected)

**File**: `src/085_agent_conscience.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(144, size=1)**: Member of SCC 144 (size: 1)

### `build_name_map` (80% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(154, size=1)**: Member of SCC 154 (size: 1)

### `common_root` (80% protected)

**File**: `src/050_cluster_006.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(95, size=1)**: Member of SCC 95 (size: 1)

### `allow_analysis_dir` (80% protected)

**File**: `src/070_layer_utilities.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(116, size=1)**: Member of SCC 116 (size: 1)

### `test_check_move_allowed_non_blocking` (100% protected)

**File**: `src/005_refactor_constraints.rs`

- 🔒 ✓ **LayerFixed(3)**: Layer 3 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=1)**: Degree: in=0, out=1 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(48, size=1)**: Member of SCC 48 (size: 1)

### `from_invariant` (80% protected)

**File**: `src/005_refactor_constraints.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(40, size=1)**: Member of SCC 40 (size: 1)

### `path_matches` (80% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(204, size=1)**: Member of SCC 204 (size: 1)

### `detect_layer_violation` (80% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(63, size=1)**: Member of SCC 63 (size: 1)

### `test_detect_layer_violations_none` (100% protected)

**File**: `src/020_layer_inference.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=2)**: Degree: in=0, out=2 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(81, size=1)**: Member of SCC 81 (size: 1)

### `build_file_dag` (80% protected)

**File**: `src/030_cluster_011.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=3, out=0)**: Degree: in=3, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(7, size=1)**: Member of SCC 7 (size: 1)

### `insert_sorted` (100% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(53, size=1)**: Member of SCC 53 (size: 1)

### `detect_violations` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(21, size=1)**: Member of SCC 21 (size: 1)

### `detect_layer` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=0)**: Degree: in=2, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(8, size=1)**: Member of SCC 8 (size: 1)

### `topo_sort_within` (80% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(3, size=1)**: Member of SCC 3 (size: 1)

### `export_program_cfg_to_path` (80% protected)

**File**: `src/030_cluster_011.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(86, size=1)**: Member of SCC 86 (size: 1)

### `collect_move_items` (80% protected)

**File**: `src/090_utilities.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=5)**: Degree: in=0, out=5 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(150, size=1)**: Member of SCC 150 (size: 1)

### `sort_structural_items` (100% protected)

**File**: `src/060_layer_core.rs`

- 🔒 ✓ **LayerFixed(3)**: Layer 3 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=0, out=3)**: Degree: in=0, out=3 (proven from graph)
- 🔒 ✓ **Root**: Root node (called by no other functions)
- 🔒 ✓ **SCC(107, size=1)**: Member of SCC 107 (size: 1)

### `scan_crate_paths` (80% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(0)**: Layer 0 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=0)**: Degree: in=1, out=0 (proven from graph)
- 🔒 ✓ **Leaf**: Leaf node (calls no other functions)
- 🔒 ✓ **SCC(201, size=1)**: Member of SCC 201 (size: 1)

### `check_action` (75% protected)

**File**: `src/191_agent_cli.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=3, out=1)**: Degree: in=3, out=1 (proven from graph)
- 🔒 ✓ **SCC(140, size=1)**: Member of SCC 140 (size: 1)

### `analyze_file_ordering` (75% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=3, out=7)**: Degree: in=3, out=7 (proven from graph)
- 🔒 ✓ **SCC(22, size=1)**: Member of SCC 22 (size: 1)

### `layer_constrained_sort` (75% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(5, size=1)**: Member of SCC 5 (size: 1)

### `extract_dependencies` (75% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(4)**: Layer 4 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(15, size=1)**: Member of SCC 15 (size: 1)

### `build_file_layers` (75% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=2, out=1)**: Degree: in=2, out=1 (proven from graph)
- 🔒 ✓ **SCC(9, size=1)**: Member of SCC 9 (size: 1)

### `resolve_required_layer_path` (75% protected)

**File**: `src/090_utilities.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(148, size=1)**: Member of SCC 148 (size: 1)

### `extract_rust_dependencies` (75% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(14, size=1)**: Member of SCC 14 (size: 1)

### `show_stats` (60% protected)

**File**: `src/191_agent_cli.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(231, size=1)**: Member of SCC 231 (size: 1)

### `generates_canonical_names_and_violations` (75% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(3)**: Layer 3 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(24, size=1)**: Member of SCC 24 (size: 1)

### `referenced_elsewhere` (60% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(206, size=1)**: Member of SCC 206 (size: 1)

### `query_function` (60% protected)

**File**: `src/191_agent_cli.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(233, size=1)**: Member of SCC 233 (size: 1)

### `resolve_module` (75% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=5, out=1)**: Degree: in=5, out=1 (proven from graph)
- 🔒 ✓ **SCC(11, size=1)**: Member of SCC 11 (size: 1)

### `gather_rust_files` (75% protected)

**File**: `src/070_layer_utilities.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(117, size=1)**: Member of SCC 117 (size: 1)

### `structural_cmp` (75% protected)

**File**: `src/060_layer_core.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=4)**: Degree: in=1, out=4 (proven from graph)
- 🔒 ✓ **SCC(106, size=1)**: Member of SCC 106 (size: 1)

### `run_analysis` (75% protected)

**File**: `src/070_layer_utilities.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=4)**: Degree: in=1, out=4 (proven from graph)
- 🔒 ✓ **SCC(119, size=1)**: Member of SCC 119 (size: 1)

### `build_dependency_map` (60% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(5)**: Layer 5 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(16, size=1)**: Member of SCC 16 (size: 1)

### `extract_julia_dependencies` (75% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(3)**: Layer 3 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=5)**: Degree: in=1, out=5 (proven from graph)
- 🔒 ✓ **SCC(13, size=1)**: Member of SCC 13 (size: 1)

### `validate_action` (75% protected)

**File**: `src/083_action_validator.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=5, out=2)**: Degree: in=5, out=2 (proven from graph)
- 🔒 ✓ **SCC(45, size=1)**: Member of SCC 45 (size: 1)

### `resolve_module_name` (75% protected)

**File**: `src/020_cluster_010.rs`

- 🔒 ✓ **LayerFixed(2)**: Layer 2 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=4, out=1)**: Degree: in=4, out=1 (proven from graph)
- 🔒 ✓ **SCC(12, size=1)**: Member of SCC 12 (size: 1)

### `structural_layer_value` (75% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=6, out=1)**: Degree: in=6, out=1 (proven from graph)
- 🔒 ✓ **SCC(61, size=1)**: Member of SCC 61 (size: 1)

### `detects_cycles` (75% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(3)**: Layer 3 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(23, size=1)**: Member of SCC 23 (size: 1)

### `cluster_target_path` (75% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(66, size=1)**: Member of SCC 66 (size: 1)

### `is_dead_code_candidate` (50% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(207, size=1)**: Member of SCC 207 (size: 1)

### `list_invariants` (60% protected)

**File**: `src/191_agent_cli.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(232, size=1)**: Member of SCC 232 (size: 1)

### `is_layer_violation` (60% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(50, size=1)**: Member of SCC 50 (size: 1)

### `collect_symbol_references` (75% protected)

**File**: `src/180_report.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=2)**: Degree: in=1, out=2 (proven from graph)
- 🔒 ✓ **SCC(202, size=1)**: Member of SCC 202 (size: 1)

### `compute_type_coupling` (50% protected)

**File**: `src/110_cohesion_analyzer.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(159, size=1)**: Member of SCC 159 (size: 1)

### `build_directory_entry_map` (75% protected)

**File**: `src/000_cluster_001.rs`

- 🔒 ✓ **LayerFixed(6)**: Layer 6 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=9)**: Degree: in=1, out=9 (proven from graph)
- 🔒 ✓ **SCC(17, size=1)**: Member of SCC 17 (size: 1)

### `topo_sort` (60% protected)

**File**: `src/010_cluster_008.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(54, size=1)**: Member of SCC 54 (size: 1)

### `generate_invariant_report` (60% protected)

**File**: `src/080_invariant_reporter.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(121, size=1)**: Member of SCC 121 (size: 1)

### `extract_calls_from_text` (60% protected)

**File**: `src/150_julia_parser.rs`

- 🔒 ✓ **LayerFixed(1)**: Layer 1 assignment is proven from call graph
- 🔒 ✓ **DegreeStable(in=1, out=1)**: Degree: in=1, out=1 (proven from graph)
- 🔒 ✓ **SCC(178, size=1)**: Member of SCC 178 (size: 1)

---

## Legend

- ✓ **PROVEN**: Mathematical certainty from graph topology
- ◆ **EMPIRICAL**: Observed across multiple paths (high confidence)
- ? **HEURISTIC**: Name-based guess (LOW CONFIDENCE - verify manually)

- 🔒 **Blocking**: Constraint mechanically enforced

