# CFG Group: src/630_correction_intelligence_report.rs

## Function: `augment_path_coherence_strategies`

- File: src/630_correction_intelligence_report.rs
- Branches: 0
- Loops: 0
- Nodes: 17
- Edges: 16

```mermaid
flowchart TD
    augment_path_coherence_strategies_0["ENTRY"]
    augment_path_coherence_strategies_1["let RefactorAction :: RenameFile { from , to } = action else { return ; }"]
    augment_path_coherence_strategies_2["let Some (old_mod) = module_name_from_path (from) else { return ; }"]
    augment_path_coherence_strategies_3["let Some (new_mod) = module_name_from_path (to) else { return ; }"]
    augment_path_coherence_strategies_4["let old_file_name = from . file_name () . and_then (| s | s . to_str ()) . unwrap_or ('')"]
    augment_path_coherence_strategies_5["let new_file_name = to . file_name () . and_then (| s | s . to_str ()) . unwrap_or ('')"]
    augment_path_coherence_strategies_6["let replace_mod = old_mod != new_mod"]
    augment_path_coherence_strategies_7["let mod_re = if replace_mod { Regex :: new (& format ! (r'^\s*(pub\s+)?mod\s+{}\s*;' , reg..."]
    augment_path_coherence_strategies_8["let use_re = if replace_mod { Regex :: new (& format ! (r'^\s*use\s+.*\b{}\b' , regex :: e..."]
    augment_path_coherence_strategies_9["let path_re = if ! old_file_name . is_empty () && ! new_file_name . is_empty () { Regex :: ..."]
    augment_path_coherence_strategies_10["let mut updates = Vec :: new ()"]
    augment_path_coherence_strategies_11["let mut seen = HashSet :: new ()"]
    augment_path_coherence_strategies_12["let rust_files = crate :: cluster_010 :: gather_rust_files (root)"]
    augment_path_coherence_strategies_13["for file in rust_files { let Ok (contents) = fs :: read_to_string (& file) el..."]
    augment_path_coherence_strategies_14["updates . sort_by (| a , b | { a . 0 . cmp (& b . 0) . then_with (| | a . 1 ...."]
    augment_path_coherence_strategies_15["for (file , old_ref , new_ref) in updates { plan . strategies . push (Correct..."]
    augment_path_coherence_strategies_16["EXIT"]
    augment_path_coherence_strategies_0 --> augment_path_coherence_strategies_1
    augment_path_coherence_strategies_1 --> augment_path_coherence_strategies_2
    augment_path_coherence_strategies_2 --> augment_path_coherence_strategies_3
    augment_path_coherence_strategies_3 --> augment_path_coherence_strategies_4
    augment_path_coherence_strategies_4 --> augment_path_coherence_strategies_5
    augment_path_coherence_strategies_5 --> augment_path_coherence_strategies_6
    augment_path_coherence_strategies_6 --> augment_path_coherence_strategies_7
    augment_path_coherence_strategies_7 --> augment_path_coherence_strategies_8
    augment_path_coherence_strategies_8 --> augment_path_coherence_strategies_9
    augment_path_coherence_strategies_9 --> augment_path_coherence_strategies_10
    augment_path_coherence_strategies_10 --> augment_path_coherence_strategies_11
    augment_path_coherence_strategies_11 --> augment_path_coherence_strategies_12
    augment_path_coherence_strategies_12 --> augment_path_coherence_strategies_13
    augment_path_coherence_strategies_13 --> augment_path_coherence_strategies_14
    augment_path_coherence_strategies_14 --> augment_path_coherence_strategies_15
    augment_path_coherence_strategies_15 --> augment_path_coherence_strategies_16
```

## Function: `build_state`

- File: src/630_correction_intelligence_report.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    build_state_0["ENTRY"]
    build_state_1["IntelligenceState { root : root . to_path_buf () , invariants : & analysis . ..."]
    build_state_2["EXIT"]
    build_state_0 --> build_state_1
    build_state_1 --> build_state_2
```

## Function: `compute_summary`

- File: src/630_correction_intelligence_report.rs
- Branches: 0
- Loops: 0
- Nodes: 12
- Edges: 11

```mermaid
flowchart TD
    compute_summary_0["ENTRY"]
    compute_summary_1["let mut trivial = 0"]
    compute_summary_2["let mut moderate = 0"]
    compute_summary_3["let mut complex = 0"]
    compute_summary_4["let mut total_violations = 0"]
    compute_summary_5["let mut total_confidence = 0.0"]
    compute_summary_6["let mut total_time = 0"]
    compute_summary_7["for plan in plans { match plan . tier { ErrorTier :: Trivial => trivial += 1 ..."]
    compute_summary_8["let avg_conf = if plans . is_empty () { 0.0 } else { total_confidence / plans . len () as f64 }"]
    compute_summary_9["let _ = deltas"]
    compute_summary_10["CorrectionSummary { trivial_count : trivial , moderate_count : moderate , com..."]
    compute_summary_11["EXIT"]
    compute_summary_0 --> compute_summary_1
    compute_summary_1 --> compute_summary_2
    compute_summary_2 --> compute_summary_3
    compute_summary_3 --> compute_summary_4
    compute_summary_4 --> compute_summary_5
    compute_summary_5 --> compute_summary_6
    compute_summary_6 --> compute_summary_7
    compute_summary_7 --> compute_summary_8
    compute_summary_8 --> compute_summary_9
    compute_summary_9 --> compute_summary_10
    compute_summary_10 --> compute_summary_11
```

## Function: `default_confidence`

- File: src/630_correction_intelligence_report.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    default_confidence_0["ENTRY"]
    default_confidence_1["match violation_type { crate :: correction_plan_types :: ViolationType :: Unr..."]
    default_confidence_2["EXIT"]
    default_confidence_0 --> default_confidence_1
    default_confidence_1 --> default_confidence_2
```

## Function: `fill_prediction_confidence`

- File: src/630_correction_intelligence_report.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    fill_prediction_confidence_0["ENTRY"]
    fill_prediction_confidence_1["for prediction in predictions { if prediction . confidence <= 0.0 { predictio..."]
    fill_prediction_confidence_2["EXIT"]
    fill_prediction_confidence_0 --> fill_prediction_confidence_1
    fill_prediction_confidence_1 --> fill_prediction_confidence_2
```

## Function: `filter_path_coherence_report`

- File: src/630_correction_intelligence_report.rs
- Branches: 0
- Loops: 0
- Nodes: 9
- Edges: 8

```mermaid
flowchart TD
    filter_path_coherence_report_0["ENTRY"]
    filter_path_coherence_report_1["let mut plans = Vec :: new ()"]
    filter_path_coherence_report_2["let mut policies = Vec :: new ()"]
    filter_path_coherence_report_3["let mut criteria = Vec :: new ()"]
    filter_path_coherence_report_4["let mut deltas = Vec :: new ()"]
    filter_path_coherence_report_5["for (idx , plan) in report . correction_plans . iter () . enumerate () { let ..."]
    filter_path_coherence_report_6["let summary = compute_summary (& plans , & deltas)"]
    filter_path_coherence_report_7["CorrectionIntelligenceReport { version : report . version . clone () , timest..."]
    filter_path_coherence_report_8["EXIT"]
    filter_path_coherence_report_0 --> filter_path_coherence_report_1
    filter_path_coherence_report_1 --> filter_path_coherence_report_2
    filter_path_coherence_report_2 --> filter_path_coherence_report_3
    filter_path_coherence_report_3 --> filter_path_coherence_report_4
    filter_path_coherence_report_4 --> filter_path_coherence_report_5
    filter_path_coherence_report_5 --> filter_path_coherence_report_6
    filter_path_coherence_report_6 --> filter_path_coherence_report_7
    filter_path_coherence_report_7 --> filter_path_coherence_report_8
```

## Function: `filter_visibility_report`

- File: src/630_correction_intelligence_report.rs
- Branches: 0
- Loops: 0
- Nodes: 9
- Edges: 8

```mermaid
flowchart TD
    filter_visibility_report_0["ENTRY"]
    filter_visibility_report_1["let mut plans = Vec :: new ()"]
    filter_visibility_report_2["let mut policies = Vec :: new ()"]
    filter_visibility_report_3["let mut criteria = Vec :: new ()"]
    filter_visibility_report_4["let mut deltas = Vec :: new ()"]
    filter_visibility_report_5["for (idx , plan) in report . correction_plans . iter () . enumerate () { let ..."]
    filter_visibility_report_6["let summary = compute_summary (& plans , & deltas)"]
    filter_visibility_report_7["CorrectionIntelligenceReport { version : report . version . clone () , timest..."]
    filter_visibility_report_8["EXIT"]
    filter_visibility_report_0 --> filter_visibility_report_1
    filter_visibility_report_1 --> filter_visibility_report_2
    filter_visibility_report_2 --> filter_visibility_report_3
    filter_visibility_report_3 --> filter_visibility_report_4
    filter_visibility_report_4 --> filter_visibility_report_5
    filter_visibility_report_5 --> filter_visibility_report_6
    filter_visibility_report_6 --> filter_visibility_report_7
    filter_visibility_report_7 --> filter_visibility_report_8
```

## Function: `module_name_from_path`

- File: src/630_correction_intelligence_report.rs
- Branches: 0
- Loops: 0
- Nodes: 5
- Edges: 4

```mermaid
flowchart TD
    module_name_from_path_0["ENTRY"]
    module_name_from_path_1["let stem = path . file_stem () . and_then (| s | s . to_str ()) ?"]
    module_name_from_path_2["let name = if stem == 'mod' { path . parent () . and_then (| p | p . file_name ()) . and..."]
    module_name_from_path_3["Some (crate :: cluster_010 :: normalize_module_name (& name))"]
    module_name_from_path_4["EXIT"]
    module_name_from_path_0 --> module_name_from_path_1
    module_name_from_path_1 --> module_name_from_path_2
    module_name_from_path_2 --> module_name_from_path_3
    module_name_from_path_3 --> module_name_from_path_4
```

## Function: `write_intelligence_outputs`

- File: src/630_correction_intelligence_report.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    write_intelligence_outputs_0["ENTRY"]
    write_intelligence_outputs_1["write_intelligence_outputs_at (report , output_dir , None , None)"]
    write_intelligence_outputs_2["EXIT"]
    write_intelligence_outputs_0 --> write_intelligence_outputs_1
    write_intelligence_outputs_1 --> write_intelligence_outputs_2
```

