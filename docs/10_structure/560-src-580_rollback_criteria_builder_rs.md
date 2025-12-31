# Structure Group: src/580_rollback_criteria_builder.rs

## File: src/580_rollback_criteria_builder.rs

- Layer(s): 580_rollback_criteria_builder.rs
- Language coverage: Rust (2)
- Element types: Function (2)
- Total elements: 2

### Elements

- [Rust | Function] `build_rollback_criteria` (line 0, pub)
  - Signature: `pub fn build_rollback_criteria (action : & RefactorAction , correction_plan : & CorrectionPlan ,) -> RollbackCriteria...`
  - Calls: push, push, push, extract_critical_tests, push, clone
- [Rust | Function] `extract_critical_tests` (line 0, priv)
  - Signature: `fn extract_critical_tests (_action : & RefactorAction) -> Vec < String > { Vec :: new () } . sig`
  - Calls: Vec::new

