# Structure Group: src/630_correction_intelligence_report.rs

## File: src/630_correction_intelligence_report.rs

- Layer(s): 630_correction_intelligence_report.rs
- Language coverage: Rust (12)
- Element types: Function (9), Struct (3)
- Total elements: 12

### Elements

- [Rust | Struct] `CorrectionIntelligenceReport` (line 0, pub)
  - Signature: `# [derive (Clone , Debug , Serialize , Deserialize)] pub struct CorrectionIntelligenceReport { pub version : String ,...`
- [Rust | Struct] `CorrectionSummary` (line 0, pub)
  - Signature: `# [derive (Clone , Debug , Serialize , Deserialize)] pub struct CorrectionSummary { pub trivial_count : usize , pub m...`
- [Rust | Struct] `IntelligenceState` (line 0, pub)
  - Signature: `# [derive (Clone , Debug)] pub struct IntelligenceState < 'a > { pub root : PathBuf , pub invariants : & 'a Invariant...`
  - Generics: 'a
- [Rust | Function] `augment_path_coherence_strategies` (line 0, pub(crate))
  - Signature: `pub (crate) fn augment_path_coherence_strategies (plan : & mut CorrectionPlan , action : & RefactorAction , root : & ...`
  - Calls: module_name_from_path, module_name_from_path, unwrap_or, and_then, file_name, to_str, unwrap_or, and_then, file_name, to_str, ok, Regex::new, ok, Regex::new, is_empty, is_empty, ok, Regex::new, Vec::new, HashSet::new, crate::cluster_010::gather_rust_files, fs::read_to_string, lines, is_match, replace, clone, to_string, clone, insert, clone, push, is_match, replace, clone, to_string, clone, insert, clone, push, is_match, contains, replace, clone, to_string, clone, insert, clone, push, sort_by, then_with, then_with, cmp, cmp, cmp, push
- [Rust | Function] `build_state` (line 0, pub)
  - Signature: `pub fn build_state < 'a > (root : & 'a Path , analysis : & 'a AnalysisResult , metrics : Metrics ,) -> IntelligenceSt...`
  - Generics: 'a
  - Calls: to_path_buf
- [Rust | Function] `compute_summary` (line 0, pub(crate))
  - Signature: `pub (crate) fn compute_summary (plans : & [CorrectionPlan] , deltas : & [QualityDelta]) -> CorrectionSummary { let mu...`
  - Calls: len, is_empty, len
- [Rust | Function] `default_confidence` (line 0, priv)
  - Signature: `fn default_confidence (violation_type : & crate :: correction_plan_types :: ViolationType) -> f64 { match violation_t...`
- [Rust | Function] `fill_prediction_confidence` (line 0, pub(crate))
  - Signature: `pub (crate) fn fill_prediction_confidence (predictions : & mut [ViolationPrediction]) { for prediction in predictions...`
  - Calls: default_confidence
- [Rust | Function] `filter_path_coherence_report` (line 0, pub)
  - Signature: `pub fn filter_path_coherence_report (report : & CorrectionIntelligenceReport ,) -> CorrectionIntelligenceReport { let...`
  - Calls: Vec::new, Vec::new, Vec::new, Vec::new, enumerate, iter, trim_start, starts_with, starts_with, starts_with, starts_with, starts_with, push, clone, get, push, clone, get, push, clone, get, push, clone, compute_summary, clone, clone, clone, len
- [Rust | Function] `filter_visibility_report` (line 0, pub)
  - Signature: `pub fn filter_visibility_report (report : & CorrectionIntelligenceReport ,) -> CorrectionIntelligenceReport { let mut...`
  - Calls: Vec::new, Vec::new, Vec::new, Vec::new, enumerate, iter, starts_with, push, clone, get, push, clone, get, push, clone, get, push, clone, compute_summary, clone, clone, clone, len
- [Rust | Function] `module_name_from_path` (line 0, priv)
  - Signature: `fn module_name_from_path (path : & Path) -> Option < String > { let stem = path . file_stem () . and_then (| s | s . ...`
  - Calls: and_then, file_stem, to_str, to_string, and_then, and_then, parent, file_name, to_str, to_string, Some, crate::cluster_010::normalize_module_name
- [Rust | Function] `write_intelligence_outputs` (line 0, pub)
  - Signature: `pub fn write_intelligence_outputs (report : & CorrectionIntelligenceReport , output_dir : & Path ,) -> std :: io :: R...`
  - Calls: write_intelligence_outputs_at

