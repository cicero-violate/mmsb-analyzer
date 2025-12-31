# Structure Group: src/380_dead_code_call_graph.rs

## File: src/380_dead_code_call_graph.rs

- Layer(s): 380_dead_code_call_graph.rs
- Language coverage: Rust (6)
- Element types: Function (6)
- Total elements: 6

### Elements

- [Rust | Function] `build_call_graph` (line 0, pub)
  - Signature: `pub fn build_call_graph (elements : & [CodeElement]) -> CallGraph { let mut graph : CallGraph = HashMap :: new () ; f...`
  - Calls: HashMap::new, or_default, entry, clone, extend, cloned, iter
- [Rust | Function] `build_reverse_call_graph` (line 0, pub)
  - Signature: `pub fn build_reverse_call_graph (graph : & CallGraph) -> CallGraph { let mut reverse : CallGraph = HashMap :: new () ...`
  - Calls: HashMap::new, push, or_default, entry, clone, clone
- [Rust | Function] `classify_symbol` (line 0, pub)
  - Signature: `pub fn classify_symbol (symbol : & str , call_graph : & CallGraph , intent_map : & IntentMap , test_boundaries : & Te...`
  - Calls: contains_key, is_test_only, is_reachable
- [Rust | Function] `compute_reachability` (line 0, pub)
  - Signature: `pub fn compute_reachability (graph : & CallGraph , entrypoints : & HashSet < String >) -> HashSet < String > { let mu...`
  - Calls: HashSet::new, collect, cloned, iter, pop_front, insert, clone, get, contains, push_back, clone
- [Rust | Function] `is_reachable` (line 0, pub)
  - Signature: `pub fn is_reachable (symbol : & str , graph : & CallGraph , entrypoints : & HashSet < String > ,) -> bool { if entryp...`
  - Calls: is_empty, contains, compute_reachability
- [Rust | Function] `is_test_only` (line 0, pub)
  - Signature: `pub fn is_test_only (symbol : & str , call_graph : & CallGraph , test_boundaries : & TestBoundaries ,) -> bool { if t...`
  - Calls: contains, build_reverse_call_graph, get, is_empty, all, iter, contains

