# Structure Group: src/570_verification_scope_planner.rs

## File: src/570_verification_scope_planner.rs

- Layer(s): 570_verification_scope_planner.rs
- Language coverage: Rust (4)
- Element types: Function (4)
- Total elements: 4

### Elements

- [Rust | Function] `action_module` (line 0, priv)
  - Signature: `fn action_module (action : & RefactorAction) -> String { match action { RefactorAction :: MoveFunction { to , .. } =>...`
  - Calls: to_string, display, to_string, display, to_string, display, to_string, display, to_string, display
- [Rust | Function] `affected_files` (line 0, priv)
  - Signature: `fn affected_files (action : & RefactorAction) -> Vec < std :: path :: PathBuf > { match action { RefactorAction :: Mo...`
- [Rust | Function] `estimate_verification_time` (line 0, priv)
  - Signature: `fn estimate_verification_time (tier : & ErrorTier) -> u32 { match tier { ErrorTier :: Trivial => 10 , ErrorTier :: Mo...`
- [Rust | Function] `plan_verification_scope` (line 0, pub)
  - Signature: `pub fn plan_verification_scope (action : & RefactorAction , correction_plan : & CorrectionPlan ,) -> VerificationPoli...`
  - Calls: len, affected_files, action_module, push, clone, estimate_verification_time

