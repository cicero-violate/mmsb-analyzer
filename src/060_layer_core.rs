//! Core layer computation functions - lowest level utilities
//! This module must have no dependencies on other local modules to avoid circular dependencies.
//! It provides fundamental layer prefix extraction and violation detection.

#[allow(unused_imports)]
pub use crate::cluster_006::{
    layer_prefix_value, order_directories, collect_directory_moves,
};
#[allow(unused_imports)]
pub use crate::cluster_008::detect_layer_violations;
#[allow(unused_imports)]
pub use crate::cluster_001::{layer_constrained_sort, topo_sort_within};
use crate::cluster_008::structural_layer_value;

pub fn structural_cmp(a: &crate::report::PlanItem, b: &crate::report::PlanItem) -> std::cmp::Ordering {
    let a_required = structural_layer_value(&a.required_layer, i32::MAX);
    let b_required = structural_layer_value(&b.required_layer, i32::MAX);
    let a_current = structural_layer_value(&a.current_layer, i32::MIN);
    let b_current = structural_layer_value(&b.current_layer, i32::MIN);
    let a_benefit = if a.cost == 0 {
        0
    } else {
        (a.benefit.saturating_mul(1000)) / a.cost
    };
    let b_benefit = if b.cost == 0 {
        0
    } else {
        (b.benefit.saturating_mul(1000)) / b.cost
    };
    a_required
        .cmp(&b_required)
        .then_with(|| b.is_utility.cmp(&a.is_utility))
        .then_with(|| b_benefit.cmp(&a_benefit))
        .then_with(|| b.impact_weight.cmp(&a.impact_weight))
        .then_with(|| b_current.cmp(&a_current))
        .then_with(|| a.description.cmp(&b.description))
}

pub fn sort_structural_items(items: &mut Vec<crate::report::PlanItem>) {
    use std::collections::HashMap;
    use std::path::PathBuf;

    if items.len() <= 1 {
        return;
    }

    let count = items.len();
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); count];
    let mut indegree = vec![0usize; count];

    let mut file_to_items: HashMap<PathBuf, Vec<usize>> = HashMap::new();
    for (idx, item) in items.iter().enumerate() {
        if let Some(path) = &item.current_file {
            file_to_items.entry(path.clone()).or_default().push(idx);
        }
    }

    for i in 0..count {
        for j in (i + 1)..count {
            let req_i = structural_layer_value(&items[i].required_layer, i32::MAX);
            let req_j = structural_layer_value(&items[j].required_layer, i32::MAX);
            let mut edge = None;
            if req_i != req_j {
                edge = if req_i < req_j { Some((i, j)) } else { Some((j, i)) };
            } else if items[i].is_utility != items[j].is_utility {
                edge = if items[i].is_utility {
                    Some((i, j))
                } else {
                    Some((j, i))
                };
            }
            if let Some((from, to)) = edge {
                edges[from].push(to);
                indegree[to] += 1;
            }
        }
    }

    for (idx, item) in items.iter().enumerate() {
        for file in &item.outgoing_files {
            if let Some(dependents) = file_to_items.get(file) {
                for &dependent_idx in dependents {
                    if dependent_idx == idx {
                        continue;
                    }
                    edges[dependent_idx].push(idx);
                    indegree[idx] += 1;
                }
            }
        }
    }

    let mut ordered_indices = Vec::with_capacity(count);
    let mut available: Vec<usize> = (0..count).filter(|&i| indegree[i] == 0).collect();
    while !available.is_empty() {
        available.sort_by(|&a, &b| structural_cmp(&items[a], &items[b]));
        let next = available.remove(0);
        ordered_indices.push(next);
        for &neighbor in &edges[next] {
            indegree[neighbor] = indegree[neighbor].saturating_sub(1);
            if indegree[neighbor] == 0 {
                available.push(neighbor);
            }
        }
    }

    if ordered_indices.len() != count {
        items.sort_by(structural_cmp);
        return;
    }

    let mut reordered = Vec::with_capacity(count);
    for idx in ordered_indices {
        reordered.push(items[idx].clone());
    }
    *items = reordered;
}

// Moved to layer_utilities in Batch 4 - no re-export needed
