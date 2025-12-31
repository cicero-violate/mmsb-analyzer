# Structure Group: src/083_action_validator.rs

## File: src/083_action_validator.rs

- Layer(s): 083_action_validator.rs
- Language coverage: Rust (14)
- Element types: Enum (2), Function (10), Module (1), Struct (1)
- Total elements: 14

### Elements

- [Rust | Enum] `AgentAction` (line 0, pub)
- [Rust | Struct] `ConstraintViolation` (line 0, pub)
  - Signature: `# [doc = " Constraint violation with explanation"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct Co...`
- [Rust | Enum] `ViolationSeverity` (line 0, pub)
- [Rust | Function] `check_move_allowed` (line 0, pub)
  - Signature: `# [doc = " Check if a specific move is allowed"] pub fn check_move_allowed (name : & str , from : & PathBuf , to : & ...`
  - Calls: to_string, clone, clone, validate_action, Ok, collect, map, iter, clone, Err, join
- [Rust | Function] `extract_layer` (line 0, priv)
  - Signature: `# [doc = " Extract layer number from file path (e.g., \"src/040_test.rs\" -> Some(40))"] fn extract_layer (path : & P...`
  - Calls: and_then, and_then, file_name, to_str, collect, split, is_empty, ok, parse
- [Rust | Function] `matches_function` (line 0, priv)
  - Signature: `# [doc = " Check if function name matches constraint target"] fn matches_function (action_name : & str , constraint :...`
  - Calls: target
- [Rust | Function] `test_check_move_allowed` (line 0, priv)
  - Signature: `# [test] fn test_check_move_allowed () { let constraints = vec ! [RefactorConstraint :: NoMove { target : "test_fn" ....`
  - Calls: check_move_allowed, PathBuf::from, PathBuf::from
- [Rust | Function] `test_extract_layer` (line 0, priv)
  - Signature: `# [test] fn test_extract_layer () { assert_eq ! (extract_layer (& PathBuf :: from ("src/040_test.rs")) , Some (40)) ;...`
- [Rust | Function] `test_validate_allowed_action` (line 0, priv)
  - Signature: `# [test] fn test_validate_allowed_action () { let constraints = vec ! [RefactorConstraint :: NoMove { target : "other...`
  - Calls: to_string, PathBuf::from, PathBuf::from, validate_action
- [Rust | Function] `test_validate_layer_fixed_constraint` (line 0, priv)
  - Signature: `# [test] fn test_validate_layer_fixed_constraint () { let constraints = vec ! [RefactorConstraint :: FixedLayer { tar...`
  - Calls: to_string, PathBuf::from, PathBuf::from, validate_action
- [Rust | Function] `test_validate_no_move_constraint` (line 0, priv)
  - Signature: `# [test] fn test_validate_no_move_constraint () { let constraints = vec ! [RefactorConstraint :: NoMove { target : "t...`
  - Calls: to_string, PathBuf::from, PathBuf::from, validate_action, unwrap_err
- [Rust | Function] `test_validate_preserve_signature` (line 0, priv)
  - Signature: `# [test] fn test_validate_preserve_signature () { let constraints = vec ! [RefactorConstraint :: PreserveSignature { ...`
  - Calls: to_string, to_string, to_string, PathBuf::from, validate_action
- [Rust | Module] `tests` (line 0, priv)
- [Rust | Function] `validate_action` (line 0, pub)
  - Signature: `# [doc = " Validate action against all constraints"] pub fn validate_action (action : & AgentAction , constraints : &...`
  - Calls: Vec::new, enumerate, iter, push, extract_layer, extract_layer, push, push, contains, push, is_empty, Ok, Err

