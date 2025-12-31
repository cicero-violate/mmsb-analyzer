# CFG Group: src/580_rollback_criteria_builder.rs

## Function: `build_rollback_criteria`

- File: src/580_rollback_criteria_builder.rs
- Branches: 0
- Loops: 0
- Nodes: 7
- Edges: 6

```mermaid
flowchart TD
    build_rollback_criteria_0["ENTRY"]
    build_rollback_criteria_1["let mut mandatory = vec ! [RollbackCondition :: BuildFailed]"]
    build_rollback_criteria_2["let mut suggested = vec ! [RollbackCondition :: QualityDecreased { threshold : 0.05 }]"]
    build_rollback_criteria_3["match correction_plan . tier { ErrorTier :: Complex => { mandatory . push (Ro..."]
    build_rollback_criteria_4["for prediction in & correction_plan . predicted_violations { if prediction . ..."]
    build_rollback_criteria_5["RollbackCriteria { action_id : correction_plan . action_id . clone () , manda..."]
    build_rollback_criteria_6["EXIT"]
    build_rollback_criteria_0 --> build_rollback_criteria_1
    build_rollback_criteria_1 --> build_rollback_criteria_2
    build_rollback_criteria_2 --> build_rollback_criteria_3
    build_rollback_criteria_3 --> build_rollback_criteria_4
    build_rollback_criteria_4 --> build_rollback_criteria_5
    build_rollback_criteria_5 --> build_rollback_criteria_6
```

## Function: `extract_critical_tests`

- File: src/580_rollback_criteria_builder.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    extract_critical_tests_0["ENTRY"]
    extract_critical_tests_1["Vec :: new ()"]
    extract_critical_tests_2["EXIT"]
    extract_critical_tests_0 --> extract_critical_tests_1
    extract_critical_tests_1 --> extract_critical_tests_2
```

