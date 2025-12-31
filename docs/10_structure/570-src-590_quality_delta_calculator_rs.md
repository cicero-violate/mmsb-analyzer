# Structure Group: src/590_quality_delta_calculator.rs

## File: src/590_quality_delta_calculator.rs

- Layer(s): 590_quality_delta_calculator.rs
- Language coverage: Rust (3)
- Element types: Function (2), Struct (1)
- Total elements: 3

### Elements

- [Rust | Struct] `Metrics` (line 0, pub)
  - Signature: `# [derive (Clone , Debug , Default)] pub struct Metrics { pub cohesion : f64 , pub violations : usize , pub complexit...`
- [Rust | Function] `calculate_quality_delta` (line 0, pub)
  - Signature: `pub fn calculate_quality_delta (action : & RefactorAction , current : & Metrics , simulated : & Metrics ,) -> Quality...`
  - Calls: to_string, to_string, to_string, action_id
- [Rust | Function] `estimate_impact` (line 0, pub)
  - Signature: `pub fn estimate_impact (action : & RefactorAction , current_state : & AnalysisState) -> QualityDelta { let simulated ...`
  - Calls: simulate_action, calculate_quality_delta

