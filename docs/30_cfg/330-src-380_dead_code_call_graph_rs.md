# CFG Group: src/380_dead_code_call_graph.rs

## Function: `build_call_graph`

- File: src/380_dead_code_call_graph.rs
- Branches: 0
- Loops: 0
- Nodes: 5
- Edges: 4

```mermaid
flowchart TD
    build_call_graph_0["ENTRY"]
    build_call_graph_1["let mut graph : CallGraph = HashMap :: new ()"]
    build_call_graph_2["for element in elements { if element . element_type != ElementType :: Functio..."]
    build_call_graph_3["graph"]
    build_call_graph_4["EXIT"]
    build_call_graph_0 --> build_call_graph_1
    build_call_graph_1 --> build_call_graph_2
    build_call_graph_2 --> build_call_graph_3
    build_call_graph_3 --> build_call_graph_4
```

## Function: `build_reverse_call_graph`

- File: src/380_dead_code_call_graph.rs
- Branches: 0
- Loops: 0
- Nodes: 5
- Edges: 4

```mermaid
flowchart TD
    build_reverse_call_graph_0["ENTRY"]
    build_reverse_call_graph_1["let mut reverse : CallGraph = HashMap :: new ()"]
    build_reverse_call_graph_2["for (caller , callees) in graph { for callee in callees { reverse . entry (ca..."]
    build_reverse_call_graph_3["reverse"]
    build_reverse_call_graph_4["EXIT"]
    build_reverse_call_graph_0 --> build_reverse_call_graph_1
    build_reverse_call_graph_1 --> build_reverse_call_graph_2
    build_reverse_call_graph_2 --> build_reverse_call_graph_3
    build_reverse_call_graph_3 --> build_reverse_call_graph_4
```

## Function: `classify_symbol`

- File: src/380_dead_code_call_graph.rs
- Branches: 3
- Loops: 0
- Nodes: 18
- Edges: 20

```mermaid
flowchart TD
    classify_symbol_0["ENTRY"]
    classify_symbol_1["if intent_map . contains_key (symbol)"]
    classify_symbol_2["THEN BB"]
    classify_symbol_3["return DeadCodeCategory :: LatentPlanned"]
    classify_symbol_4["EMPTY ELSE"]
    classify_symbol_5["IF JOIN"]
    classify_symbol_6["if is_test_only (symbol , call_graph , test_boundaries)"]
    classify_symbol_7["THEN BB"]
    classify_symbol_8["return DeadCodeCategory :: TestOnly"]
    classify_symbol_9["EMPTY ELSE"]
    classify_symbol_10["IF JOIN"]
    classify_symbol_11["if ! is_reachable (symbol , call_graph , entrypoints)"]
    classify_symbol_12["THEN BB"]
    classify_symbol_13["return DeadCodeCategory :: Unreachable"]
    classify_symbol_14["EMPTY ELSE"]
    classify_symbol_15["IF JOIN"]
    classify_symbol_16["DeadCodeCategory :: ReachableUnused"]
    classify_symbol_17["EXIT"]
    classify_symbol_0 --> classify_symbol_1
    classify_symbol_1 --> classify_symbol_2
    classify_symbol_2 --> classify_symbol_3
    classify_symbol_1 --> classify_symbol_4
    classify_symbol_3 --> classify_symbol_5
    classify_symbol_4 --> classify_symbol_5
    classify_symbol_5 --> classify_symbol_6
    classify_symbol_6 --> classify_symbol_7
    classify_symbol_7 --> classify_symbol_8
    classify_symbol_6 --> classify_symbol_9
    classify_symbol_8 --> classify_symbol_10
    classify_symbol_9 --> classify_symbol_10
    classify_symbol_10 --> classify_symbol_11
    classify_symbol_11 --> classify_symbol_12
    classify_symbol_12 --> classify_symbol_13
    classify_symbol_11 --> classify_symbol_14
    classify_symbol_13 --> classify_symbol_15
    classify_symbol_14 --> classify_symbol_15
    classify_symbol_15 --> classify_symbol_16
    classify_symbol_16 --> classify_symbol_17
```

## Function: `compute_reachability`

- File: src/380_dead_code_call_graph.rs
- Branches: 0
- Loops: 0
- Nodes: 6
- Edges: 5

```mermaid
flowchart TD
    compute_reachability_0["ENTRY"]
    compute_reachability_1["let mut reachable = HashSet :: new ()"]
    compute_reachability_2["let mut queue : VecDeque < String > = entrypoints . iter () . cloned () . collect ()"]
    compute_reachability_3["while let Some (node) = queue . pop_front () { if ! reachable . insert (node ..."]
    compute_reachability_4["reachable"]
    compute_reachability_5["EXIT"]
    compute_reachability_0 --> compute_reachability_1
    compute_reachability_1 --> compute_reachability_2
    compute_reachability_2 --> compute_reachability_3
    compute_reachability_3 --> compute_reachability_4
    compute_reachability_4 --> compute_reachability_5
```

## Function: `is_reachable`

- File: src/380_dead_code_call_graph.rs
- Branches: 1
- Loops: 0
- Nodes: 8
- Edges: 8

```mermaid
flowchart TD
    is_reachable_0["ENTRY"]
    is_reachable_1["if entrypoints . is_empty ()"]
    is_reachable_2["THEN BB"]
    is_reachable_3["return false"]
    is_reachable_4["EMPTY ELSE"]
    is_reachable_5["IF JOIN"]
    is_reachable_6["compute_reachability (graph , entrypoints) . contains (symbol)"]
    is_reachable_7["EXIT"]
    is_reachable_0 --> is_reachable_1
    is_reachable_1 --> is_reachable_2
    is_reachable_2 --> is_reachable_3
    is_reachable_1 --> is_reachable_4
    is_reachable_3 --> is_reachable_5
    is_reachable_4 --> is_reachable_5
    is_reachable_5 --> is_reachable_6
    is_reachable_6 --> is_reachable_7
```

## Function: `is_test_only`

- File: src/380_dead_code_call_graph.rs
- Branches: 2
- Loops: 0
- Nodes: 16
- Edges: 17

```mermaid
flowchart TD
    is_test_only_0["ENTRY"]
    is_test_only_1["if test_boundaries . test_symbols . contains (symbol)"]
    is_test_only_2["THEN BB"]
    is_test_only_3["return true"]
    is_test_only_4["EMPTY ELSE"]
    is_test_only_5["IF JOIN"]
    is_test_only_6["let reverse = build_reverse_call_graph (call_graph)"]
    is_test_only_7["let callers = reverse . get (symbol)"]
    is_test_only_8["let Some (callers) = callers else { return false ; }"]
    is_test_only_9["if callers . is_empty ()"]
    is_test_only_10["THEN BB"]
    is_test_only_11["return false"]
    is_test_only_12["EMPTY ELSE"]
    is_test_only_13["IF JOIN"]
    is_test_only_14["callers . iter () . all (| caller | test_boundaries . test_symbols . contains..."]
    is_test_only_15["EXIT"]
    is_test_only_0 --> is_test_only_1
    is_test_only_1 --> is_test_only_2
    is_test_only_2 --> is_test_only_3
    is_test_only_1 --> is_test_only_4
    is_test_only_3 --> is_test_only_5
    is_test_only_4 --> is_test_only_5
    is_test_only_5 --> is_test_only_6
    is_test_only_6 --> is_test_only_7
    is_test_only_7 --> is_test_only_8
    is_test_only_8 --> is_test_only_9
    is_test_only_9 --> is_test_only_10
    is_test_only_10 --> is_test_only_11
    is_test_only_9 --> is_test_only_12
    is_test_only_11 --> is_test_only_13
    is_test_only_12 --> is_test_only_13
    is_test_only_13 --> is_test_only_14
    is_test_only_14 --> is_test_only_15
```

