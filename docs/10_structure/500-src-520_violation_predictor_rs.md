# Structure Group: src/520_violation_predictor.rs

## File: src/520_violation_predictor.rs

- Layer(s): 520_violation_predictor.rs
- Language coverage: Rust (7)
- Element types: Function (7)
- Total elements: 7

### Elements

- [Rust | Function] `find_callers` (line 0, priv)
  - Signature: `fn find_callers (function : & str , call_graph : & HashMap < String , CallGraphNode > , elements : & [CodeElement] ,)...`
  - Calls: HashSet::new, get, find_element_file, insert, collect, into_iter
- [Rust | Function] `find_element_file` (line 0, priv)
  - Signature: `fn find_element_file (function : & str , elements : & [CodeElement]) -> Option < PathBuf > { elements . iter () . fin...`
  - Calls: map, find, iter, PathBuf::from
- [Rust | Function] `find_reference_files` (line 0, priv)
  - Signature: `fn find_reference_files (function : & str , call_graph : & HashMap < String , CallGraphNode > , elements : & [CodeEle...`
  - Calls: HashSet::new, any, iter, find_element_file, insert, collect, into_iter
- [Rust | Function] `generate_intelligence_report` (line 0, pub)
  - Signature: `pub fn generate_intelligence_report (actions : & [RefactorAction] , state : & IntelligenceState < '_ > ,) -> Correcti...`
  - Calls: Vec::new, Vec::new, Vec::new, Vec::new, predict_violations, fill_prediction_confidence, generate_correction_plan, augment_path_coherence_strategies, plan_verification_scope, build_rollback_criteria, estimate_impact, clone, push, push, push, push, compute_summary, to_string, to_rfc3339, chrono::Utc::now, clone, len
- [Rust | Function] `move_violates_invariant` (line 0, priv)
  - Signature: `fn move_violates_invariant (_function : & str , _from : & PathBuf , _to : & PathBuf , _invariants : & InvariantAnalys...`
- [Rust | Function] `predict_violations` (line 0, pub)
  - Signature: `pub fn predict_violations (action : & RefactorAction , invariants : & InvariantAnalysisResult , call_graph : & HashMa...`
  - Calls: Vec::new, find_callers, is_empty, push, is_empty, push, move_violates_invariant, push, symbol_exists, push, find_reference_files, is_empty, push, push, push, push
- [Rust | Function] `symbol_exists` (line 0, priv)
  - Signature: `fn symbol_exists (symbol : & str , elements : & [CodeElement]) -> bool { elements . iter () . any (| el | el . name =...`
  - Calls: any, iter

