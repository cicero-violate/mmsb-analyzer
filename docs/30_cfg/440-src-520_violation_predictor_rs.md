# CFG Group: src/520_violation_predictor.rs

## Function: `find_callers`

- File: src/520_violation_predictor.rs
- Branches: 1
- Loops: 0
- Nodes: 9
- Edges: 9

```mermaid
flowchart TD
    find_callers_0["ENTRY"]
    find_callers_1["let mut files = HashSet :: new ()"]
    find_callers_2["if let Some (node) = call_graph . get (function)"]
    find_callers_3["THEN BB"]
    find_callers_4["for caller in & node . called_by { if let Some (file) = find_element_file (ca..."]
    find_callers_5["EMPTY ELSE"]
    find_callers_6["IF JOIN"]
    find_callers_7["files . into_iter () . collect ()"]
    find_callers_8["EXIT"]
    find_callers_0 --> find_callers_1
    find_callers_1 --> find_callers_2
    find_callers_2 --> find_callers_3
    find_callers_3 --> find_callers_4
    find_callers_2 --> find_callers_5
    find_callers_4 --> find_callers_6
    find_callers_5 --> find_callers_6
    find_callers_6 --> find_callers_7
    find_callers_7 --> find_callers_8
```

## Function: `find_element_file`

- File: src/520_violation_predictor.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    find_element_file_0["ENTRY"]
    find_element_file_1["elements . iter () . find (| el | el . name == function) . map (| el | PathBu..."]
    find_element_file_2["EXIT"]
    find_element_file_0 --> find_element_file_1
    find_element_file_1 --> find_element_file_2
```

## Function: `find_reference_files`

- File: src/520_violation_predictor.rs
- Branches: 0
- Loops: 0
- Nodes: 5
- Edges: 4

```mermaid
flowchart TD
    find_reference_files_0["ENTRY"]
    find_reference_files_1["let mut files = HashSet :: new ()"]
    find_reference_files_2["for (caller , node) in call_graph { if node . calls . iter () . any (| c | c ..."]
    find_reference_files_3["files . into_iter () . collect ()"]
    find_reference_files_4["EXIT"]
    find_reference_files_0 --> find_reference_files_1
    find_reference_files_1 --> find_reference_files_2
    find_reference_files_2 --> find_reference_files_3
    find_reference_files_3 --> find_reference_files_4
```

## Function: `generate_intelligence_report`

- File: src/520_violation_predictor.rs
- Branches: 0
- Loops: 0
- Nodes: 9
- Edges: 8

```mermaid
flowchart TD
    generate_intelligence_report_0["ENTRY"]
    generate_intelligence_report_1["let mut plans = Vec :: new ()"]
    generate_intelligence_report_2["let mut policies = Vec :: new ()"]
    generate_intelligence_report_3["let mut criteria = Vec :: new ()"]
    generate_intelligence_report_4["let mut deltas = Vec :: new ()"]
    generate_intelligence_report_5["for action in actions { let mut predictions = predict_violations (action , st..."]
    generate_intelligence_report_6["let summary = compute_summary (& plans , & deltas)"]
    generate_intelligence_report_7["CorrectionIntelligenceReport { version : '1.0' . to_string () , timestamp : c..."]
    generate_intelligence_report_8["EXIT"]
    generate_intelligence_report_0 --> generate_intelligence_report_1
    generate_intelligence_report_1 --> generate_intelligence_report_2
    generate_intelligence_report_2 --> generate_intelligence_report_3
    generate_intelligence_report_3 --> generate_intelligence_report_4
    generate_intelligence_report_4 --> generate_intelligence_report_5
    generate_intelligence_report_5 --> generate_intelligence_report_6
    generate_intelligence_report_6 --> generate_intelligence_report_7
    generate_intelligence_report_7 --> generate_intelligence_report_8
```

## Function: `move_violates_invariant`

- File: src/520_violation_predictor.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    move_violates_invariant_0["ENTRY"]
    move_violates_invariant_1["false"]
    move_violates_invariant_2["EXIT"]
    move_violates_invariant_0 --> move_violates_invariant_1
    move_violates_invariant_1 --> move_violates_invariant_2
```

## Function: `predict_violations`

- File: src/520_violation_predictor.rs
- Branches: 0
- Loops: 0
- Nodes: 5
- Edges: 4

```mermaid
flowchart TD
    predict_violations_0["ENTRY"]
    predict_violations_1["let mut predictions = Vec :: new ()"]
    predict_violations_2["match action { RefactorAction :: MoveFunction { function , from , to , requir..."]
    predict_violations_3["predictions"]
    predict_violations_4["EXIT"]
    predict_violations_0 --> predict_violations_1
    predict_violations_1 --> predict_violations_2
    predict_violations_2 --> predict_violations_3
    predict_violations_3 --> predict_violations_4
```

## Function: `symbol_exists`

- File: src/520_violation_predictor.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    symbol_exists_0["ENTRY"]
    symbol_exists_1["elements . iter () . any (| el | el . name == symbol)"]
    symbol_exists_2["EXIT"]
    symbol_exists_0 --> symbol_exists_1
    symbol_exists_1 --> symbol_exists_2
```

