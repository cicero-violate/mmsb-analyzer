# Structure Group: src/060_layer_core.rs

## File: src/060_layer_core.rs

- Layer(s): 060_layer_core.rs
- Language coverage: Rust (2)
- Element types: Function (2)
- Total elements: 2

### Elements

- [Rust | Function] `sort_structural_items` (line 0, pub)
  - Signature: `pub fn sort_structural_items (items : & mut Vec < crate :: report :: PlanItem >) { use std :: collections :: HashMap ...`
  - Calls: len, len, HashMap::new, enumerate, iter, push, or_default, entry, clone, structural_layer_value, structural_layer_value, Some, Some, Some, Some, push, enumerate, iter, get, push, Vec::with_capacity, collect, filter, is_empty, sort_by, structural_cmp, remove, push, saturating_sub, push, len, sort_by, Vec::with_capacity, push, clone
- [Rust | Function] `structural_cmp` (line 0, pub)
  - Signature: `pub fn structural_cmp (a : & crate :: report :: PlanItem , b : & crate :: report :: PlanItem) -> std :: cmp :: Orderi...`
  - Calls: structural_layer_value, structural_layer_value, structural_layer_value, structural_layer_value, saturating_mul, saturating_mul, then_with, then_with, then_with, then_with, then_with, cmp, cmp, cmp, cmp, cmp, cmp

