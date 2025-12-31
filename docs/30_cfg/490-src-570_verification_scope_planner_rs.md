# CFG Group: src/570_verification_scope_planner.rs

## Function: `action_module`

- File: src/570_verification_scope_planner.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    action_module_0["ENTRY"]
    action_module_1["match action { RefactorAction :: MoveFunction { to , .. } => to . display () ..."]
    action_module_2["EXIT"]
    action_module_0 --> action_module_1
    action_module_1 --> action_module_2
```

## Function: `affected_files`

- File: src/570_verification_scope_planner.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    affected_files_0["ENTRY"]
    affected_files_1["match action { RefactorAction :: MoveFunction { from , to , .. } => vec ! [fr..."]
    affected_files_2["EXIT"]
    affected_files_0 --> affected_files_1
    affected_files_1 --> affected_files_2
```

## Function: `estimate_verification_time`

- File: src/570_verification_scope_planner.rs
- Branches: 0
- Loops: 0
- Nodes: 3
- Edges: 2

```mermaid
flowchart TD
    estimate_verification_time_0["ENTRY"]
    estimate_verification_time_1["match tier { ErrorTier :: Trivial => 10 , ErrorTier :: Moderate => 60 , Error..."]
    estimate_verification_time_2["EXIT"]
    estimate_verification_time_0 --> estimate_verification_time_1
    estimate_verification_time_1 --> estimate_verification_time_2
```

## Function: `plan_verification_scope`

- File: src/570_verification_scope_planner.rs
- Branches: 1
- Loops: 0
- Nodes: 10
- Edges: 10

```mermaid
flowchart TD
    plan_verification_scope_0["ENTRY"]
    plan_verification_scope_1["let scope = match correction_plan . tier { ErrorTier :: Trivial if correction_plan . pred..."]
    plan_verification_scope_2["let mut required_checks = vec ! [VerificationCheck :: CargoCheck]"]
    plan_verification_scope_3["if matches ! (correction_plan . tier , ErrorTier :: Moderate | ErrorTier :: C..."]
    plan_verification_scope_4["THEN BB"]
    plan_verification_scope_5["required_checks . push (VerificationCheck :: CargoTest { filter : None })"]
    plan_verification_scope_6["EMPTY ELSE"]
    plan_verification_scope_7["IF JOIN"]
    plan_verification_scope_8["VerificationPolicy { action_id : correction_plan . action_id . clone () , sco..."]
    plan_verification_scope_9["EXIT"]
    plan_verification_scope_0 --> plan_verification_scope_1
    plan_verification_scope_1 --> plan_verification_scope_2
    plan_verification_scope_2 --> plan_verification_scope_3
    plan_verification_scope_3 --> plan_verification_scope_4
    plan_verification_scope_4 --> plan_verification_scope_5
    plan_verification_scope_3 --> plan_verification_scope_6
    plan_verification_scope_5 --> plan_verification_scope_7
    plan_verification_scope_6 --> plan_verification_scope_7
    plan_verification_scope_7 --> plan_verification_scope_8
    plan_verification_scope_8 --> plan_verification_scope_9
```

