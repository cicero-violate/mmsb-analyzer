# CFG Group: src/590_quality_delta_calculator.rs

## Function: `calculate_quality_delta`

- File: src/590_quality_delta_calculator.rs
- Branches: 0
- Loops: 0
- Nodes: 9
- Edges: 8

```mermaid
flowchart TD
    calculate_quality_delta_0["ENTRY"]
    calculate_quality_delta_1["let cohesion_delta = simulated . cohesion - current . cohesion"]
    calculate_quality_delta_2["let violation_delta = simulated . violations as i32 - current . violations as i32"]
    calculate_quality_delta_3["let complexity_delta = simulated . complexity - current . complexity"]
    calculate_quality_delta_4["let overall = 0.5 * cohesion_delta - 0.3 * violation_delta as f64 - 0.2 * complexity_delta"]
    calculate_quality_delta_5["let acceptable = overall > - 0.05 && violation_delta <= 0"]
    calculate_quality_delta_6["let reason = if acceptable { 'Quality improved or maintained' . to_string () } else if ove..."]
    calculate_quality_delta_7["QualityDelta { action_id : action . action_id () , cohesion_delta , violation..."]
    calculate_quality_delta_8["EXIT"]
    calculate_quality_delta_0 --> calculate_quality_delta_1
    calculate_quality_delta_1 --> calculate_quality_delta_2
    calculate_quality_delta_2 --> calculate_quality_delta_3
    calculate_quality_delta_3 --> calculate_quality_delta_4
    calculate_quality_delta_4 --> calculate_quality_delta_5
    calculate_quality_delta_5 --> calculate_quality_delta_6
    calculate_quality_delta_6 --> calculate_quality_delta_7
    calculate_quality_delta_7 --> calculate_quality_delta_8
```

## Function: `estimate_impact`

- File: src/590_quality_delta_calculator.rs
- Branches: 0
- Loops: 0
- Nodes: 4
- Edges: 3

```mermaid
flowchart TD
    estimate_impact_0["ENTRY"]
    estimate_impact_1["let simulated = simulate_action (action , current_state)"]
    estimate_impact_2["calculate_quality_delta (action , & current_state . metrics , & simulated . m..."]
    estimate_impact_3["EXIT"]
    estimate_impact_0 --> estimate_impact_1
    estimate_impact_1 --> estimate_impact_2
    estimate_impact_2 --> estimate_impact_3
```

