# Structure Group: src/005_refactor_constraints.rs

## File: src/005_refactor_constraints.rs

- Layer(s): 005_refactor_constraints.rs
- Language coverage: Rust (11)
- Element types: Enum (1), Function (7), Impl (2), Module (1)
- Total elements: 11

### Elements

- [Rust | Impl] `Display for impl fmt :: Display for RefactorConstraint { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { RefactorConstraint :: NoMove { target , reason , strength } => { write ! (f , "[{}] NoMove: {} (reason: {})" , strength , target , reason) } RefactorConstraint :: PreserveSignature { target , signature , strength } => { write ! (f , "[{}] PreserveSignature: {} -> {}" , strength , target , signature) } RefactorConstraint :: FixedLayer { target , layer , strength } => { write ! (f , "[{}] FixedLayer: {} @ layer {}" , strength , target , layer) } RefactorConstraint :: PreserveOrdering { target , must_come_before , strength } => { write ! (f , "[{}] PreserveOrdering: {} before {:?}" , strength , target , must_come_before) } RefactorConstraint :: MustPreserve { target , facts , strength } => { write ! (f , "[{}] MustPreserve: {} -> {} facts" , strength , target , facts . len ()) } RefactorConstraint :: NoDelete { target , dependents , strength } => { write ! (f , "[{}] NoDelete: {} (dependents: {})" , strength , target , dependents . len ()) } RefactorConstraint :: PreserveDegree { target , in_degree , out_degree , strength } => { write ! (f , "[{}] PreserveDegree: {} (in={}, out={})" , strength , target , in_degree , out_degree) } } } } . self_ty` (line 0, priv)
- [Rust | Enum] `RefactorConstraint` (line 0, pub)
- [Rust | Function] `check_move_allowed` (line 0, pub)
  - Signature: `# [doc = " Check if a move operation is allowed by constraints"] pub fn check_move_allowed (target : & str , current_...`
  - Calls: target, is_blocking, Err, Err, Ok
- [Rust | Function] `from_invariant` (line 0, pub)
  - Signature: `# [doc = " Convert an invariant to a refactoring constraint"] pub fn from_invariant (invariant : & Invariant) -> Opti...`
  - Calls: Some, clone, Some, clone, Some, clone, to_string, Some, clone, Some, clone, to_string, Some, clone, clone, Some, clone, to_string, Some, clone, to_string, Some, clone, Vec::new, Some, clone, collect, cloned, iter
- [Rust | Function] `generate_constraints` (line 0, pub)
  - Signature: `# [doc = " Generate all constraints from an invariant analysis result"] pub fn generate_constraints (analysis : & Inv...`
  - Calls: collect, filter_map, iter
- [Rust | Impl] `impl RefactorConstraint { # [doc = " Get the target of this constraint"] pub fn target (& self) -> & str { match self { RefactorConstraint :: NoMove { target , .. } => target , RefactorConstraint :: PreserveSignature { target , .. } => target , RefactorConstraint :: FixedLayer { target , .. } => target , RefactorConstraint :: PreserveOrdering { target , .. } => target , RefactorConstraint :: MustPreserve { target , .. } => target , RefactorConstraint :: NoDelete { target , .. } => target , RefactorConstraint :: PreserveDegree { target , .. } => target , } } # [doc = " Get the strength of the invariant backing this constraint"] pub fn strength (& self) -> & InvariantStrength { match self { RefactorConstraint :: NoMove { strength , .. } => strength , RefactorConstraint :: PreserveSignature { strength , .. } => strength , RefactorConstraint :: FixedLayer { strength , .. } => strength , RefactorConstraint :: PreserveOrdering { strength , .. } => strength , RefactorConstraint :: MustPreserve { strength , .. } => strength , RefactorConstraint :: NoDelete { strength , .. } => strength , RefactorConstraint :: PreserveDegree { strength , .. } => strength , } } # [doc = " Check if this constraint should block a refactoring"] pub fn is_blocking (& self) -> bool { match self . strength () { InvariantStrength :: Proven => true , InvariantStrength :: Empirical { paths_checked } => * paths_checked >= 10 , InvariantStrength :: Heuristic => false , } } } . self_ty` (line 0, priv)
- [Rust | Function] `test_check_move_allowed_blocking` (line 0, priv)
  - Signature: `# [test] fn test_check_move_allowed_blocking () { let constraints = vec ! [RefactorConstraint :: NoMove { target : "t...`
  - Calls: check_move_allowed
- [Rust | Function] `test_check_move_allowed_non_blocking` (line 0, priv)
  - Signature: `# [test] fn test_check_move_allowed_non_blocking () { let constraints = vec ! [RefactorConstraint :: NoMove { target ...`
  - Calls: check_move_allowed
- [Rust | Function] `test_constraint_is_blocking` (line 0, priv)
  - Signature: `# [test] fn test_constraint_is_blocking () { let proven = RefactorConstraint :: NoMove { target : "fn1" . to_string (...`
  - Calls: to_string, to_string, to_string, to_string
- [Rust | Function] `test_from_invariant_layer_fixed` (line 0, priv)
  - Signature: `# [test] fn test_from_invariant_layer_fixed () { let inv = Invariant :: new ("test_fn" . to_string () , "test.rs" . t...`
  - Calls: Invariant::new, to_string, to_string, InvariantKind::Structural, to_string, unwrap, from_invariant
- [Rust | Module] `tests` (line 0, priv)

