# File Ordering Report

Generated: 2025-12-30 22:29:05

## Rust File Ordering

### Metrics

- Total files: 35
- Rename suggestions: 34
- Ordering violations: 0
- Layer violations: 26
- Directories: 1

### Cycles Detected
- MMSB/tools/mmsb-analyzer/src/130_control_flow.rs, MMSB/tools/mmsb-analyzer/src/090_utilities.rs, MMSB/tools/mmsb-analyzer/src/180_report.rs, MMSB/tools/mmsb-analyzer/src/140_file_ordering.rs, MMSB/tools/mmsb-analyzer/src/120_directory_analyzer.rs, MMSB/tools/mmsb-analyzer/src/110_cohesion_analyzer.rs, MMSB/tools/mmsb-analyzer/src/060_layer_core.rs, MMSB/tools/mmsb-analyzer/src/050_cluster_006.rs, MMSB/tools/mmsb-analyzer/src/010_cluster_008.rs, MMSB/tools/mmsb-analyzer/src/040_dependency.rs, MMSB/tools/mmsb-analyzer/src/020_cluster_010.rs, MMSB/tools/mmsb-analyzer/src/170_dot_exporter.rs, MMSB/tools/mmsb-analyzer/src/000_cluster_001.rs, MMSB/tools/mmsb-analyzer/src/070_layer_utilities.rs

### Canonical Order

| Order | Current | Suggested | Rename |
| --- | --- | --- | --- |
| 0 | `src/000_cluster_001.rs` | `000_cluster_001.rs` | no |
| 10 | `src/000_invariant_types.rs` | `010_invariant_types.rs` | yes |
| 20 | `src/005_refactor_constraints.rs` | `020_refactor_constraints.rs` | yes |
| 30 | `src/010_cluster_008.rs` | `030_cluster_008.rs` | yes |
| 40 | `src/010_scc_compressor.rs` | `040_scc_compressor.rs` | yes |
| 50 | `src/020_cluster_010.rs` | `050_cluster_010.rs` | yes |
| 60 | `src/020_layer_inference.rs` | `060_layer_inference.rs` | yes |
| 70 | `src/030_cluster_011.rs` | `070_cluster_011.rs` | yes |
| 80 | `src/030_fixpoint_solver.rs` | `080_fixpoint_solver.rs` | yes |
| 90 | `src/040_dependency.rs` | `090_dependency.rs` | yes |
| 100 | `src/040_structural_detector.rs` | `100_structural_detector.rs` | yes |
| 110 | `src/050_cluster_006.rs` | `110_cluster_006.rs` | yes |
| 120 | `src/050_semantic_detector.rs` | `120_semantic_detector.rs` | yes |
| 130 | `src/060_layer_core.rs` | `130_layer_core.rs` | yes |
| 140 | `src/060_path_detector.rs` | `140_path_detector.rs` | yes |
| 150 | `src/070_invariant_integrator.rs` | `150_invariant_integrator.rs` | yes |
| 160 | `src/070_layer_utilities.rs` | `160_layer_utilities.rs` | yes |
| 170 | `src/080_file_gathering.rs` | `170_file_gathering.rs` | yes |
| 180 | `src/080_invariant_reporter.rs` | `180_invariant_reporter.rs` | yes |
| 190 | `src/082_conscience_graph.rs` | `190_conscience_graph.rs` | yes |
| 200 | `src/083_action_validator.rs` | `200_action_validator.rs` | yes |
| 210 | `src/085_agent_conscience.rs` | `210_agent_conscience.rs` | yes |
| 220 | `src/090_utilities.rs` | `220_utilities.rs` | yes |
| 230 | `src/100_types.rs` | `230_types.rs` | yes |
| 240 | `src/110_cohesion_analyzer.rs` | `240_cohesion_analyzer.rs` | yes |
| 250 | `src/120_directory_analyzer.rs` | `250_directory_analyzer.rs` | yes |
| 260 | `src/130_control_flow.rs` | `260_control_flow.rs` | yes |
| 270 | `src/140_file_ordering.rs` | `270_file_ordering.rs` | yes |
| 280 | `src/150_julia_parser.rs` | `280_julia_parser.rs` | yes |
| 290 | `src/160_rust_parser.rs` | `290_rust_parser.rs` | yes |
| 300 | `src/170_dot_exporter.rs` | `300_dot_exporter.rs` | yes |
| 310 | `src/180_report.rs` | `310_report.rs` | yes |
| 320 | `src/190_main.rs` | `320_main.rs` | yes |
| 330 | `src/191_agent_cli.rs` | `330_agent_cli.rs` | yes |
| 340 | `src/200_lib.rs` | `340_lib.rs` | yes |

### Ordering Violations
- None detected.

### Layer Violations
- `src/030_cluster_011.rs` (030_cluster_011.rs) depends on `src/100_types.rs` (100_types.rs)
- `src/070_layer_utilities.rs` (070_layer_utilities.rs) depends on `src/110_cohesion_analyzer.rs` (110_cohesion_analyzer.rs)
- `src/070_layer_utilities.rs` (070_layer_utilities.rs) depends on `src/130_control_flow.rs` (130_control_flow.rs)
- `src/070_layer_utilities.rs` (070_layer_utilities.rs) depends on `src/120_directory_analyzer.rs` (120_directory_analyzer.rs)
- `src/070_layer_utilities.rs` (070_layer_utilities.rs) depends on `src/170_dot_exporter.rs` (170_dot_exporter.rs)
- `src/070_layer_utilities.rs` (070_layer_utilities.rs) depends on `src/080_invariant_reporter.rs` (080_invariant_reporter.rs)
- `src/070_layer_utilities.rs` (070_layer_utilities.rs) depends on `src/150_julia_parser.rs` (150_julia_parser.rs)
- `src/070_layer_utilities.rs` (070_layer_utilities.rs) depends on `src/180_report.rs` (180_report.rs)
- `src/070_layer_utilities.rs` (070_layer_utilities.rs) depends on `src/160_rust_parser.rs` (160_rust_parser.rs)
- `src/070_layer_utilities.rs` (070_layer_utilities.rs) depends on `src/100_types.rs` (100_types.rs)
- `src/050_semantic_detector.rs` (050_semantic_detector.rs) depends on `src/100_types.rs` (100_types.rs)
- `src/010_cluster_008.rs` (010_cluster_008.rs) depends on `src/040_dependency.rs` (040_dependency.rs)
- `src/010_cluster_008.rs` (010_cluster_008.rs) depends on `src/100_types.rs` (100_types.rs)
- `src/040_structural_detector.rs` (040_structural_detector.rs) depends on `src/100_types.rs` (100_types.rs)
- `src/090_utilities.rs` (090_utilities.rs) depends on `src/180_report.rs` (180_report.rs)
- `src/090_utilities.rs` (090_utilities.rs) depends on `src/100_types.rs` (100_types.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/050_cluster_006.rs` (050_cluster_006.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/040_dependency.rs` (040_dependency.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/140_file_ordering.rs` (140_file_ordering.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/060_layer_core.rs` (060_layer_core.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/070_layer_utilities.rs` (070_layer_utilities.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/100_types.rs` (100_types.rs)
- `src/000_cluster_001.rs` (000_cluster_001.rs) depends on `src/090_utilities.rs` (090_utilities.rs)
- `src/070_invariant_integrator.rs` (070_invariant_integrator.rs) depends on `src/100_types.rs` (100_types.rs)
- `src/190_main.rs` (190_main.rs) depends on `src/191_agent_cli.rs` (191_agent_cli.rs)
- `src/020_cluster_010.rs` (020_cluster_010.rs) depends on `src/040_dependency.rs` (040_dependency.rs)

### Directory Order
- `src`

## Julia File Ordering

No files analyzed.

