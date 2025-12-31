//! Utility functions shared across modules

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use crate::types::{DirectoryAnalysis, FunctionPlacement, PlacementStatus};
use crate::report::{PlanItem, Priority, ActionKind, ClusterPlan};

/// Compress absolute paths to MMSB-relative format
pub fn compress_path(path: &str) -> String {
    // Find MMSB in the path and return everything from there
    if let Some(idx) = path.find("/MMSB/") {
        return format!("MMSB{}", &path[idx + 5..]);
    }
    // If already starts with MMSB/, return as-is
    if path.starts_with("MMSB/") {
        return path.to_string();
    }
    // Fallback: try to find src/ or other common markers
    if let Some(idx) = path.rfind("/src/") {
        return format!("MMSB/src{}", &path[idx + 4..]);
    }
    // Last resort: return original
    path.to_string()
}

// Layer helpers live in 070_layer_utilities.rs.

pub fn collect_directory_files(directory: &DirectoryAnalysis, out: &mut Vec<PathBuf>) {
    out.extend(directory.files.iter().cloned());
    for sub in &directory.subdirectories {
        collect_directory_files(sub, out);
    }
}

pub fn path_common_prefix_len(a: &Path, b: &Path) -> isize {
    let mut count = 0isize;
    for (a_comp, b_comp) in a.components().zip(b.components()) {
        if a_comp == b_comp {
            count += 1;
        } else {
            break;
        }
    }
    count
}

pub fn resolve_required_layer_path(
    required_layer: &str,
    current_file: &Path,
    directory: &DirectoryAnalysis,
    root_path: &Path,
) -> PathBuf {
    let mut files = Vec::new();
    collect_directory_files(directory, &mut files);
    let candidates = files
        .into_iter()
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name == required_layer)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return current_file
            .parent()
            .unwrap_or(root_path)
            .join(required_layer);
    }

    let current_dir = current_file.parent().unwrap_or(root_path);
    let mut best = None;
    let mut best_score = -1isize;
    for candidate in candidates {
        let candidate_dir = candidate.parent().unwrap_or(root_path);
        let score = path_common_prefix_len(current_dir, candidate_dir);
        let length = candidate.components().count() as isize;
        let combined = score * 1000 - length;
        if combined > best_score {
            best_score = combined;
            best = Some(candidate);
        }
    }
    best.unwrap_or_else(|| {
        current_file
            .parent()
            .unwrap_or(root_path)
            .join(required_layer)
    })
}

pub fn compute_move_metrics(
    placement: &FunctionPlacement,
) -> (usize, usize, usize, usize, Vec<PathBuf>, Vec<PathBuf>) {
    let incoming_calls = placement
        .call_analysis
        .calls_from_other_files
        .iter()
        .map(|(_, count)| *count)
        .sum::<usize>();
    let callers = placement.call_analysis.calls_from_other_files.len();
    let mut touched = BTreeSet::new();
    touched.insert(placement.current_file.clone());
    let mut outgoing_files = Vec::new();
    for (path, _) in &placement.call_analysis.inter_file_calls {
        touched.insert(path.clone());
        outgoing_files.push(path.clone());
    }
    let mut caller_files = Vec::new();
    for (path, _) in &placement.call_analysis.calls_from_other_files {
        touched.insert(path.clone());
        caller_files.push(path.clone());
    }
    let cost = touched.len().max(1);
    let benefit = 1 + callers;
    (incoming_calls, benefit, cost, callers, caller_files, outgoing_files)
}

pub fn collect_move_items(
    placements: &[FunctionPlacement],
    utility_names: &BTreeSet<String>,
    directory: &DirectoryAnalysis,
    root_path: &Path,
) -> Vec<PlanItem> {
    let mut items = Vec::new();
    for placement in placements {
        match &placement.placement_status {
            PlacementStatus::ShouldMove { reason, impact } => {
                let priority = if *impact >= 0.5 {
                    Priority::Critical
                } else if *impact >= 0.2 {
                    Priority::High
                } else if *impact >= 0.1 {
                    Priority::Medium
                } else {
                    Priority::Low
                };
                let (impact_weight, benefit, cost, callers, caller_files, outgoing_files) =
                    compute_move_metrics(placement);
                let to = placement
                    .suggested_file
                    .as_ref()
                    .map(|p| compress_path(p.to_string_lossy().as_ref()))
                    .unwrap_or_else(|| "-".to_string());
                items.push(PlanItem {
                    kind: ActionKind::Cohesion,
                    priority,
                    description: format!(
                        "`{}` from `{}` to `{}`: {} (impact {:.2})",
                        placement.name,
                        compress_path(placement.current_file.to_string_lossy().as_ref()),
                        to,
                        reason,
                        impact
                    ),
                    command: String::new(),
                    current_layer: None,
                    required_layer: None,
                    is_utility: utility_names.contains(&placement.name),
                    impact_weight,
                    benefit,
                    cost,
                    callers,
                    caller_files,
                    current_file: Some(placement.current_file.clone()),
                    target_file: placement.suggested_file.clone(),
                    outgoing_files,
                    name: Some(placement.name.clone()),
                    cluster_cohesion: 0.0,
                    member_count: 0,
                });
            }
            PlacementStatus::LayerViolation {
                current_layer,
                required_layer,
            } => {
                let target_path = resolve_required_layer_path(
                    required_layer,
                    &placement.current_file,
                    directory,
                    root_path,
                );
                let to = compress_path(target_path.to_string_lossy().as_ref());
                let (impact_weight, benefit, cost, callers, caller_files, outgoing_files) =
                    compute_move_metrics(placement);
                items.push(PlanItem {
                    kind: ActionKind::Structural,
                    priority: Priority::Critical,
                    description: format!(
                        "`{}` from `{}` to `{}`: layer violation {} -> {}",
                        placement.name,
                        compress_path(placement.current_file.to_string_lossy().as_ref()),
                        to,
                        current_layer,
                        required_layer
                    ),
                    command: String::new(),
                    current_layer: Some(current_layer.clone()),
                    required_layer: Some(required_layer.clone()),
                    is_utility: utility_names.contains(&placement.name),
                    impact_weight,
                    benefit,
                    cost,
                    callers,
                    caller_files,
                    current_file: Some(placement.current_file.clone()),
                    target_file: Some(target_path),
                    outgoing_files,
                    name: Some(placement.name.clone()),
                    cluster_cohesion: 0.0,
                    member_count: 0,
                });
            }
            _ => {}
        }
    }
    items
}

pub fn write_structural_batches(content: &mut String, items: &[PlanItem]) {
    if items.is_empty() {
        return;
    }

    let mut ordered_targets = Vec::new();
    let mut batches: HashMap<PathBuf, Vec<&PlanItem>> = HashMap::new();
    for item in items {
        let Some(target) = &item.target_file else {
            continue;
        };
        let entry = batches.entry(target.clone()).or_default();
        if entry.is_empty() {
            ordered_targets.push(target.clone());
        }
        entry.push(item);
    }

    content.push_str("### Phase 3 Batches\n\n");
    content.push_str("Action: execute batches in order and verify after each batch.\n");
    content.push_str("Note: each batch targets one destination module.\n\n");
    for (idx, target) in ordered_targets.iter().enumerate() {
        let empty: Vec<&PlanItem> = Vec::new();
        let items = batches.get(target).unwrap_or(&empty);
        content.push_str(&format!(
            "#### Batch {}: target `{}`\n\n",
            idx + 1,
            compress_path(target.to_string_lossy().as_ref())
        ));
        content.push_str("Action: move the listed functions into the target module.\n");
        content.push_str("Note: use the rg commands to locate definitions and callers.\n\n");
        let mut commands: Vec<String> = Vec::new();
        if !target.exists() {
            let target_label = compress_path(target.to_string_lossy().as_ref());
            content.push_str(&format!(
                "- Create target file: `{}`\n",
                target_label
            ));
            commands.push(format!("touch \"{}\"", target.to_string_lossy()));
        }
        for item in items {
            let name = item.name.as_deref().unwrap_or("function");
            let current = item
                .current_file
                .as_ref()
                .map(|p| compress_path(p.to_string_lossy().as_ref()))
                .unwrap_or_else(|| "-".to_string());
            let ratio = if item.cost == 0 {
                0.0
            } else {
                item.benefit as f64 / item.cost as f64
            };
            let caller_hint = if item.callers == 0 {
                "no external callers".to_string()
            } else {
                format!("update {} caller files", item.callers)
            };
            content.push_str(&format!(
                "- Move `{}` from `{}` (impact {}, benefit/cost {:.2}, touches {} files; {})\n",
                name,
                current,
                item.impact_weight,
                ratio,
                item.cost,
                caller_hint
            ));
            if let Some(current_file) = &item.current_file {
                commands.push(format!(
                    "rg -n \"{}\" \"{}\"",
                    name,
                    current_file.to_string_lossy()
                ));
            }
            let mut callers = item.caller_files.clone();
            callers.sort();
            callers.dedup();
            if !callers.is_empty() {
                content.push_str("- Update imports in:\n");
                for caller in callers {
                    content.push_str(&format!(
                        "  - `{}`\n",
                        compress_path(caller.to_string_lossy().as_ref())
                    ));
                    commands.push(format!(
                        "rg -n \"{}\" \"{}\"",
                        name,
                        caller.to_string_lossy()
                    ));
                }
            }
        }
        content.push_str("- Verification gate: `cargo test`\n");
        if !commands.is_empty() {
            content.push_str("\n```bash\n");
            for command in commands {
                content.push_str(&format!("{}\n", command));
            }
            content.push_str("```\n");
        }
        content.push('\n');
    }
}

pub fn write_cluster_batches(content: &mut String, plans: &[ClusterPlan], root_path: &Path) {
    if plans.is_empty() {
        return;
    }
    content.push_str("### Phase 2 Batches\n\n");
    content.push_str("Action: execute batches in order and verify after each batch.\n");
    content.push_str("Note: each batch creates or fills a cluster file.\n\n");
    for (idx, plan) in plans.iter().enumerate() {
        content.push_str(&format!(
            "#### Batch {}: target `{}`\n\n",
            idx + 1,
            compress_path(plan.target.to_string_lossy().as_ref())
        ));
        content.push_str("Action: move the listed functions into the target module.\n");
        content.push_str("Note: use the rg commands to locate definitions and callers.\n\n");
        let mut commands = Vec::new();
        if !plan.target.exists() {
            content.push_str(&format!(
                "- Create target file: `{}`\n",
                compress_path(plan.target.to_string_lossy().as_ref())
            ));
            commands.push(format!("touch \"{}\"", plan.target.to_string_lossy()));
        }
        content.push_str(&format!(
            "- Cluster cohesion {:.2}, {} functions\n",
            plan.cohesion,
            plan.members.len()
        ));
        for member in &plan.members {
            let file = compress_path(member.file.to_string_lossy().as_ref());
            content.push_str(&format!("- Move `{}` from `{}`\n", member.name, file));
            commands.push(format!(
                "rg -n \"{}\" \"{}\"",
                member.name,
                member.file.to_string_lossy()
            ));
            commands.push(format!(
                "rg -n \"{}\" \"{}\"",
                member.name,
                root_path.to_string_lossy()
            ));
        }
        content.push_str("- Verification gate: `cargo test`\n");
        if !commands.is_empty() {
            content.push_str("\n```bash\n");
            for command in commands {
                content.push_str(&format!("{}\n", command));
            }
            content.push_str("```\n");
        }
        content.push('\n');
    }
}
