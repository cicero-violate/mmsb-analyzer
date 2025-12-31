# Structure Group: src/030_fixpoint_solver.rs

## File: src/030_fixpoint_solver.rs

- Layer(s): 030_fixpoint_solver.rs
- Language coverage: Rust (9)
- Element types: Function (4), Impl (2), Module (1), Struct (2)
- Total elements: 9

### Elements

- [Rust | Impl] `Default for impl Default for SymbolicAbstraction { fn default () -> Self { Self :: new () } } . self_ty` (line 0, priv)
- [Rust | Struct] `FixpointResult` (line 0, pub)
  - Signature: `# [doc = " Result of fixpoint computation"] # [derive (Debug)] pub struct FixpointResult { # [doc = " Final abstracti...`
- [Rust | Struct] `SymbolicAbstraction` (line 0, pub)
  - Signature: `# [doc = " Symbolic abstraction of program state"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct SymbolicAbs...`
- [Rust | Impl] `impl SymbolicAbstraction { pub fn new () -> Self { Self { type_sig : None , effects : HashSet :: new () , layer : None , visibility : None , properties : HashSet :: new () , } } # [doc = " Check if two abstractions are approximately equal"] pub fn approx_eq (& self , other : & Self) -> bool { self . type_sig == other . type_sig && self . effects == other . effects && self . layer == other . layer && self . visibility == other . visibility && self . properties == other . properties } # [doc = " Merge two abstractions (used during propagation)"] pub fn merge (& mut self , other : & Self) { if self . type_sig . is_none () && other . type_sig . is_some () { self . type_sig = other . type_sig . clone () ; } self . effects . extend (other . effects . clone ()) ; match (self . layer , other . layer) { (Some (l1) , Some (l2)) => self . layer = Some (l1 . max (l2)) , (None , Some (l)) => self . layer = Some (l) , _ => { } } if self . visibility . is_none () && other . visibility . is_some () { self . visibility = other . visibility . clone () ; } self . properties . extend (other . properties . clone ()) ; } } . self_ty` (line 0, priv)
- [Rust | Function] `propagate_to_fixpoint` (line 0, pub)
  - Signature: `# [doc = " Propagate symbolic abstractions until fixpoint"] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `graph...`
  - Calls: HashMap::new, node_indices, insert, clone, HashMap::new, get, insert, clone, clone, node_indices, unwrap_or_else, cloned, get, neighbors_directed, get, merge, get, approx_eq, insert, HashMap::new, Vec::new, clone, get, approx_eq, push, clone, insert, clone
- [Rust | Function] `test_fixpoint_convergence` (line 0, priv)
  - Signature: `# [test] fn test_fixpoint_convergence () { let mut graph = DiGraph :: new () ; let a = graph . add_node ("A" . to_str...`
  - Calls: DiGraph::new, add_node, to_string, add_node, to_string, add_edge, HashMap::new, insert, to_string, SymbolicAbstraction::new, insert, to_string, SymbolicAbstraction::new, propagate_to_fixpoint
- [Rust | Function] `test_fixpoint_simple` (line 0, priv)
  - Signature: `# [test] fn test_fixpoint_simple () { let mut graph = DiGraph :: new () ; let a = graph . add_node ("A" . to_string (...`
  - Calls: DiGraph::new, add_node, to_string, add_node, to_string, add_node, to_string, add_edge, add_edge, HashMap::new, SymbolicAbstraction::new, insert, to_string, insert, to_string, insert, to_string, SymbolicAbstraction::new, insert, to_string, SymbolicAbstraction::new, propagate_to_fixpoint
- [Rust | Function] `test_symbolic_abstraction_merge` (line 0, priv)
  - Signature: `# [test] fn test_symbolic_abstraction_merge () { let mut abs1 = SymbolicAbstraction :: new () ; abs1 . type_sig = Som...`
  - Calls: SymbolicAbstraction::new, Some, to_string, insert, to_string, Some, SymbolicAbstraction::new, insert, to_string, Some, merge
- [Rust | Module] `tests` (line 0, priv)

