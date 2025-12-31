# Structure Group: src/082_conscience_graph.rs

## File: src/082_conscience_graph.rs

- Layer(s): 082_conscience_graph.rs
- Language coverage: Rust (9)
- Element types: Function (7), Module (1), Struct (1)
- Total elements: 9

### Elements

- [Rust | Struct] `ConscienceStats` (line 0, pub)
  - Signature: `pub struct ConscienceStats { pub total_functions : usize , pub protected_functions : usize , pub total_invariants : u...`
- [Rust | Function] `generate_conscience_map` (line 0, pub)
  - Signature: `# [doc = " Generate conscience map showing protection levels per function"] pub fn generate_conscience_map (invariant...`
  - Calls: String::new, push_str, push_str, push_str, push_str, HashMap::new, push, or_default, entry, clone, len, count, filter, values, any, iter, is_blocking, push_str, push_str, collect, into_iter, sort_by_key, count, filter, iter, is_blocking, push_str, push_str, count, filter, iter, is_blocking, len, push_str, first, is_empty, push_str, filter, iter, is_blocking, push_str, push_str, push_str, push_str, push_str, push_str, push_str, push_str, std::fs::write, Ok
- [Rust | Function] `generate_conscience_stats` (line 0, pub)
  - Signature: `# [doc = " Generate summary statistics"] pub fn generate_conscience_stats (invariants : & [Invariant]) -> ConscienceS...`
  - Calls: HashMap::new, push, or_default, entry, clone, len, count, filter, values, any, iter, is_blocking, count, filter, iter, count, filter, iter, count, filter, iter, len, count, filter, iter, is_blocking
- [Rust | Function] `kind_name` (line 0, priv)
  - Signature: `# [doc = " Get short name for invariant kind"] fn kind_name (inv : & Invariant) -> String { match & inv . kind { Inva...`
  - Calls: to_string, to_string, to_string, to_string, to_string, to_string, to_string, to_string, to_string
- [Rust | Function] `make_test_invariant` (line 0, priv)
  - Signature: `fn make_test_invariant (name : & str , kind : InvariantKind , strength : InvariantStrength ,) -> Invariant { Invarian...`
  - Calls: Invariant::new, to_string, to_string, to_string
- [Rust | Function] `strength_emoji` (line 0, priv)
  - Signature: `# [doc = " Get emoji for invariant strength"] fn strength_emoji (inv : & Invariant) -> & 'static str { match inv . st...`
- [Rust | Function] `test_generate_stats` (line 0, priv)
  - Signature: `# [test] fn test_generate_stats () { let invariants = vec ! [make_test_invariant ("fn1" , InvariantKind :: Structural...`
  - Calls: generate_conscience_stats
- [Rust | Function] `test_strength_emoji` (line 0, priv)
  - Signature: `# [test] fn test_strength_emoji () { let proven = make_test_invariant ("test" , InvariantKind :: Structural (Structur...`
  - Calls: make_test_invariant, InvariantKind::Structural, make_test_invariant, InvariantKind::Semantic, to_string, make_test_invariant, InvariantKind::Semantic
- [Rust | Module] `tests` (line 0, priv)

