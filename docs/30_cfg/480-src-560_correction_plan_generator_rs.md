# CFG Group: src/560_correction_plan_generator.rs

## Function: `action_function`

- File: src/560_correction_plan_generator.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    action_function_0["ENTRY"]
    action_function_1["match action { RefactorAction :: MoveFunction { function , .. } => Some (func..."]
    action_function_2["EXIT"]
    action_function_0 --> action_function_1
    action_function_1 --> action_function_2
```

## Function: `action_module_path`

- File: src/560_correction_plan_generator.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    action_module_path_0["ENTRY"]
    action_module_path_1["match action { RefactorAction :: MoveFunction { to , .. } => to . display () ..."]
    action_module_path_2["EXIT"]
    action_module_path_0 --> action_module_path_1
    action_module_path_1 --> action_module_path_2
```

## Function: `action_refs`

- File: src/560_correction_plan_generator.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    action_refs_0["ENTRY"]
    action_refs_1["match action { RefactorAction :: RenameFunction { old_name , new_name , .. } ..."]
    action_refs_2["EXIT"]
    action_refs_0 --> action_refs_1
    action_refs_1 --> action_refs_2
```

## Function: `action_symbol`

- File: src/560_correction_plan_generator.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    action_symbol_0["ENTRY"]
    action_symbol_1["match action { RefactorAction :: MoveFunction { function , .. } => Some (func..."]
    action_symbol_2["EXIT"]
    action_symbol_0 --> action_symbol_1
    action_symbol_1 --> action_symbol_2
```

## Function: `action_target_layer`

- File: src/560_correction_plan_generator.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    action_target_layer_0["ENTRY"]
    action_target_layer_1["match action { RefactorAction :: MoveFunction { required_layer , .. } => requ..."]
    action_target_layer_2["EXIT"]
    action_target_layer_0 --> action_target_layer_1
    action_target_layer_1 --> action_target_layer_2
```

## Function: `action_visibility`

- File: src/560_correction_plan_generator.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    action_visibility_0["ENTRY"]
    action_visibility_1["match action { RefactorAction :: AdjustVisibility { symbol , file , from , to..."]
    action_visibility_2["EXIT"]
    action_visibility_0 --> action_visibility_1
    action_visibility_1 --> action_visibility_2
```

## Function: `average_confidence`

- File: src/560_correction_plan_generator.rs
- Branches: 1
- Loops: 0
- Nodes: 9
- Edges: 9

```mermaid
flowchart TD
    average_confidence_0["ENTRY"]
    average_confidence_1["if predictions . is_empty ()"]
    average_confidence_2["THEN BB"]
    average_confidence_3["return 1.0"]
    average_confidence_4["EMPTY ELSE"]
    average_confidence_5["IF JOIN"]
    average_confidence_6["let total : f64 = predictions . iter () . map (| p | p . confidence) . sum ()"]
    average_confidence_7["total / predictions . len () as f64"]
    average_confidence_8["EXIT"]
    average_confidence_0 --> average_confidence_1
    average_confidence_1 --> average_confidence_2
    average_confidence_2 --> average_confidence_3
    average_confidence_1 --> average_confidence_4
    average_confidence_3 --> average_confidence_5
    average_confidence_4 --> average_confidence_5
    average_confidence_5 --> average_confidence_6
    average_confidence_6 --> average_confidence_7
    average_confidence_7 --> average_confidence_8
```

## Function: `estimate_fix_time`

- File: src/560_correction_plan_generator.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    estimate_fix_time_0["ENTRY"]
    estimate_fix_time_1["10 + (count as u32 * 5)"]
    estimate_fix_time_2["EXIT"]
    estimate_fix_time_0 --> estimate_fix_time_1
    estimate_fix_time_1 --> estimate_fix_time_2
```

## Function: `generate_correction_plan`

- File: src/560_correction_plan_generator.rs
- Branches: 0
- Loops: 0
- Nodes: 6
- Edges: 5

```mermaid
flowchart TD
    generate_correction_plan_0["ENTRY"]
    generate_correction_plan_1["let mut strategies = Vec :: new ()"]
    generate_correction_plan_2["for prediction in predictions { match prediction . violation_type { Violation..."]
    generate_correction_plan_3["let tier = predictions . iter () . map (classify_tier) . max () . unwrap_or (ErrorTier :..."]
    generate_correction_plan_4["CorrectionPlan { action_id : action . action_id () , tier , predicted_violati..."]
    generate_correction_plan_5["EXIT"]
    generate_correction_plan_0 --> generate_correction_plan_1
    generate_correction_plan_1 --> generate_correction_plan_2
    generate_correction_plan_2 --> generate_correction_plan_3
    generate_correction_plan_3 --> generate_correction_plan_4
    generate_correction_plan_4 --> generate_correction_plan_5
```

