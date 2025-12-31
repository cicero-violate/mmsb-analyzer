# Structure Group: src/560_correction_plan_generator.rs

## File: src/560_correction_plan_generator.rs

- Layer(s): 560_correction_plan_generator.rs
- Language coverage: Rust (9)
- Element types: Function (9)
- Total elements: 9

### Elements

- [Rust | Function] `action_function` (line 0, priv)
  - Signature: `fn action_function (action : & RefactorAction) -> Option < String > { match action { RefactorAction :: MoveFunction {...`
  - Calls: Some, clone
- [Rust | Function] `action_module_path` (line 0, priv)
  - Signature: `fn action_module_path (action : & RefactorAction) -> String { match action { RefactorAction :: MoveFunction { to , .....`
  - Calls: to_string, display, to_string, display, to_string, display, to_string, display, to_string
- [Rust | Function] `action_refs` (line 0, priv)
  - Signature: `fn action_refs (action : & RefactorAction) -> Option < (String , String) > { match action { RefactorAction :: RenameF...`
  - Calls: Some, clone, clone, Some, to_string, display, to_string, display
- [Rust | Function] `action_symbol` (line 0, priv)
  - Signature: `fn action_symbol (action : & RefactorAction) -> Option < String > { match action { RefactorAction :: MoveFunction { f...`
  - Calls: Some, clone, Some, clone, Some, clone
- [Rust | Function] `action_target_layer` (line 0, priv)
  - Signature: `fn action_target_layer (action : & RefactorAction) -> Option < String > { match action { RefactorAction :: MoveFuncti...`
  - Calls: clone
- [Rust | Function] `action_visibility` (line 0, priv)
  - Signature: `fn action_visibility (action : & RefactorAction ,) -> Option < (String , std :: path :: PathBuf , crate :: types :: V...`
  - Calls: Some, clone, clone, clone, clone, clone
- [Rust | Function] `average_confidence` (line 0, priv)
  - Signature: `fn average_confidence (predictions : & [ViolationPrediction]) -> f64 { if predictions . is_empty () { return 1.0 ; } ...`
  - Calls: is_empty, sum, map, iter, len
- [Rust | Function] `estimate_fix_time` (line 0, priv)
  - Signature: `fn estimate_fix_time (count : usize) -> u32 { 10 + (count as u32 * 5) } . sig`
- [Rust | Function] `generate_correction_plan` (line 0, pub)
  - Signature: `pub fn generate_correction_plan (action : & RefactorAction , predictions : & [ViolationPrediction] ,) -> CorrectionPl...`
  - Calls: Vec::new, action_symbol, push, action_module_path, action_refs, push, action_refs, push, clone, clone, clone, action_symbol, push, to_string, action_target_layer, action_function, push, action_function, action_target_layer, push, action_visibility, starts_with, push, to_string, push, push, unwrap_or, max, map, iter, action_id, to_vec, average_confidence, estimate_fix_time, len

