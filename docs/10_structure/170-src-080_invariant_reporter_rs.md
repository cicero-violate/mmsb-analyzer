# Structure Group: src/080_invariant_reporter.rs

## File: src/080_invariant_reporter.rs

- Layer(s): 080_invariant_reporter.rs
- Language coverage: Rust (5)
- Element types: Function (4), Module (1)
- Total elements: 5

### Elements

- [Rust | Function] `export_constraints_json` (line 0, pub)
  - Signature: `# [doc = " Export refactoring constraints to JSON"] pub fn export_constraints_json (constraints : & [RefactorConstrai...`
  - Calls: join, fs::create_dir_all, join, map_err, serde_json::to_string_pretty, std::io::Error::new, fs::write, Ok
- [Rust | Function] `export_json` (line 0, pub)
  - Signature: `# [doc = " Export invariants to JSON for agent consumption"] pub fn export_json (result : & InvariantAnalysisResult ,...`
  - Calls: join, map_err, serde_json::to_string_pretty, std::io::Error::new, fs::write, Ok
- [Rust | Function] `generate_invariant_report` (line 0, pub)
  - Signature: `# [doc = " Generate invariant report in markdown format"] pub fn generate_invariant_report (result : & InvariantAnaly...`
  - Calls: join, fs::create_dir_all, join, String::new, push_str, push_str, push_str, push_str, push_str, push_str, push_str, push_str, push_str, push_str, push_str, push_str, push_str, push_str, push_str, collect, filter, iter, is_empty, push_str, push_str, push_str, push_str, push_str, is_empty, push_str, push_str, push_str, push_str, push_str, collect, filter, iter, is_empty, push_str, take, iter, push_str, push_str, push_str, push_str, push_str, len, push_str, push_str, push_str, collect, filter, iter, is_empty, push_str, take, iter, push_str, len, push_str, push_str, is_empty, push_str, push_str, collect, filter, iter, sort_by_key, is_empty, push_str, push_str, push_str, push_str, is_empty, push_str, push_str, collect, iter, sort_by_key, take, iter, push_str, len, push_str, push_str, fs::write, export_json, join, crate::conscience_graph::generate_conscience_map, Ok
- [Rust | Function] `test_generate_report` (line 0, priv)
  - Signature: `# [test] fn test_generate_report () { let result = InvariantAnalysisResult { invariants : vec ! [Invariant :: new ("t...`
  - Calls: Vec::new, HashMap::new, join, std::env::temp_dir, unwrap, fs::create_dir_all, generate_invariant_report, fs::remove_dir_all
- [Rust | Module] `tests` (line 0, priv)

