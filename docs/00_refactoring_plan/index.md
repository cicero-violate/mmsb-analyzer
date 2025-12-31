# Refactoring Plan

Generated: 2025-12-30 22:29:05

## Summary

Action: use this as the quick status snapshot for planning work.
Note: counts are derived from current analysis output.

- File/dir renames: 34
- Function moves: 17
- Orphaned functions: 76
- Clusters: 23

## Metrics

Action: monitor trends and regressions across runs.
Note: compare against baseline metrics when available.

- Directory cohesion: 1.00
- Ordering correctness: 100.0%
- Avg function cohesion: 0.76
- Rename ops needed: 34
- Function relocations suggested: 17

## Baseline Regression Warnings

Action: investigate any regressions before proceeding with refactors.
Note: only shown when a baseline metrics file exists.

- Avg function cohesion dropped from 0.77 to 0.76.
- Rename ops needed increased from 0 to 34.

## Phase 1: Correctness Blockers

Action: fix these first; they block correctness or builds.
Note: empty means no critical blockers detected.

- None.

## Phase 2: Cluster Extraction

Action: create the listed cluster files and move the grouped functions.
Note: use the batches below to keep changes small.

- Create cluster file `src/070_layer_utilities.rs` with 4 functions (cohesion 1.00)
- Create cluster file `src/010_cluster_008.rs` with 2 functions (cohesion 0.78)
- Create cluster file `src/000_cluster_001.rs` with 2 functions (cohesion 0.48)
- Create cluster file `src/083_action_validator.rs` with 2 functions (cohesion 0.40)
- Create cluster file `src/010_cluster_008.rs` with 6 functions (cohesion 0.14)
- Create cluster file `src/000_cluster_001.rs` with 2 functions (cohesion 0.12)

```bash
touch "src/070_layer_utilities.rs"
touch "src/010_cluster_008.rs"
touch "src/000_cluster_001.rs"
touch "src/083_action_validator.rs"
touch "src/010_cluster_008.rs"
touch "src/000_cluster_001.rs"
```

### Phase 2 Tips

Action: apply these guidelines while executing Phase 2 batches.
Note: these are advisory, not checklist items.

- Extract clusters as a unit; avoid splitting a cluster across files.
- Prefer creating new files before moving functions to keep diffs small.
- After each batch, update imports and run tests to lock in behavior.

### Phase 2 Batches

Action: execute batches in order and verify after each batch.
Note: each batch creates or fills a cluster file.

#### Batch 1: target `src/070_layer_utilities.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 1.00, 4 functions
- Move `gather_julia_files` from `src/000_cluster_001.rs`
- Move `generate_constraints` from `src/005_refactor_constraints.rs`
- Move `resolve_source_root` from `src/020_cluster_010.rs`
- Move `export_program_cfg_to_path` from `src/030_cluster_011.rs`
- Verification gate: `cargo test`

```bash
rg -n "gather_julia_files" "src/000_cluster_001.rs"
rg -n "gather_julia_files" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "generate_constraints" "src/005_refactor_constraints.rs"
rg -n "generate_constraints" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "resolve_source_root" "src/020_cluster_010.rs"
rg -n "resolve_source_root" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "export_program_cfg_to_path" "src/030_cluster_011.rs"
rg -n "export_program_cfg_to_path" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 2: target `src/010_cluster_008.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.78, 2 functions
- Move `test_detect_layer_violations_none` from `src/020_layer_inference.rs`
- Move `layer_prefix_value` from `src/050_cluster_006.rs`
- Verification gate: `cargo test`

```bash
rg -n "test_detect_layer_violations_none" "src/020_layer_inference.rs"
rg -n "test_detect_layer_violations_none" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "layer_prefix_value" "src/050_cluster_006.rs"
rg -n "layer_prefix_value" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 3: target `src/000_cluster_001.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.48, 2 functions
- Move `build_module_map` from `src/030_cluster_011.rs`
- Move `build_file_dag` from `src/030_cluster_011.rs`
- Verification gate: `cargo test`

```bash
rg -n "build_module_map" "src/030_cluster_011.rs"
rg -n "build_module_map" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "build_file_dag" "src/030_cluster_011.rs"
rg -n "build_file_dag" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 4: target `src/083_action_validator.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.40, 2 functions
- Move `test_check_move_allowed_blocking` from `src/005_refactor_constraints.rs`
- Move `test_check_move_allowed_non_blocking` from `src/005_refactor_constraints.rs`
- Verification gate: `cargo test`

```bash
rg -n "test_check_move_allowed_blocking" "src/005_refactor_constraints.rs"
rg -n "test_check_move_allowed_blocking" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "test_check_move_allowed_non_blocking" "src/005_refactor_constraints.rs"
rg -n "test_check_move_allowed_non_blocking" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 5: target `src/010_cluster_008.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.14, 6 functions
- Move `layer_constrained_sort` from `src/000_cluster_001.rs`
- Move `topo_sort_within` from `src/000_cluster_001.rs`
- Move `infer_layers` from `src/020_layer_inference.rs`
- Move `detect_layer_violations` from `src/020_layer_inference.rs`
- Move `test_layer_inference_simple_dag` from `src/020_layer_inference.rs`
- Move `test_layer_inference_diamond` from `src/020_layer_inference.rs`
- Verification gate: `cargo test`

```bash
rg -n "layer_constrained_sort" "src/000_cluster_001.rs"
rg -n "layer_constrained_sort" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "topo_sort_within" "src/000_cluster_001.rs"
rg -n "topo_sort_within" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "infer_layers" "src/020_layer_inference.rs"
rg -n "infer_layers" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "detect_layer_violations" "src/020_layer_inference.rs"
rg -n "detect_layer_violations" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "test_layer_inference_simple_dag" "src/020_layer_inference.rs"
rg -n "test_layer_inference_simple_dag" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "test_layer_inference_diamond" "src/020_layer_inference.rs"
rg -n "test_layer_inference_diamond" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

#### Batch 6: target `src/000_cluster_001.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Cluster cohesion 0.12, 2 functions
- Move `build_directory_dag` from `src/030_cluster_011.rs`
- Move `build_file_dependency_graph` from `src/030_cluster_011.rs`
- Verification gate: `cargo test`

```bash
rg -n "build_directory_dag" "src/030_cluster_011.rs"
rg -n "build_directory_dag" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
rg -n "build_file_dependency_graph" "src/030_cluster_011.rs"
rg -n "build_file_dependency_graph" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
```

## Phase 3: Structural Constraints

Action: resolve the layer violations by moving functions to target modules.
Note: follow batch order to avoid cascading dependency churn.

- `order_julia_files_by_dependency` from `src/020_cluster_010.rs` to `src/000_cluster_001.rs`: layer violation 020_cluster_010.rs -> 000_cluster_001.rs
- `run_analysis` from `src/070_layer_utilities.rs` to `src/000_cluster_001.rs`: layer violation 070_layer_utilities.rs -> 000_cluster_001.rs
- `test_check_move_allowed` from `src/083_action_validator.rs` to `src/005_refactor_constraints.rs`: layer violation 083_action_validator.rs -> 005_refactor_constraints.rs
- `sort_structural_items` from `src/060_layer_core.rs` to `src/010_cluster_008.rs`: layer violation 060_layer_core.rs -> 010_cluster_008.rs
- `structural_cmp` from `src/060_layer_core.rs` to `src/010_cluster_008.rs`: layer violation 060_layer_core.rs -> 010_cluster_008.rs
- `test_detect_layer_violations_none` from `src/020_layer_inference.rs` to `src/010_cluster_008.rs`: layer violation 020_layer_inference.rs -> 010_cluster_008.rs
- `gather_rust_files` from `src/070_layer_utilities.rs` to `src/020_cluster_010.rs`: layer violation 070_layer_utilities.rs -> 020_cluster_010.rs
- `test_query_allowed_actions` from `src/085_agent_conscience.rs` to `src/082_conscience_graph.rs`: layer violation 085_agent_conscience.rs -> 082_conscience_graph.rs
- `test_conscience_allows_valid_action` from `src/085_agent_conscience.rs` to `src/082_conscience_graph.rs`: layer violation 085_agent_conscience.rs -> 082_conscience_graph.rs
- `test_conscience_blocks_invalid_move` from `src/085_agent_conscience.rs` to `src/082_conscience_graph.rs`: layer violation 085_agent_conscience.rs -> 082_conscience_graph.rs

### Phase 3 Tips

Action: apply these guidelines while executing Phase 3 batches.
Note: these are advisory, not checklist items.

- Move lowest-layer helpers first; higher layers should depend on stable primitives.
- Keep moves small: move one function + update imports + rerun tests.
- If a target module is missing, create it before moving functions.
- Prefer consolidating shared utilities into their destination layer once.
- Avoid touching `_old/` unless explicitly refactoring archives.

### Phase 3 Batches

Action: execute batches in order and verify after each batch.
Note: each batch targets one destination module.

#### Batch 1: target `src/000_cluster_001.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Move `order_julia_files_by_dependency` from `src/020_cluster_010.rs` (impact 0, benefit/cost 0.50, touches 2 files; no external callers)
- Move `run_analysis` from `src/070_layer_utilities.rs` (impact 0, benefit/cost 0.25, touches 4 files; no external callers)
- Verification gate: `cargo test`

```bash
rg -n "order_julia_files_by_dependency" "src/020_cluster_010.rs"
rg -n "run_analysis" "src/070_layer_utilities.rs"
```

#### Batch 2: target `src/005_refactor_constraints.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Move `test_check_move_allowed` from `src/083_action_validator.rs` (impact 0, benefit/cost 0.50, touches 2 files; no external callers)
- Verification gate: `cargo test`

```bash
rg -n "test_check_move_allowed" "src/083_action_validator.rs"
```

#### Batch 3: target `src/010_cluster_008.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Move `sort_structural_items` from `src/060_layer_core.rs` (impact 0, benefit/cost 0.50, touches 2 files; no external callers)
- Move `structural_cmp` from `src/060_layer_core.rs` (impact 0, benefit/cost 0.50, touches 2 files; no external callers)
- Move `test_detect_layer_violations_none` from `src/020_layer_inference.rs` (impact 0, benefit/cost 0.50, touches 2 files; no external callers)
- Verification gate: `cargo test`

```bash
rg -n "sort_structural_items" "src/060_layer_core.rs"
rg -n "structural_cmp" "src/060_layer_core.rs"
rg -n "test_detect_layer_violations_none" "src/020_layer_inference.rs"
```

#### Batch 4: target `src/020_cluster_010.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Move `gather_rust_files` from `src/070_layer_utilities.rs` (impact 0, benefit/cost 0.50, touches 2 files; no external callers)
- Verification gate: `cargo test`

```bash
rg -n "gather_rust_files" "src/070_layer_utilities.rs"
```

#### Batch 5: target `src/082_conscience_graph.rs`

Action: move the listed functions into the target module.
Note: use the rg commands to locate definitions and callers.

- Move `test_query_allowed_actions` from `src/085_agent_conscience.rs` (impact 0, benefit/cost 0.50, touches 2 files; no external callers)
- Move `test_conscience_allows_valid_action` from `src/085_agent_conscience.rs` (impact 0, benefit/cost 0.33, touches 3 files; no external callers)
- Move `test_conscience_blocks_invalid_move` from `src/085_agent_conscience.rs` (impact 0, benefit/cost 0.33, touches 3 files; no external callers)
- Verification gate: `cargo test`

```bash
rg -n "test_query_allowed_actions" "src/085_agent_conscience.rs"
rg -n "test_conscience_allows_valid_action" "src/085_agent_conscience.rs"
rg -n "test_conscience_blocks_invalid_move" "src/085_agent_conscience.rs"
```

## Phase 4: Cohesion Improvements

Action: optional: improve cohesion by moving functions to better-fit modules.
Note: safe to defer unless you are actively refactoring.

- `detect_layer_violations` from `src/010_cluster_008.rs` to `-`: cohesion 0.35 below threshold 0.60 (impact 0.25)
- `test_strength_emoji` from `src/082_conscience_graph.rs` to `-`: cohesion 0.35 below threshold 0.60 (impact 0.25)
- `build_directory_entry_map` from `src/000_cluster_001.rs` to `-`: cohesion 0.49 below threshold 0.60 (impact 0.11)
- `cluster_target_path` from `src/010_cluster_008.rs` to `-`: cohesion 0.43 below threshold 0.60 (impact 0.17)
- `layer_constrained_sort` from `src/000_cluster_001.rs` to `-`: cohesion 0.57 below threshold 0.60 (impact 0.03)
- `test_check_move_allowed_blocking` from `src/005_refactor_constraints.rs` to `-`: cohesion 0.50 below threshold 0.60 (impact 0.10)
- `test_check_move_allowed_non_blocking` from `src/005_refactor_constraints.rs` to `-`: cohesion 0.50 below threshold 0.60 (impact 0.10)

## Phase 5: Ordering & Renames

Action: optional: rename files to match ordering conventions.
Note: update module paths and imports after renames.

- [Rust] `MMSB/tools/mmsb-analyzer/src/010_cluster_008.rs` -> `MMSB/tools/mmsb-analyzer/src/030_cluster_008.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/020_cluster_010.rs` -> `MMSB/tools/mmsb-analyzer/src/050_cluster_010.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/030_cluster_011.rs` -> `MMSB/tools/mmsb-analyzer/src/070_cluster_011.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/040_structural_detector.rs` -> `MMSB/tools/mmsb-analyzer/src/100_structural_detector.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/050_semantic_detector.rs` -> `MMSB/tools/mmsb-analyzer/src/120_semantic_detector.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/070_invariant_integrator.rs` -> `MMSB/tools/mmsb-analyzer/src/150_invariant_integrator.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/070_layer_utilities.rs` -> `MMSB/tools/mmsb-analyzer/src/160_layer_utilities.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/090_utilities.rs` -> `MMSB/tools/mmsb-analyzer/src/220_utilities.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/190_main.rs` -> `MMSB/tools/mmsb-analyzer/src/320_main.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/000_invariant_types.rs` -> `MMSB/tools/mmsb-analyzer/src/010_invariant_types.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/005_refactor_constraints.rs` -> `MMSB/tools/mmsb-analyzer/src/020_refactor_constraints.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/010_scc_compressor.rs` -> `MMSB/tools/mmsb-analyzer/src/040_scc_compressor.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/020_layer_inference.rs` -> `MMSB/tools/mmsb-analyzer/src/060_layer_inference.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/030_fixpoint_solver.rs` -> `MMSB/tools/mmsb-analyzer/src/080_fixpoint_solver.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/040_dependency.rs` -> `MMSB/tools/mmsb-analyzer/src/090_dependency.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/050_cluster_006.rs` -> `MMSB/tools/mmsb-analyzer/src/110_cluster_006.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/060_layer_core.rs` -> `MMSB/tools/mmsb-analyzer/src/130_layer_core.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/060_path_detector.rs` -> `MMSB/tools/mmsb-analyzer/src/140_path_detector.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/080_file_gathering.rs` -> `MMSB/tools/mmsb-analyzer/src/170_file_gathering.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/080_invariant_reporter.rs` -> `MMSB/tools/mmsb-analyzer/src/180_invariant_reporter.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/082_conscience_graph.rs` -> `MMSB/tools/mmsb-analyzer/src/190_conscience_graph.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/083_action_validator.rs` -> `MMSB/tools/mmsb-analyzer/src/200_action_validator.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/085_agent_conscience.rs` -> `MMSB/tools/mmsb-analyzer/src/210_agent_conscience.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/100_types.rs` -> `MMSB/tools/mmsb-analyzer/src/230_types.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/110_cohesion_analyzer.rs` -> `MMSB/tools/mmsb-analyzer/src/240_cohesion_analyzer.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/120_directory_analyzer.rs` -> `MMSB/tools/mmsb-analyzer/src/250_directory_analyzer.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/130_control_flow.rs` -> `MMSB/tools/mmsb-analyzer/src/260_control_flow.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/140_file_ordering.rs` -> `MMSB/tools/mmsb-analyzer/src/270_file_ordering.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/150_julia_parser.rs` -> `MMSB/tools/mmsb-analyzer/src/280_julia_parser.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/160_rust_parser.rs` -> `MMSB/tools/mmsb-analyzer/src/290_rust_parser.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/170_dot_exporter.rs` -> `MMSB/tools/mmsb-analyzer/src/300_dot_exporter.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/180_report.rs` -> `MMSB/tools/mmsb-analyzer/src/310_report.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/191_agent_cli.rs` -> `MMSB/tools/mmsb-analyzer/src/330_agent_cli.rs`
- [Rust] `MMSB/tools/mmsb-analyzer/src/200_lib.rs` -> `MMSB/tools/mmsb-analyzer/src/340_lib.rs`

```bash
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/010_cluster_008.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/030_cluster_008.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/020_cluster_010.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/050_cluster_010.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/030_cluster_011.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/070_cluster_011.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/040_structural_detector.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/100_structural_detector.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/050_semantic_detector.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/120_semantic_detector.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/070_invariant_integrator.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/150_invariant_integrator.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/070_layer_utilities.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/160_layer_utilities.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/090_utilities.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/220_utilities.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/190_main.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/320_main.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/000_invariant_types.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/010_invariant_types.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/005_refactor_constraints.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/020_refactor_constraints.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/010_scc_compressor.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/040_scc_compressor.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/020_layer_inference.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/060_layer_inference.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/030_fixpoint_solver.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/080_fixpoint_solver.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/040_dependency.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/090_dependency.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/050_cluster_006.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/110_cluster_006.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/060_layer_core.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/130_layer_core.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/060_path_detector.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/140_path_detector.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/080_file_gathering.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/170_file_gathering.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/080_invariant_reporter.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/180_invariant_reporter.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/082_conscience_graph.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/190_conscience_graph.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/083_action_validator.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/200_action_validator.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/085_agent_conscience.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/210_agent_conscience.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/100_types.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/230_types.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/110_cohesion_analyzer.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/240_cohesion_analyzer.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/120_directory_analyzer.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/250_directory_analyzer.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/130_control_flow.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/260_control_flow.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/140_file_ordering.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/270_file_ordering.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/150_julia_parser.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/280_julia_parser.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/160_rust_parser.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/290_rust_parser.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/170_dot_exporter.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/300_dot_exporter.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/180_report.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/310_report.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/191_agent_cli.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/330_agent_cli.rs"
git mv "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/200_lib.rs" "/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer/src/340_lib.rs"
```

## Orphaned Functions (Review Only)

Action: review each item for expected usage. Delete only if it also appears under "Delete Candidates (Orphaned + Dead Code)".
Note: excludes public symbols referenced by other modules and entry points. Delete candidates require dead_code warnings.

- `escape_dot` in `src/000_cluster_001.rs`
- `test_confidence_from_strength` in `src/000_invariant_types.rs`
- `test_is_blocking` in `src/000_invariant_types.rs`
- `test_stats_calculation` in `src/000_invariant_types.rs`
- `test_constraint_is_blocking` in `src/005_refactor_constraints.rs`
- `test_scc_compression_dag` in `src/010_scc_compressor.rs`
- `test_scc_compression_cycle` in `src/010_scc_compressor.rs`
- `test_scc_compression_mixed` in `src/010_scc_compressor.rs`
- `test_symbolic_abstraction_merge` in `src/030_fixpoint_solver.rs`
- `test_detect_leaf_root` in `src/040_structural_detector.rs`
- `test_detect_degree_stable` in `src/040_structural_detector.rs`
- `test_all_structural_invariants_proven` in `src/040_structural_detector.rs`
- `make_function` in `src/050_semantic_detector.rs`
- `test_detect_type_stable` in `src/050_semantic_detector.rs`
- `test_detect_pure_function_heuristic` in `src/050_semantic_detector.rs`
- `test_detect_idempotent_heuristic` in `src/050_semantic_detector.rs`
- `test_no_pure_for_mutable` in `src/050_semantic_detector.rs`
- `test_path_detector_simple` in `src/060_path_detector.rs`
- `test_path_detector_diamond` in `src/060_path_detector.rs`
- `test_extract_facts_from_path` in `src/060_path_detector.rs`
- `test_path_stats` in `src/060_path_detector.rs`
- `export_constraints_json` in `src/080_invariant_reporter.rs`
- `strength_emoji` in `src/082_conscience_graph.rs`
- `kind_name` in `src/082_conscience_graph.rs`
- `matches_function` in `src/083_action_validator.rs`
- `test_extract_layer` in `src/083_action_validator.rs`
- `test_conscience_stats` in `src/085_agent_conscience.rs`
- `build_function_layers` in `src/110_cohesion_analyzer.rs`
- `build_type_maps` in `src/110_cohesion_analyzer.rs`
- `determine_status` in `src/110_cohesion_analyzer.rs`
- `suggest_file` in `src/110_cohesion_analyzer.rs`
- `compute_cluster_cohesion` in `src/110_cohesion_analyzer.rs`
- `suggest_cluster_file` in `src/110_cohesion_analyzer.rs`
- `is_source_file` in `src/120_directory_analyzer.rs`
- `should_skip_path` in `src/120_directory_analyzer.rs`
- `sanitize_identifier` in `src/130_control_flow.rs`
- `parallel_build_file_dag` in `src/140_file_ordering.rs`
- `slugify_relative` in `src/150_julia_parser.rs`
- `resolve_julia_binary` in `src/150_julia_parser.rs`
- `find_julia_project_dir` in `src/150_julia_parser.rs`
- `parse_module_name` in `src/150_julia_parser.rs`
- `parse_struct_name` in `src/150_julia_parser.rs`
- `relativize_path` in `src/150_julia_parser.rs`
- `paren_balance` in `src/150_julia_parser.rs`
- `relativize_path` in `src/160_rust_parser.rs`
- `display_path` in `src/180_report.rs`
- `placement_status_label` in `src/180_report.rs`
- `placement_status_notes` in `src/180_report.rs`
- `collect_rename_items` in `src/180_report.rs`
- `collect_utility_candidates` in `src/180_report.rs`
- `directory_moves_to_plan` in `src/180_report.rs`
- `write_priority_section` in `src/180_report.rs`
- `write_structural_tips` in `src/180_report.rs`
- `write_cluster_tips` in `src/180_report.rs`
- `sort_plan_items` in `src/180_report.rs`
- `sort_cluster_items` in `src/180_report.rs`
- `parse_dead_code_warnings` in `src/180_report.rs`
- `load_report_config` in `src/180_report.rs`
- `load_baseline_metrics` in `src/180_report.rs`
- `baseline_deltas` in `src/180_report.rs`
- `write_baseline_metrics` in `src/180_report.rs`
- `slugify_path` in `src/180_report.rs`
- `render_mermaid_graph` in `src/180_report.rs`
- `compute_ordering_correctness` in `src/180_report.rs`
- `compute_directory_cohesion` in `src/180_report.rs`
- `prefix_key_from_path` in `src/180_report.rs`
- `slugify_key` in `src/180_report.rs`
- `group_key_cmp` in `src/180_report.rs`
- `function_bucket_label` in `src/180_report.rs`
- `slugify_file_path` in `src/180_report.rs`
- `language_label` in `src/180_report.rs`
- `visibility_label` in `src/180_report.rs`
- `short_signature` in `src/180_report.rs`
- `normalize_use_stmt` in `src/180_report.rs`
- `sanitize_mermaid_id` in `src/180_report.rs`
- `sanitize_mermaid_label` in `src/180_report.rs`

## Delete Candidates (Orphaned + Dead Code)

Action: consider removal after confirming behavior and running tests.
Note: derived from orphaned list plus compiler dead_code warnings.

- `matches_function` in `src/083_action_validator.rs`

## Suggested New Files (Clusters)

Action: consider creating these files to improve cohesion.
Note: suggestions are heuristic and should be validated.

- cohesion 1.00, suggested `src/070_layer_utilities.rs`
  - src/000_cluster_001.rs::gather_julia_files, src/005_refactor_constraints.rs::generate_constraints, src/020_cluster_010.rs::resolve_source_root, src/030_cluster_011.rs::export_program_cfg_to_path, src/070_layer_utilities.rs::resolve_source_root, src/070_layer_utilities.rs::allow_analysis_dir, src/070_layer_utilities.rs::gather_rust_files, src/070_layer_utilities.rs::main, src/070_layer_utilities.rs::run_analysis
- cohesion 1.00, suggested `src/080_invariant_reporter.rs`
  - src/080_invariant_reporter.rs::generate_invariant_report, src/080_invariant_reporter.rs::export_json, src/080_invariant_reporter.rs::test_generate_report
- cohesion 1.00, suggested `src/090_utilities.rs`
  - src/090_utilities.rs::compress_path, src/090_utilities.rs::collect_directory_files, src/090_utilities.rs::path_common_prefix_len, src/090_utilities.rs::resolve_required_layer_path, src/090_utilities.rs::compute_move_metrics, src/090_utilities.rs::collect_move_items, src/090_utilities.rs::write_structural_batches, src/090_utilities.rs::write_cluster_batches
- cohesion 1.00, suggested `src/110_cohesion_analyzer.rs`
  - src/110_cohesion_analyzer.rs::collect_functions, src/110_cohesion_analyzer.rs::build_call_edges, src/110_cohesion_analyzer.rs::build_name_map
- cohesion 0.22, suggested `src/020_cluster_010.rs`
  - src/000_cluster_001.rs::collect_julia_dependencies, src/020_cluster_010.rs::normalize_module_name, src/020_cluster_010.rs::build_module_root_map, src/020_cluster_010.rs::extract_julia_dependencies, src/020_cluster_010.rs::extract_dependencies
- cohesion 0.00, suggested `src/180_report.rs`
  - src/180_report.rs::load_cargo_warnings, src/180_report.rs::collect_symbol_references, src/180_report.rs::is_public_function, src/180_report.rs::is_entrypoint_main, src/180_report.rs::is_dead_code_candidate
- cohesion 1.00, suggested `src/110_cohesion_analyzer.rs`
  - src/110_cohesion_analyzer.rs::build_call_analysis, src/110_cohesion_analyzer.rs::compute_type_coupling, src/110_cohesion_analyzer.rs::extract_identifiers
- cohesion 0.40, suggested `src/085_agent_conscience.rs`
  - src/082_conscience_graph.rs::make_test_invariant, src/085_agent_conscience.rs::test_conscience_blocks_invalid_move, src/085_agent_conscience.rs::test_query_allowed_actions
- cohesion 0.50, suggested `src/020_cluster_010.rs`
  - src/020_cluster_010.rs::resolve_module, src/020_cluster_010.rs::order_julia_files_by_dependency, src/020_cluster_010.rs::resolve_module_name
- cohesion 1.00, suggested `src/030_fixpoint_solver.rs`
  - src/030_fixpoint_solver.rs::propagate_to_fixpoint, src/030_fixpoint_solver.rs::test_fixpoint_simple, src/030_fixpoint_solver.rs::test_fixpoint_convergence
- cohesion 1.00, suggested `src/150_julia_parser.rs`
  - src/150_julia_parser.rs::extract_calls_from_lines, src/150_julia_parser.rs::extract_calls_from_text, src/150_julia_parser.rs::is_reserved
- cohesion 0.50, suggested `src/083_action_validator.rs`
  - src/005_refactor_constraints.rs::check_move_allowed, src/083_action_validator.rs::extract_layer, src/083_action_validator.rs::test_check_move_allowed
- cohesion 0.29, suggested `src/180_report.rs`
  - src/180_report.rs::parse_use_symbols, src/180_report.rs::scan_crate_paths, src/180_report.rs::path_matches, src/180_report.rs::referenced_elsewhere, src/180_report.rs::filter_orphaned
- cohesion 0.50, suggested `src/191_agent_cli.rs`
  - src/085_agent_conscience.rs::test_conscience_allows_valid_action, src/191_agent_cli.rs::run_agent_cli, src/191_agent_cli.rs::query_function, src/191_agent_cli.rs::show_stats, src/191_agent_cli.rs::load_invariants, src/191_agent_cli.rs::test_load_invariants_empty
- cohesion 1.00, suggested `src/160_rust_parser.rs`
  - src/160_rust_parser.rs::expr_snippet, src/160_rust_parser.rs::pat_snippet, src/160_rust_parser.rs::truncate_label
- cohesion 1.00, suggested `src/070_invariant_integrator.rs`
  - src/070_invariant_integrator.rs::make_simple_analysis, src/070_invariant_integrator.rs::test_invariant_detector_creation, src/070_invariant_integrator.rs::test_detect_all
- cohesion 1.00, suggested `src/050_cluster_006.rs`
  - src/050_cluster_006.rs::strip_numeric_prefix, src/050_cluster_006.rs::generate_canonical_name, src/050_cluster_006.rs::collect_directory_moves
- cohesion 0.12, suggested `src/000_cluster_001.rs`
  - src/000_cluster_001.rs::generates_canonical_names_and_violations, src/000_cluster_001.rs::rust_entry_paths, src/000_cluster_001.rs::collect_rust_dependencies, src/000_cluster_001.rs::build_file_layers, src/000_cluster_001.rs::topological_sort, src/000_cluster_001.rs::ordered_by_name, src/000_cluster_001.rs::build_entries, src/000_cluster_001.rs::test_generates_canonical_names_and_violations, src/030_cluster_011.rs::build_directory_dag, src/030_cluster_011.rs::build_file_dependency_graph
- cohesion 0.14, suggested `src/010_cluster_008.rs`
  - src/000_cluster_001.rs::layer_constrained_sort, src/000_cluster_001.rs::topo_sort_within, src/010_cluster_008.rs::is_layer_violation, src/010_cluster_008.rs::compare_dir_layers, src/010_cluster_008.rs::compare_path_components, src/010_cluster_008.rs::detect_layer_violation, src/010_cluster_008.rs::parse_cluster_members, src/010_cluster_008.rs::is_core_module_path, src/020_layer_inference.rs::infer_layers, src/020_layer_inference.rs::detect_layer_violations, src/020_layer_inference.rs::test_layer_inference_simple_dag, src/020_layer_inference.rs::test_layer_inference_diamond
- cohesion 0.00, suggested `src/020_cluster_010.rs`
  - src/020_cluster_010.rs::contains_tools, src/020_cluster_010.rs::extract_rust_dependencies, src/020_cluster_010.rs::build_dependency_map
- cohesion 0.48, suggested `src/000_cluster_001.rs`
  - src/000_cluster_001.rs::build_directory_entry_map, src/000_cluster_001.rs::collect_naming_warnings, src/000_cluster_001.rs::temp_dir, src/000_cluster_001.rs::detects_cycles, src/000_cluster_001.rs::topo_sort_orders_dependencies, src/000_cluster_001.rs::detect_layer, src/000_cluster_001.rs::order_rust_files_by_dependency, src/000_cluster_001.rs::analyze_file_ordering, src/000_cluster_001.rs::naming_score_for_file, src/000_cluster_001.rs::detect_cycles, src/000_cluster_001.rs::detect_violations, src/000_cluster_001.rs::test_detects_cycles, src/030_cluster_011.rs::build_module_map, src/030_cluster_011.rs::build_file_dag
- cohesion 0.40, suggested `src/083_action_validator.rs`
  - src/005_refactor_constraints.rs::test_check_move_allowed_blocking, src/005_refactor_constraints.rs::test_check_move_allowed_non_blocking, src/083_action_validator.rs::validate_action, src/083_action_validator.rs::test_validate_no_move_constraint, src/083_action_validator.rs::test_validate_layer_fixed_constraint, src/083_action_validator.rs::test_validate_preserve_signature, src/083_action_validator.rs::test_validate_allowed_action
- cohesion 0.78, suggested `src/010_cluster_008.rs`
  - src/010_cluster_008.rs::build_result, src/010_cluster_008.rs::adjacency_from_edges, src/010_cluster_008.rs::topo_sort, src/010_cluster_008.rs::layer_rank_map, src/010_cluster_008.rs::insert_sorted, src/010_cluster_008.rs::is_mmsb_main, src/010_cluster_008.rs::layer_prefix_value, src/010_cluster_008.rs::layer_adheres, src/010_cluster_008.rs::detect_layer_violations, src/010_cluster_008.rs::cluster_target_path, src/010_cluster_008.rs::collect_cluster_plans, src/020_layer_inference.rs::test_detect_layer_violations_none, src/050_cluster_006.rs::layer_prefix_value

## Utility Module Candidates

Action: consider consolidating these into a shared utilities module.
Note: candidates are based on cross-file usage frequency.

- None detected.

## Naming Warnings

Action: rename files if the suggested name improves ordering clarity.
Note: `_old` paths are excluded from naming warnings.

- File `MMSB/tools/mmsb-analyzer/src/000_main.jl` has naming score 60; consider renaming to `020_main.jl`.
- File `MMSB/tools/mmsb-analyzer/src/005_refactor_constraints.rs` has naming score 60; consider renaming to `030_refactor_constraints.rs`.
- File `MMSB/tools/mmsb-analyzer/src/010_MMSBAnalyzerJulia.jl` has naming score 40; consider renaming to `040_MMSBAnalyzerJulia.jl`.
- File `MMSB/tools/mmsb-analyzer/src/010_cluster_008.rs` has naming score 60; consider renaming to `050_cluster_008.rs`.
- File `MMSB/tools/mmsb-analyzer/src/020_ast_cfg.jl` has naming score 60; consider renaming to `070_ast_cfg.jl`.
- File `MMSB/tools/mmsb-analyzer/src/020_cluster_010.rs` has naming score 60; consider renaming to `080_cluster_010.rs`.
- File `MMSB/tools/mmsb-analyzer/src/030_cluster_011.rs` has naming score 60; consider renaming to `100_cluster_011.rs`.
- File `MMSB/tools/mmsb-analyzer/src/030_fixpoint_solver.rs` has naming score 60; consider renaming to `110_fixpoint_solver.rs`.
- File `MMSB/tools/mmsb-analyzer/src/040_build_model.jl` has naming score 60; consider renaming to `130_build_model.jl`.
- File `MMSB/tools/mmsb-analyzer/src/040_dependency.rs` has naming score 60; consider renaming to `140_dependency.rs`.
- File `MMSB/tools/mmsb-analyzer/src/040_structural_detector.rs` has naming score 60; consider renaming to `150_structural_detector.rs`.
- File `MMSB/tools/mmsb-analyzer/src/050_cluster_006.rs` has naming score 60; consider renaming to `160_cluster_006.rs`.
- File `MMSB/tools/mmsb-analyzer/src/050_semantic_detector.rs` has naming score 60; consider renaming to `170_semantic_detector.rs`.
- File `MMSB/tools/mmsb-analyzer/src/060_layer_core.rs` has naming score 60; consider renaming to `180_layer_core.rs`.
- File `MMSB/tools/mmsb-analyzer/src/070_invariant_integrator.rs` has naming score 60; consider renaming to `200_invariant_integrator.rs`.
- File `MMSB/tools/mmsb-analyzer/src/070_layer_utilities.rs` has naming score 60; consider renaming to `210_layer_utilities.rs`.
- File `MMSB/tools/mmsb-analyzer/src/080_invariant_reporter.rs` has naming score 60; consider renaming to `230_invariant_reporter.rs`.
- File `MMSB/tools/mmsb-analyzer/src/082_conscience_graph.rs` has naming score 60; consider renaming to `240_conscience_graph.rs`.
- File `MMSB/tools/mmsb-analyzer/src/090_utilities.rs` has naming score 60; consider renaming to `270_utilities.rs`.
- File `MMSB/tools/mmsb-analyzer/src/100_types.rs` has naming score 60; consider renaming to `280_types.rs`.
- File `MMSB/tools/mmsb-analyzer/src/110_cohesion_analyzer.rs` has naming score 60; consider renaming to `290_cohesion_analyzer.rs`.
- File `MMSB/tools/mmsb-analyzer/src/120_directory_analyzer.rs` has naming score 60; consider renaming to `300_directory_analyzer.rs`.
- File `MMSB/tools/mmsb-analyzer/src/130_control_flow.rs` has naming score 60; consider renaming to `310_control_flow.rs`.
- File `MMSB/tools/mmsb-analyzer/src/160_rust_parser.rs` has naming score 60; consider renaming to `340_rust_parser.rs`.
- File `MMSB/tools/mmsb-analyzer/src/170_dot_exporter.rs` has naming score 60; consider renaming to `350_dot_exporter.rs`.
- File `MMSB/tools/mmsb-analyzer/src/180_report.rs` has naming score 60; consider renaming to `360_report.rs`.
- File `MMSB/tools/mmsb-analyzer/src/191_agent_cli.rs` has naming score 60; consider renaming to `380_agent_cli.rs`.
- File `MMSB/tools/mmsb-analyzer/src/200_lib.rs` has naming score 60; consider renaming to `390_lib.rs`.

## Size Warnings

Action: consider extracting helpers to reduce file size.
Note: thresholds come from report configuration.

- Directory `MMSB/tools/mmsb-analyzer/src` has 40 files; consider splitting into submodules.
- File `MMSB/tools/mmsb-analyzer/src/000_cluster_001.rs` has 1032 lines; consider extracting helpers.
- File `MMSB/tools/mmsb-analyzer/src/180_report.rs` has 2534 lines; consider extracting helpers.

## Cargo Warnings

Action: address compiler warnings before major refactors.
Note: captured from cargo check/test outputs.

```text
Checking mmsb-analyzer v0.1.0 (/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer)
warning: unused import: `petgraph::algo::is_cyclic_directed`
  --> src/040_structural_detector.rs:11:5
   |
11 | use petgraph::algo::is_cyclic_directed;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::collections::HashSet`
  --> src/040_structural_detector.rs:15:5
   |
15 | use std::collections::HashSet;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
  --> src/050_semantic_detector.rs:10:5
   |
10 | use std::collections::HashSet;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `HashMap`
  --> src/060_path_detector.rs:11:24
   |
11 | use std::collections::{HashMap, HashSet};
   |                        ^^^^^^^

warning: unused import: `std::io::Write`
 --> src/082_conscience_graph.rs:7:5
  |
7 | use std::io::Write;
  |     ^^^^^^^^^^^^^^

warning: unused variable: `original_move_count`
    --> src/180_report.rs:1617:13
     |
1617 |         let original_move_count = moves.len();
     |             ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_original_move_count`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `matches_function` is never used
  --> src/083_action_validator.rs:80:4
   |
80 | fn matches_function(action_name: &str, constraint: &RefactorConstraint) -> bool {
   |    ^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `mmsb-analyzer` (lib) generated 7 warnings (run `cargo fix --lib -p mmsb-analyzer` to apply 6 suggestions)
warning: method `get_blocking_invariants` is never used
   --> src/000_invariant_types.rs:324:12
    |
288 | impl InvariantAnalysisResult {
    | ---------------------------- method in this implementation
...
324 |     pub fn get_blocking_invariants(&self, target: &str) -> Vec<&Invariant> {
    |            ^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: methods `target`, `strength`, and `is_blocking` are never used
   --> src/005_refactor_constraints.rs:95:12
    |
 93 | impl RefactorConstraint {
    | ----------------------- methods in this implementation
 94 |     /// Get the target of this constraint
 95 |     pub fn target(&self) -> &str {
    |            ^^^^^^
...
108 |     pub fn strength(&self) -> &InvariantStrength {
    |            ^^^^^^^^
...
121 |     pub fn is_blocking(&self) -> bool {
    |            ^^^^^^^^^^^

warning: function `check_move_allowed` is never used
   --> src/005_refactor_constraints.rs:203:8
    |
203 | pub fn check_move_allowed(
    |        ^^^^^^^^^^^^^^^^^^

warning: fields `original_graph`, `node_to_scc`, and `scc_to_node` are never read
  --> src/010_scc_compressor.rs:16:9
   |
14 | pub struct SccCompression {
   |            -------------- fields in this struct
15 |     /// Original graph
16 |     pub original_graph: DiGraph<String, ()>,
   |         ^^^^^^^^^^^^^^
...
22 |     pub node_to_scc: HashMap<NodeIndex, usize>,
   |         ^^^^^^^^^^^
...
25 |     pub scc_to_node: HashMap<usize, NodeIndex>,
   |         ^^^^^^^^^^^
   |
   = note: `SccCompression` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: methods `is_dag`, `get_scc_for_node`, `get_scc_members`, `count_trivial_sccs`, `count_nontrivial_sccs`, and `stats` are never used
   --> src/010_scc_compressor.rs:94:12
    |
 31 | impl SccCompression {
    | ------------------- methods in this implementation
...
 94 |     pub fn is_dag(&self) -> bool {
    |            ^^^^^^
...
 99 |     pub fn get_scc_for_node(&self, node_name: &str) -> Option<usize> {
    |            ^^^^^^^^^^^^^^^^
...
110 |     pub fn get_scc_members(&self, scc_id: usize) -> Vec<String> {
    |            ^^^^^^^^^^^^^^^
...
121 |     pub fn count_trivial_sccs(&self) -> usize {
    |            ^^^^^^^^^^^^^^^^^^
...
126 |     pub fn count_nontrivial_sccs(&self) -> usize {
    |            ^^^^^^^^^^^^^^^^^^^^^
...
131 |     pub fn stats(&self) -> SccStats {
    |            ^^^^^

warning: struct `SccStats` is never constructed
   --> src/010_scc_compressor.rs:144:12
    |
144 | pub struct SccStats {
    |            ^^^^^^^^

warning: struct `SymbolicAbstraction` is never constructed
  --> src/030_fixpoint_solver.rs:13:12
   |
13 | pub struct SymbolicAbstraction {
   |            ^^^^^^^^^^^^^^^^^^^

warning: associated items `new`, `approx_eq`, and `merge` are never used
  --> src/030_fixpoint_solver.rs:31:12
   |
30 | impl SymbolicAbstraction {
   | ------------------------ associated items in this implementation
31 |     pub fn new() -> Self {
   |            ^^^
...
42 |     pub fn approx_eq(&self, other: &Self) -> bool {
   |            ^^^^^^^^^
...
51 |     pub fn merge(&mut self, other: &Self) {
   |            ^^^^^

warning: struct `FixpointResult` is never constructed
  --> src/030_fixpoint_solver.rs:85:12
   |
85 | pub struct FixpointResult {
   |            ^^^^^^^^^^^^^^

warning: function `propagate_to_fixpoint` is never used
   --> src/030_fixpoint_solver.rs:108:8
    |
108 | pub fn propagate_to_fixpoint(
    |        ^^^^^^^^^^^^^^^^^^^^^

warning: method `get_stats` is never used
   --> src/060_path_detector.rs:179:12
    |
 19 | impl PathDetector {
    | ----------------- method in this implementation
...
179 |     pub fn get_stats(&self) -> PathStats {
    |            ^^^^^^^^^

warning: struct `PathStats` is never constructed
   --> src/060_path_detector.rs:192:12
    |
192 | pub struct PathStats {
    |            ^^^^^^^^^

warning: method `check_violations` is never used
   --> src/070_invariant_integrator.rs:144:12
    |
 21 | impl<'a> InvariantDetector<'a> {
    | ------------------------------ method in this implementation
...
144 |     pub fn check_violations(&self, invariants: &[Invariant]) -> Vec<InvariantViolation> {
    |            ^^^^^^^^^^^^^^^^

warning: function `generate_conscience_stats` is never used
   --> src/082_conscience_graph.rs:141:8
    |
141 | pub fn generate_conscience_stats(invariants: &[Invariant]) -> ConscienceStats {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `ConscienceStats` is never constructed
   --> src/082_conscience_graph.rs:183:12
    |
183 | pub struct ConscienceStats {
    |            ^^^^^^^^^^^^^^^

warning: function `matches_function` is never used
  --> src/083_action_validator.rs:80:4
   |
80 | fn matches_function(action_name: &str, constraint: &RefactorConstraint) -> bool {
   |    ^^^^^^^^^^^^^^^^

warning: associated items `load` and `export_json` are never used
   --> src/085_agent_conscience.rs:43:12
    |
 28 | impl AgentConscience {
    | -------------------- associated items in this implementation
...
 43 |     pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    |            ^^^^
...
144 |     pub fn export_json(&self, path: &Path) -> std::io::Result<()> {
    |            ^^^^^^^^^^^

warning: `mmsb-analyzer` (bin "mmsb-analyzer") generated 23 warnings (6 duplicates)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.00s
   Compiling mmsb-analyzer v0.1.0 (/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer)
warning: unused import: `std::collections::HashSet`
   --> src/005_refactor_constraints.rs:243:9
    |
243 |     use std::collections::HashSet;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `petgraph::algo::is_cyclic_directed`
  --> src/040_structural_detector.rs:11:5
   |
11 | use petgraph::algo::is_cyclic_directed;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
  --> src/040_structural_detector.rs:15:5
   |
15 | use std::collections::HashSet;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
  --> src/050_semantic_detector.rs:10:5
   |
10 | use std::collections::HashSet;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `HashMap`
  --> src/060_path_detector.rs:11:24
   |
11 | use std::collections::{HashMap, HashSet};
   |                        ^^^^^^^

warning: unused import: `std::io::Write`
 --> src/082_conscience_graph.rs:7:5
  |
7 | use std::io::Write;
  |     ^^^^^^^^^^^^^^

warning: unused import: `petgraph::algo::is_cyclic_directed`
  --> src/040_structural_detector.rs:11:5
   |
11 | use petgraph::algo::is_cyclic_directed;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `original_move_count`
    --> src/180_report.rs:1617:13
     |
1617 |         let original_move_count = moves.len();
     |             ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_original_move_count`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `matches_function` is never used
  --> src/083_action_validator.rs:80:4
   |
80 | fn matches_function(action_name: &str, constraint: &RefactorConstraint) -> bool {
   |    ^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: comparison is useless due to type limits
   --> src/060_path_detector.rs:217:17
    |
217 |         assert!(invariants.len() >= 0);
    |                 ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(unused_comparisons)]` on by default

warning: `mmsb-analyzer` (lib test) generated 9 warnings (run `cargo fix --lib -p mmsb-analyzer --tests` to apply 7 suggestions)
warning: `mmsb-analyzer` (lib) generated 7 warnings (6 duplicates) (run `cargo fix --lib -p mmsb-analyzer` to apply 1 suggestion)
warning: method `get_blocking_invariants` is never used
   --> src/000_invariant_types.rs:324:12
    |
288 | impl InvariantAnalysisResult {
    | ---------------------------- method in this implementation
...
324 |     pub fn get_blocking_invariants(&self, target: &str) -> Vec<&Invariant> {
    |            ^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: fields `original_graph`, `node_to_scc`, and `scc_to_node` are never read
  --> src/010_scc_compressor.rs:16:9
   |
14 | pub struct SccCompression {
   |            -------------- fields in this struct
15 |     /// Original graph
16 |     pub original_graph: DiGraph<String, ()>,
   |         ^^^^^^^^^^^^^^
...
22 |     pub node_to_scc: HashMap<NodeIndex, usize>,
   |         ^^^^^^^^^^^
...
25 |     pub scc_to_node: HashMap<usize, NodeIndex>,
   |         ^^^^^^^^^^^
   |
   = note: `SccCompression` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: methods `get_scc_for_node`, `get_scc_members`, and `stats` are never used
   --> src/010_scc_compressor.rs:99:12
    |
 31 | impl SccCompression {
    | ------------------- methods in this implementation
...
 99 |     pub fn get_scc_for_node(&self, node_name: &str) -> Option<usize> {
    |            ^^^^^^^^^^^^^^^^
...
110 |     pub fn get_scc_members(&self, scc_id: usize) -> Vec<String> {
    |            ^^^^^^^^^^^^^^^
...
131 |     pub fn stats(&self) -> SccStats {
    |            ^^^^^

warning: struct `SccStats` is never constructed
   --> src/010_scc_compressor.rs:144:12
    |
144 | pub struct SccStats {
    |            ^^^^^^^^

warning: field `stable_nodes` is never read
  --> src/030_fixpoint_solver.rs:96:9
   |
85 | pub struct FixpointResult {
   |            -------------- field in this struct
...
96 |     pub stable_nodes: Vec<String>,
   |         ^^^^^^^^^^^^
   |
   = note: `FixpointResult` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: field `compressed_nodes` is never read
   --> src/060_path_detector.rs:194:9
    |
192 | pub struct PathStats {
    |            --------- field in this struct
193 |     pub original_nodes: usize,
194 |     pub compressed_nodes: usize,
    |         ^^^^^^^^^^^^^^^^
    |
    = note: `PathStats` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: method `check_violations` is never used
   --> src/070_invariant_integrator.rs:144:12
    |
 21 | impl<'a> InvariantDetector<'a> {
    | ------------------------------ method in this implementation
...
144 |     pub fn check_violations(&self, invariants: &[Invariant]) -> Vec<InvariantViolation> {
    |            ^^^^^^^^^^^^^^^^

warning: fields `protected_functions`, `blocking_invariants`, and `empirical_count` are never read
   --> src/082_conscience_graph.rs:185:9
    |
183 | pub struct ConscienceStats {
    |            --------------- fields in this struct
184 |     pub total_functions: usize,
185 |     pub protected_functions: usize,
    |         ^^^^^^^^^^^^^^^^^^^
186 |     pub total_invariants: usize,
187 |     pub blocking_invariants: usize,
    |         ^^^^^^^^^^^^^^^^^^^
188 |     pub proven_count: usize,
189 |     pub empirical_count: usize,
    |         ^^^^^^^^^^^^^^^

warning: function `matches_function` is never used
  --> src/083_action_validator.rs:80:4
   |
80 | fn matches_function(action_name: &str, constraint: &RefactorConstraint) -> bool {
   |    ^^^^^^^^^^^^^^^^

warning: associated items `load` and `export_json` are never used
   --> src/085_agent_conscience.rs:43:12
    |
 28 | impl AgentConscience {
    | -------------------- associated items in this implementation
...
 43 |     pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    |            ^^^^
...
144 |     pub fn export_json(&self, path: &Path) -> std::io::Result<()> {
    |            ^^^^^^^^^^^

warning: `mmsb-analyzer` (bin "mmsb-analyzer" test) generated 18 warnings (8 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.97s
  Executable unittests src/200_lib.rs (target/debug/deps/mmsb_analyzer-c07b06d80c15a2f1)
  Executable unittests src/190_main.rs (target/debug/deps/mmsb_analyzer-53a5352556fc2668)
```

