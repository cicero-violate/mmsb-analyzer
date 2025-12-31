# Structure Group: src/020_layer_inference.rs

## File: src/020_layer_inference.rs

- Layer(s): 020_layer_inference.rs
- Language coverage: Rust (6)
- Element types: Function (5), Module (1)
- Total elements: 6

### Elements

- [Rust | Function] `detect_layer_violations` (line 0, pub)
  - Signature: `# [doc = " Detect layer violations in the call graph"] # [doc = ""] # [doc = " A violation occurs when a lower-layer ...`
  - Calls: Vec::new, edge_references, source, target, get, get, push, clone, clone
- [Rust | Function] `infer_layers` (line 0, pub)
  - Signature: `# [doc = " Infer layers from call graph structure"] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `graph` - Call...`
  - Calls: HashMap::new, HashMap::new, node_indices, count, neighbors_directed, insert, node_indices, contains_key, collect, neighbors_directed, all, iter, contains_key, is_empty, unwrap_or, copied, max, filter_map, iter, get, insert, unwrap_or, copied, max, values, node_indices, contains_key, insert, clone, collect, map, neighbors_directed, clone, is_empty, max, filter_map, iter, copied, and_then, find, node_indices, get, insert, clone
- [Rust | Function] `test_detect_layer_violations_none` (line 0, priv)
  - Signature: `# [test] fn test_detect_layer_violations_none () { let mut graph = DiGraph :: new () ; let a = graph . add_node ("A" ...`
  - Calls: DiGraph::new, add_node, to_string, add_node, to_string, add_node, to_string, add_edge, add_edge, infer_layers, detect_layer_violations
- [Rust | Function] `test_layer_inference_diamond` (line 0, priv)
  - Signature: `# [test] fn test_layer_inference_diamond () { let mut graph = DiGraph :: new () ; let a = graph . add_node ("A" . to_...`
  - Calls: DiGraph::new, add_node, to_string, add_node, to_string, add_node, to_string, add_node, to_string, add_edge, add_edge, add_edge, add_edge, infer_layers
- [Rust | Function] `test_layer_inference_simple_dag` (line 0, priv)
  - Signature: `# [test] fn test_layer_inference_simple_dag () { let mut graph = DiGraph :: new () ; let a = graph . add_node ("A" . ...`
  - Calls: DiGraph::new, add_node, to_string, add_node, to_string, add_node, to_string, add_edge, add_edge, infer_layers
- [Rust | Module] `tests` (line 0, priv)

