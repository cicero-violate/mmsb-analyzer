# CFG Group: src/090_utilities.rs

## Function: `collect_directory_files`

- File: src/090_utilities.rs
- Branches: 0
- Loops: 0
- Nodes: 4
- Edges: 3

```mermaid
flowchart TD
    collect_directory_files_0["ENTRY"]
    collect_directory_files_1["out . extend (directory . files . iter () . cloned ())"]
    collect_directory_files_2["for sub in & directory . subdirectories { collect_directory_files (sub , out)..."]
    collect_directory_files_3["EXIT"]
    collect_directory_files_0 --> collect_directory_files_1
    collect_directory_files_1 --> collect_directory_files_2
    collect_directory_files_2 --> collect_directory_files_3
```

## Function: `collect_move_items`

- File: src/090_utilities.rs
- Branches: 0
- Loops: 0
- Nodes: 5
- Edges: 4

```mermaid
flowchart TD
    collect_move_items_0["ENTRY"]
    collect_move_items_1["let mut items = Vec :: new ()"]
    collect_move_items_2["for placement in placements { match & placement . placement_status { Placemen..."]
    collect_move_items_3["items"]
    collect_move_items_4["EXIT"]
    collect_move_items_0 --> collect_move_items_1
    collect_move_items_1 --> collect_move_items_2
    collect_move_items_2 --> collect_move_items_3
    collect_move_items_3 --> collect_move_items_4
```

## Function: `compress_path`

- File: src/090_utilities.rs
- Branches: 3
- Loops: 0
- Nodes: 18
- Edges: 20

```mermaid
flowchart TD
    compress_path_0["ENTRY"]
    compress_path_1["if let Some (idx) = path . find ('/MMSB/')"]
    compress_path_2["THEN BB"]
    compress_path_3["return format ! ('MMSB{}' , & path [idx + 5 ..])"]
    compress_path_4["EMPTY ELSE"]
    compress_path_5["IF JOIN"]
    compress_path_6["if path . starts_with ('MMSB/')"]
    compress_path_7["THEN BB"]
    compress_path_8["return path . to_string ()"]
    compress_path_9["EMPTY ELSE"]
    compress_path_10["IF JOIN"]
    compress_path_11["if let Some (idx) = path . rfind ('/src/')"]
    compress_path_12["THEN BB"]
    compress_path_13["return format ! ('MMSB/src{}' , & path [idx + 4 ..])"]
    compress_path_14["EMPTY ELSE"]
    compress_path_15["IF JOIN"]
    compress_path_16["path . to_string ()"]
    compress_path_17["EXIT"]
    compress_path_0 --> compress_path_1
    compress_path_1 --> compress_path_2
    compress_path_2 --> compress_path_3
    compress_path_1 --> compress_path_4
    compress_path_3 --> compress_path_5
    compress_path_4 --> compress_path_5
    compress_path_5 --> compress_path_6
    compress_path_6 --> compress_path_7
    compress_path_7 --> compress_path_8
    compress_path_6 --> compress_path_9
    compress_path_8 --> compress_path_10
    compress_path_9 --> compress_path_10
    compress_path_10 --> compress_path_11
    compress_path_11 --> compress_path_12
    compress_path_12 --> compress_path_13
    compress_path_11 --> compress_path_14
    compress_path_13 --> compress_path_15
    compress_path_14 --> compress_path_15
    compress_path_15 --> compress_path_16
    compress_path_16 --> compress_path_17
```

## Function: `compute_move_metrics`

- File: src/090_utilities.rs
- Branches: 0
- Loops: 0
- Nodes: 13
- Edges: 12

```mermaid
flowchart TD
    compute_move_metrics_0["ENTRY"]
    compute_move_metrics_1["let incoming_calls = placement . call_analysis . calls_from_other_files . iter () . map (| (_ , co..."]
    compute_move_metrics_2["let callers = placement . call_analysis . calls_from_other_files . len ()"]
    compute_move_metrics_3["let mut touched = BTreeSet :: new ()"]
    compute_move_metrics_4["touched . insert (placement . current_file . clone ())"]
    compute_move_metrics_5["let mut outgoing_files = Vec :: new ()"]
    compute_move_metrics_6["for (path , _) in & placement . call_analysis . inter_file_calls { touched . ..."]
    compute_move_metrics_7["let mut caller_files = Vec :: new ()"]
    compute_move_metrics_8["for (path , _) in & placement . call_analysis . calls_from_other_files { touc..."]
    compute_move_metrics_9["let cost = touched . len () . max (1)"]
    compute_move_metrics_10["let benefit = 1 + callers"]
    compute_move_metrics_11["(incoming_calls , benefit , cost , callers , caller_files , outgoing_files)"]
    compute_move_metrics_12["EXIT"]
    compute_move_metrics_0 --> compute_move_metrics_1
    compute_move_metrics_1 --> compute_move_metrics_2
    compute_move_metrics_2 --> compute_move_metrics_3
    compute_move_metrics_3 --> compute_move_metrics_4
    compute_move_metrics_4 --> compute_move_metrics_5
    compute_move_metrics_5 --> compute_move_metrics_6
    compute_move_metrics_6 --> compute_move_metrics_7
    compute_move_metrics_7 --> compute_move_metrics_8
    compute_move_metrics_8 --> compute_move_metrics_9
    compute_move_metrics_9 --> compute_move_metrics_10
    compute_move_metrics_10 --> compute_move_metrics_11
    compute_move_metrics_11 --> compute_move_metrics_12
```

## Function: `path_common_prefix_len`

- File: src/090_utilities.rs
- Branches: 0
- Loops: 0
- Nodes: 5
- Edges: 4

```mermaid
flowchart TD
    path_common_prefix_len_0["ENTRY"]
    path_common_prefix_len_1["let mut count = 0isize"]
    path_common_prefix_len_2["for (a_comp , b_comp) in a . components () . zip (b . components ()) { if a_c..."]
    path_common_prefix_len_3["count"]
    path_common_prefix_len_4["EXIT"]
    path_common_prefix_len_0 --> path_common_prefix_len_1
    path_common_prefix_len_1 --> path_common_prefix_len_2
    path_common_prefix_len_2 --> path_common_prefix_len_3
    path_common_prefix_len_3 --> path_common_prefix_len_4
```

## Function: `resolve_required_layer_path`

- File: src/090_utilities.rs
- Branches: 1
- Loops: 0
- Nodes: 15
- Edges: 15

```mermaid
flowchart TD
    resolve_required_layer_path_0["ENTRY"]
    resolve_required_layer_path_1["let mut files = Vec :: new ()"]
    resolve_required_layer_path_2["collect_directory_files (directory , & mut files)"]
    resolve_required_layer_path_3["let candidates = files . into_iter () . filter (| path | { path . file_name () . and_then (| n..."]
    resolve_required_layer_path_4["if candidates . is_empty ()"]
    resolve_required_layer_path_5["THEN BB"]
    resolve_required_layer_path_6["return current_file . parent () . unwrap_or (root_path) . join (required_layer)"]
    resolve_required_layer_path_7["EMPTY ELSE"]
    resolve_required_layer_path_8["IF JOIN"]
    resolve_required_layer_path_9["let current_dir = current_file . parent () . unwrap_or (root_path)"]
    resolve_required_layer_path_10["let mut best = None"]
    resolve_required_layer_path_11["let mut best_score = - 1isize"]
    resolve_required_layer_path_12["for candidate in candidates { let candidate_dir = candidate . parent () . unw..."]
    resolve_required_layer_path_13["best . unwrap_or_else (| | { current_file . parent () . unwrap_or (root_path)..."]
    resolve_required_layer_path_14["EXIT"]
    resolve_required_layer_path_0 --> resolve_required_layer_path_1
    resolve_required_layer_path_1 --> resolve_required_layer_path_2
    resolve_required_layer_path_2 --> resolve_required_layer_path_3
    resolve_required_layer_path_3 --> resolve_required_layer_path_4
    resolve_required_layer_path_4 --> resolve_required_layer_path_5
    resolve_required_layer_path_5 --> resolve_required_layer_path_6
    resolve_required_layer_path_4 --> resolve_required_layer_path_7
    resolve_required_layer_path_6 --> resolve_required_layer_path_8
    resolve_required_layer_path_7 --> resolve_required_layer_path_8
    resolve_required_layer_path_8 --> resolve_required_layer_path_9
    resolve_required_layer_path_9 --> resolve_required_layer_path_10
    resolve_required_layer_path_10 --> resolve_required_layer_path_11
    resolve_required_layer_path_11 --> resolve_required_layer_path_12
    resolve_required_layer_path_12 --> resolve_required_layer_path_13
    resolve_required_layer_path_13 --> resolve_required_layer_path_14
```

## Function: `write_cluster_batches`

- File: src/090_utilities.rs
- Branches: 1
- Loops: 0
- Nodes: 11
- Edges: 11

```mermaid
flowchart TD
    write_cluster_batches_0["ENTRY"]
    write_cluster_batches_1["if plans . is_empty ()"]
    write_cluster_batches_2["THEN BB"]
    write_cluster_batches_3["return"]
    write_cluster_batches_4["EMPTY ELSE"]
    write_cluster_batches_5["IF JOIN"]
    write_cluster_batches_6["content . push_str ('### Phase 2 Batches\n\n')"]
    write_cluster_batches_7["content . push_str ('Action: execute batches in order and verify after each b..."]
    write_cluster_batches_8["content . push_str ('Note: each batch creates or fills a cluster file.\n\n')"]
    write_cluster_batches_9["for (idx , plan) in plans . iter () . enumerate () { content . push_str (& fo..."]
    write_cluster_batches_10["EXIT"]
    write_cluster_batches_0 --> write_cluster_batches_1
    write_cluster_batches_1 --> write_cluster_batches_2
    write_cluster_batches_2 --> write_cluster_batches_3
    write_cluster_batches_1 --> write_cluster_batches_4
    write_cluster_batches_3 --> write_cluster_batches_5
    write_cluster_batches_4 --> write_cluster_batches_5
    write_cluster_batches_5 --> write_cluster_batches_6
    write_cluster_batches_6 --> write_cluster_batches_7
    write_cluster_batches_7 --> write_cluster_batches_8
    write_cluster_batches_8 --> write_cluster_batches_9
    write_cluster_batches_9 --> write_cluster_batches_10
```

## Function: `write_structural_batches`

- File: src/090_utilities.rs
- Branches: 1
- Loops: 0
- Nodes: 14
- Edges: 14

```mermaid
flowchart TD
    write_structural_batches_0["ENTRY"]
    write_structural_batches_1["if items . is_empty ()"]
    write_structural_batches_2["THEN BB"]
    write_structural_batches_3["return"]
    write_structural_batches_4["EMPTY ELSE"]
    write_structural_batches_5["IF JOIN"]
    write_structural_batches_6["let mut ordered_targets = Vec :: new ()"]
    write_structural_batches_7["let mut batches : HashMap < PathBuf , Vec < & PlanItem > > = HashMap :: new ()"]
    write_structural_batches_8["for item in items { let Some (target) = & item . target_file else { continue ..."]
    write_structural_batches_9["content . push_str ('### Phase 3 Batches\n\n')"]
    write_structural_batches_10["content . push_str ('Action: execute batches in order and verify after each b..."]
    write_structural_batches_11["content . push_str ('Note: each batch targets one destination module.\n\n')"]
    write_structural_batches_12["for (idx , target) in ordered_targets . iter () . enumerate () { let empty : ..."]
    write_structural_batches_13["EXIT"]
    write_structural_batches_0 --> write_structural_batches_1
    write_structural_batches_1 --> write_structural_batches_2
    write_structural_batches_2 --> write_structural_batches_3
    write_structural_batches_1 --> write_structural_batches_4
    write_structural_batches_3 --> write_structural_batches_5
    write_structural_batches_4 --> write_structural_batches_5
    write_structural_batches_5 --> write_structural_batches_6
    write_structural_batches_6 --> write_structural_batches_7
    write_structural_batches_7 --> write_structural_batches_8
    write_structural_batches_8 --> write_structural_batches_9
    write_structural_batches_9 --> write_structural_batches_10
    write_structural_batches_10 --> write_structural_batches_11
    write_structural_batches_11 --> write_structural_batches_12
    write_structural_batches_12 --> write_structural_batches_13
```

