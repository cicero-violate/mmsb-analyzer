//! Markdown report generation

use crate::utilities::compress_path;
use crate::control_flow::ControlFlowAnalyzer;
use crate::dependency::LayerGraph;
use crate::file_ordering::{analyze_file_ordering, build_file_dependency_graph};
use crate::types::*;
use anyhow::Result;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
fn display_path(path: &Path, root_path: &Path) -> String {
    let relative = path.strip_prefix(root_path).unwrap_or(path);
    relative.to_string_lossy().to_string()
}

fn placement_status_label(status: &PlacementStatus) -> String {
    match status {
        PlacementStatus::Correct => "ok".to_string(),
        PlacementStatus::ShouldMove { .. } => "move".to_string(),
        PlacementStatus::Orphaned { .. } => "orphaned".to_string(),
        PlacementStatus::LayerViolation { .. } => "layer violation".to_string(),
    }
}

fn placement_status_notes(status: &PlacementStatus) -> String {
    match status {
        PlacementStatus::Correct => String::new(),
        PlacementStatus::ShouldMove { reason, impact } => {
            format!("{} (impact {:.2})", reason, impact)
        }
        PlacementStatus::Orphaned { suggested_module } => {
            format!("suggest module {}", suggested_module)
        }
        PlacementStatus::LayerViolation {
            current_layer,
            required_layer,
        } => format!("{} -> {}", current_layer, required_layer),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ActionKind {
    Correctness,
    Structural,
    Cohesion,
    Ordering,
}

impl ActionKind {
    fn order(self) -> usize {
        match self {
            ActionKind::Correctness => 0,
            ActionKind::Structural => 1,
            ActionKind::Cohesion => 2,
            ActionKind::Ordering => 3,
        }
    }
}

#[derive(Clone)]
struct PlanItem {
    kind: ActionKind,
    priority: Priority,
    description: String,
    command: String,
}

fn collect_rename_items(ordering: &FileOrderingResult, label: &str) -> Vec<PlanItem> {
    let mut layer_violation_files = BTreeSet::new();
    for violation in &ordering.layer_violations {
        layer_violation_files.insert(violation.to.clone());
    }

    ordering
        .ordered_files
        .iter()
        .filter(|entry| entry.needs_rename)
        .map(|entry| {
            let from = entry.current_path.clone();
            let to = entry
                .current_path
                .parent()
                .map(|p| p.join(&entry.suggested_name))
                .unwrap_or_else(|| PathBuf::from(&entry.suggested_name));
            let priority = if layer_violation_files.contains(&entry.current_path) {
                Priority::Critical
            } else {
                Priority::Medium
            };
            PlanItem {
                kind: ActionKind::Ordering,
                priority,
                description: format!(
                    "[{}] `{}` -> `{}`",
                    label,
                    compress_path(from.to_string_lossy().as_ref()),
                    compress_path(to.to_string_lossy().as_ref())
                ),
                command: format!(
                    "git mv \"{}\" \"{}\"",
                    from.to_string_lossy(),
                    to.to_string_lossy()
                ),
            }
        })
        .collect()
}

fn collect_move_items(placements: &[FunctionPlacement]) -> Vec<PlanItem> {
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
                });
            }
            PlacementStatus::LayerViolation {
                current_layer,
                required_layer,
            } => {
                let to = placement
                    .suggested_file
                    .as_ref()
                    .map(|p| compress_path(p.to_string_lossy().as_ref()))
                    .unwrap_or_else(|| "-".to_string());
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
                });
            }
            _ => {}
        }
    }
    items
}

fn collect_utility_candidates(placements: &[FunctionPlacement]) -> Vec<String> {
    let mut candidates = BTreeSet::new();
    for placement in placements {
        let external_files = placement.call_analysis.calls_from_other_files.len();
        if external_files >= 3 {
            candidates.insert(format!(
                "`{}` called by {} files (suggest `utilities`)",
                placement.name, external_files
            ));
        }
    }
    candidates.into_iter().collect()
}

fn collect_directory_moves(
    ordering: &FileOrderingResult,
    label: &str,
    root_path: &Path,
) -> Vec<PlanItem> {
    let mut moves = Vec::new();
    let mut by_parent: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    let src_dir = root_path.join("src");

    for dir in &ordering.ordered_directories {
        if dir == root_path {
            continue;
        }
        if dir == &src_dir {
            continue;
        }
        if let Some(parent) = dir.parent() {
            by_parent
                .entry(parent.to_path_buf())
                .or_default()
                .push(dir.clone());
        }
    }

    for (parent, mut dirs) in by_parent {
        dirs.sort_by(|a, b| compare_dir_layers(a, b));
        for (idx, dir) in dirs.iter().enumerate() {
            let Some(name) = dir.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let clean = strip_numeric_prefix(name);
            let suggested = format!("{:03}_{}", idx * 10, clean);
            if name == suggested {
                continue;
            }
            let to = parent.join(&suggested);
            moves.push(PlanItem {
                kind: ActionKind::Ordering,
                priority: Priority::Medium,
                description: format!(
                    "[{}] dir `{}` -> `{}`",
                    label,
                    compress_path(dir.to_string_lossy().as_ref()),
                    compress_path(to.to_string_lossy().as_ref())
                ),
                command: format!(
                    "git mv \"{}\" \"{}\"",
                    dir.to_string_lossy(),
                    to.to_string_lossy()
                ),
            });
        }
    }

    moves
}

fn write_priority_section(content: &mut String, title: &str, items: &[PlanItem]) {
    content.push_str(&format!("## {}\n\n", title));
    if items.is_empty() {
        content.push_str("- None.\n\n");
        return;
    }

    let mut commands = Vec::new();
    for item in items {
        content.push_str(&format!("- {}\n", item.description));
        if !item.command.is_empty() {
            commands.push(item.command.clone());
        }
    }
    content.push('\n');

    if !commands.is_empty() {
        content.push_str("```bash\n");
        for cmd in commands {
            content.push_str(&format!("{}\n", cmd));
        }
        content.push_str("```\n\n");
    }
}

fn sort_plan_items(items: &mut Vec<PlanItem>) {
    items.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then_with(|| a.description.cmp(&b.description))
    });
}

fn load_cargo_warnings(output_dir: &str) -> Option<String> {
    let path = Path::new(output_dir).join("cargo_warnings.txt");
    if !path.exists() {
        return None;
    }
    fs::read_to_string(path).ok()
}

#[derive(Clone, Debug)]
struct ReportConfig {
    file_line_warning: usize,
    dir_file_warning: usize,
    naming_score_warning: f64,
    baseline_path: String,
}

impl ReportConfig {
    fn defaults() -> Self {
        Self {
            file_line_warning: 800,
            dir_file_warning: 30,
            naming_score_warning: 70.0,
            baseline_path: "metrics_baseline.txt".to_string(),
        }
    }
}

fn load_report_config(output_dir: &str) -> ReportConfig {
    let path = Path::new(output_dir).join("analyzer_config.toml");
    let mut config = ReportConfig::defaults();
    let Ok(contents) = fs::read_to_string(path) else {
        return config;
    };
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim().trim_matches('"');
        match key {
            "file_line_warning" => {
                if let Ok(parsed) = value.parse::<usize>() {
                    config.file_line_warning = parsed;
                }
            }
            "dir_file_warning" => {
                if let Ok(parsed) = value.parse::<usize>() {
                    config.dir_file_warning = parsed;
                }
            }
            "baseline_path" => {
                if !value.is_empty() {
                    config.baseline_path = value.to_string();
                }
            }
            "naming_score_warning" => {
                if let Ok(parsed) = value.parse::<f64>() {
                    config.naming_score_warning = parsed;
                }
            }
            _ => {}
        }
    }
    config
}

fn collect_size_warnings(
    directory: &DirectoryAnalysis,
    config: &ReportConfig,
    warnings: &mut Vec<String>,
) {
    if directory.files.len() >= config.dir_file_warning {
        warnings.push(format!(
            "Directory `{}` has {} files; consider splitting into submodules.",
            compress_path(directory.path.to_string_lossy().as_ref()),
            directory.files.len()
        ));
    }

    for file in &directory.files {
        if let Ok(contents) = fs::read_to_string(file) {
            let lines = contents.lines().count();
            if lines >= config.file_line_warning {
                warnings.push(format!(
                    "File `{}` has {} lines; consider extracting helpers.",
                    compress_path(file.to_string_lossy().as_ref()),
                    lines
                ));
            }
        }
    }

    for child in &directory.subdirectories {
        collect_size_warnings(child, config, warnings);
    }
}

fn naming_score_for_file(file: &Path, order_entry: Option<&FileOrderEntry>) -> Option<f64> {
    let name = file.file_name()?.to_string_lossy();
    let stem = file.file_stem()?.to_string_lossy();
    let mut score = 1.0f64;

    if stem.len() < 3 {
        score -= 0.2;
    }
    if stem.len() > 40 {
        score -= 0.1;
    }
    if stem.chars().any(|c| c.is_uppercase()) {
        score -= 0.1;
    }
    if !stem
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        score -= 0.1;
    }
    if name.contains("__") {
        score -= 0.1;
    }

    if let Some(entry) = order_entry {
        let expected = entry.suggested_name.as_str();
        let actual = name.as_ref();
        if expected != actual {
            score -= 0.3;
        } else {
            score += 0.1;
        }
    }

    if let Ok(contents) = fs::read_to_string(file) {
        let mut ident_counts: HashMap<String, usize> = HashMap::new();
        let ident_re = match Regex::new(r"[A-Za-z_][A-Za-z0-9_]*") {
            Ok(regex) => regex,
            Err(_) => return None,
        };
        for cap in ident_re.captures_iter(&contents) {
            let Some(m) = cap.get(0) else {
                continue;
            };
            let ident = m.as_str().to_lowercase();
            if matches!(
                ident.as_str(),
                "fn"
                    | "pub"
                    | "use"
                    | "struct"
                    | "enum"
                    | "impl"
                    | "mod"
                    | "let"
                    | "mut"
                    | "ref"
                    | "self"
                    | "crate"
                    | "super"
                    | "where"
                    | "trait"
                    | "type"
                    | "const"
                    | "static"
                    | "match"
                    | "if"
                    | "else"
                    | "for"
                    | "while"
                    | "loop"
                    | "return"
                    | "async"
                    | "await"
                    | "move"
                    | "dyn"
                    | "as"
            ) {
                continue;
            }
            *ident_counts.entry(ident).or_insert(0) += 1;
        }

        let mut idents = ident_counts.into_iter().collect::<Vec<_>>();
        idents.sort_by(|a, b| b.1.cmp(&a.1));
        let top_idents = idents.into_iter().take(8).map(|(k, _)| k).collect::<Vec<_>>();
        let name_tokens = stem
            .split('_')
            .map(|s| s.to_lowercase())
            .filter(|s| !s.is_empty() && !s.chars().all(|c| c.is_ascii_digit()))
            .collect::<Vec<_>>();
        let overlap = top_idents
            .iter()
            .filter(|ident| name_tokens.iter().any(|t| t == *ident))
            .count();

        if overlap == 0 {
            score -= 0.1;
        } else if overlap >= 2 {
            score += 0.1;
        }
    }

    if score < 0.0 {
        score = 0.0;
    }
    if score > 1.0 {
        score = 1.0;
    }
    Some(score * 100.0)
}

fn collect_naming_warnings(
    directory: &DirectoryAnalysis,
    config: &ReportConfig,
    warnings: &mut Vec<String>,
) -> Result<()> {
    let file_map = build_directory_entry_map(&directory.files)?;
    for file in &directory.files {
        let entry = file_map.get(file);
        if let Some(score) = naming_score_for_file(file, entry) {
            if score < config.naming_score_warning {
                let suggested = entry
                    .map(|e| e.suggested_name.as_str())
                    .unwrap_or("suggested name unavailable");
                warnings.push(format!(
                    "File `{}` has naming score {:.0}; consider renaming to `{}`.",
                    compress_path(file.to_string_lossy().as_ref()),
                    score,
                    suggested,
                ));
            }
        }
    }
    for child in &directory.subdirectories {
        collect_naming_warnings(child, config, warnings)?;
    }
    Ok(())
}

fn load_baseline_metrics(config: &ReportConfig, output_dir: &str) -> Option<HashMap<String, f64>> {
    let path = Path::new(output_dir).join(&config.baseline_path);
    let Ok(contents) = fs::read_to_string(path) else {
        return None;
    };
    let mut metrics = HashMap::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        let key = key.trim().to_string();
        let value = value.trim();
        if let Ok(parsed) = value.parse::<f64>() {
            metrics.insert(key, parsed);
        }
    }
    Some(metrics)
}

fn baseline_deltas(
    baseline: &HashMap<String, f64>,
    dir_cohesion: f64,
    ordering_correctness: f64,
    avg_cohesion: f64,
    renames_len: usize,
    relocations: usize,
) -> Vec<String> {
    let mut deltas = Vec::new();
    if let Some(prev) = baseline.get("directory_cohesion") {
        deltas.push(format!(
            "directory_cohesion: {:.2} -> {:.2} (delta {:+.2})",
            prev,
            dir_cohesion,
            dir_cohesion - prev
        ));
    }
    if let Some(prev) = baseline.get("ordering_correctness") {
        let current = ordering_correctness * 100.0;
        deltas.push(format!(
            "ordering_correctness: {:.1}% -> {:.1}% (delta {:+.1}%)",
            prev,
            current,
            current - prev
        ));
    }
    if let Some(prev) = baseline.get("avg_function_cohesion") {
        deltas.push(format!(
            "avg_function_cohesion: {:.2} -> {:.2} (delta {:+.2})",
            prev,
            avg_cohesion,
            avg_cohesion - prev
        ));
    }
    if let Some(prev) = baseline.get("rename_ops_needed") {
        let current = renames_len as f64;
        deltas.push(format!(
            "rename_ops_needed: {:.0} -> {} (delta {:+.0})",
            prev,
            renames_len,
            current - prev
        ));
    }
    if let Some(prev) = baseline.get("function_relocations") {
        let current = relocations as f64;
        deltas.push(format!(
            "function_relocations: {:.0} -> {} (delta {:+.0})",
            prev,
            relocations,
            current - prev
        ));
    }
    deltas
}

fn write_baseline_metrics(
    config: &ReportConfig,
    output_dir: &str,
    dir_cohesion: f64,
    ordering_correctness: f64,
    avg_cohesion: f64,
    renames_len: usize,
    relocations: usize,
) {
    let path = Path::new(output_dir).join(&config.baseline_path);
    if path.exists() {
        return;
    }
    let content = format!(
        "directory_cohesion={:.2}\nordering_correctness={:.1}\navg_function_cohesion={:.2}\nrename_ops_needed={}\nfunction_relocations={}\n",
        dir_cohesion,
        ordering_correctness * 100.0,
        avg_cohesion,
        renames_len,
        relocations
    );
    let _ = fs::write(path, content);
}

fn strip_numeric_prefix(name: &str) -> &str {
    let mut digits = 0usize;
    for ch in name.chars() {
        if ch.is_ascii_digit() {
            digits += 1;
        } else {
            break;
        }
    }
    if digits > 0 && name.chars().nth(digits) == Some('_') {
        &name[digits + 1..]
    } else {
        name
    }
}

fn compare_dir_layers(a: &Path, b: &Path) -> Ordering {
    let a_name = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let b_name = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let a_layer = layer_prefix_value(a_name).unwrap_or(i32::MAX);
    let b_layer = layer_prefix_value(b_name).unwrap_or(i32::MAX);
    a_layer.cmp(&b_layer).then_with(|| a_name.cmp(b_name))
}

fn layer_prefix_value(name: &str) -> Option<i32> {
    let mut digits = String::new();
    for ch in name.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else {
            break;
        }
    }
    if digits.is_empty() {
        None
    } else {
        digits.parse::<i32>().ok()
    }
}

fn collect_directories<'a>(node: &'a DirectoryAnalysis, acc: &mut Vec<&'a DirectoryAnalysis>) {
    acc.push(node);
    for child in &node.subdirectories {
        collect_directories(child, acc);
    }
}

fn build_directory_entry_map(files: &[PathBuf]) -> Result<HashMap<PathBuf, FileOrderEntry>> {
    if files.is_empty() {
        return Ok(HashMap::new());
    }
    let ordering = analyze_file_ordering(files, None)?;
    let mut map = HashMap::new();
    for entry in ordering.ordered_files {
        map.insert(entry.current_path.clone(), entry);
    }
    Ok(map)
}

fn slugify_path(path: &Path) -> String {
    let mut slug = String::new();
    for component in path.components() {
        if !slug.is_empty() {
            slug.push_str("__");
        }
        slug.push_str(&component.as_os_str().to_string_lossy().replace('/', "_"));
    }
    if slug.is_empty() {
        "root".to_string()
    } else {
        slug
    }
}

fn render_mermaid_graph(graph: &petgraph::graph::DiGraph<PathBuf, ()>) -> String {
    let mut output = String::from("```mermaid\ngraph TD\n");
    let mut node_ids: HashMap<usize, String> = HashMap::new();
    let mut idx = 0usize;
    for node in graph.node_indices() {
        let node_name = graph[node]
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");
        let safe_id = format!("F{}", idx);
        idx += 1;
        node_ids.insert(node.index(), safe_id.clone());
        output.push_str(&format!("    {}[\"{}\"]\n", safe_id, node_name));
    }
    for edge in graph.edge_indices() {
        if let Some((src, dst)) = graph.edge_endpoints(edge) {
            if let (Some(from), Some(to)) = (node_ids.get(&src.index()), node_ids.get(&dst.index()))
            {
                output.push_str(&format!("    {} --> {}\n", from, to));
            }
        }
    }
    output.push_str("```\n");
    output
}

fn compute_ordering_correctness(
    rust_ordering: &FileOrderingResult,
    julia_ordering: &FileOrderingResult,
) -> f64 {
    let mut total = 0usize;
    let mut correct = 0usize;
    for ordering in [rust_ordering, julia_ordering] {
        total += ordering.ordered_files.len();
        correct += ordering.ordered_files.len().saturating_sub(ordering.violations.len());
    }
    if total == 0 {
        1.0
    } else {
        correct as f64 / total as f64
    }
}

fn compute_directory_cohesion(placements: &[FunctionPlacement]) -> f64 {
    let mut intra = 0usize;
    let mut inter = 0usize;
    for placement in placements {
        let current_dir = placement.current_file.parent().map(|p| p.to_path_buf());
        intra += placement.call_analysis.intra_file_calls;
        for (file, count) in &placement.call_analysis.inter_file_calls {
            let same_dir = current_dir
                .as_ref()
                .and_then(|dir| file.parent().map(|p| p == dir))
                .unwrap_or(false);
            if same_dir {
                intra += count;
            } else {
                inter += count;
            }
        }
    }
    let total = intra + inter;
    if total == 0 {
        1.0
    } else {
        intra as f64 / total as f64
    }
}

pub struct ReportGenerator {
    output_dir: String,
}

impl ReportGenerator {
    pub fn new(output_dir: String) -> Self {
        Self { output_dir }
    }

    pub fn generate_all(
        &self,
        result: &AnalysisResult,
        cf_analyzer: &ControlFlowAnalyzer,
        rust_layers: &LayerGraph,
        julia_layers: &LayerGraph,
        rust_ordering: &FileOrderingResult,
        julia_ordering: &FileOrderingResult,
        function_placements: &[FunctionPlacement],
        function_clusters: &[FunctionCluster],
        directory_structure: &DirectoryAnalysis,
        root_path: &Path,
    ) -> Result<()> {
        fs::create_dir_all(&self.output_dir)?;
        self.cleanup_legacy_reports()?;

        println!("  Report: structure");
        self.generate_structure_report(result)?;
        println!("  Report: call_graph");
        self.generate_call_graph_report(cf_analyzer)?;
        println!("  Report: cfg");
        self.generate_cfg_report(cf_analyzer)?;
        println!("  Report: module_dependencies");
        self.generate_module_dependencies(result)?;
        println!("  Report: function_analysis");
        self.generate_function_analysis(result)?;
        println!("  Report: layer_dependencies");
        self.generate_layer_dependency_report(rust_layers, julia_layers, root_path)?;
        println!("  Report: file_ordering");
        self.generate_file_ordering_report(rust_ordering, julia_ordering, root_path)?;
        println!("  Report: cohesion_analysis");
        self.generate_cohesion_report(function_placements, function_clusters)?;
        println!("  Report: refactoring_plan");
        self.generate_refactoring_plan(
            rust_ordering,
            julia_ordering,
            function_placements,
            function_clusters,
            directory_structure,
            root_path,
        )?;
        println!("  Report: file_organization");
        self.generate_file_organization_report(
            directory_structure,
            rust_ordering,
            julia_ordering,
            root_path,
        )?;

        Ok(())
    }

    fn cleanup_legacy_reports(&self) -> Result<()> {
        let legacy_files = [
            "structure.md",
            "call_graph.md",
            "cfg.md",
            "module_dependencies.md",
            "function_analysis.md",
            "layer_dependencies.md",
            "file_ordering.md",
            "cohesion_analysis.md",
            "refactoring_plan.md",
            "file_organization.md",
        ];
        for file in legacy_files {
            let path = Path::new(&self.output_dir).join(file);
            if path.exists() {
                fs::remove_file(path)?;
            }
        }
        let report_dirs = [
            "structure",
            "call_graph",
            "cfg",
            "module_dependencies",
            "function_analysis",
            "layer_dependencies",
            "file_ordering",
            "cohesion_analysis",
            "refactoring_plan",
            "file_organization",
        ];
        for dir in report_dirs {
            let path = Path::new(&self.output_dir).join(dir);
            if !path.exists() {
                continue;
            }
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if dir == "cfg" && entry_path.file_name().map_or(false, |n| n == "dots") {
                        continue;
                    }
                    fs::remove_dir_all(entry_path)?;
                } else {
                    fs::remove_file(entry_path)?;
                }
            }
        }
        Ok(())
    }

    fn prepare_report_dir(&self, name: &str) -> Result<PathBuf> {
        let dir = Path::new(&self.output_dir).join(name);
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    fn generate_structure_report(&self, result: &AnalysisResult) -> Result<()> {
        let dir = self.prepare_report_dir("structure")?;
        let generated_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

        let mut files: BTreeMap<String, Vec<&CodeElement>> = BTreeMap::new();
        for elem in &result.elements {
            files
                .entry(elem.file_path.clone())
                .or_insert_with(Vec::new)
                .push(elem);
        }

        let mut grouped: BTreeMap<String, Vec<(String, Vec<&CodeElement>)>> = BTreeMap::new();
        for (file_path, elements) in files {
            let compressed = compress_path(&file_path);
            let key = prefix_key_from_path(&compressed);
            grouped
                .entry(key)
                .or_insert_with(Vec::new)
                .push((compressed, elements));
        }

        let mut grouped: Vec<_> = grouped.into_iter().collect();
        grouped.sort_by(|a, b| group_key_cmp(&a.0, &b.0));

        let mut index = String::from("# MMSB Code Structure Overview\n\n");
        index.push_str(&format!("Generated: {}\n\n", generated_at));
        index.push_str(
            "Each numbered file groups source files by MMSB prefix so a simple `ls structure/` \
shows the traversal order.\n\n",
        );

        if grouped.is_empty() {
            index.push_str("No code elements were recorded.\n");
        } else {
            index.push_str("## Group Files\n\n");
            for (idx, (group_key, _)) in grouped.iter().enumerate() {
                let slug = slugify_key(group_key);
                let file_name = format!("{:03}-{}.md", idx * 10, slug);
                index.push_str(&format!("- `{}` → `{}`\n", group_key, file_name));
            }
        }

        for (idx, (group_key, mut entries)) in grouped.into_iter().enumerate() {
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            let slug = slugify_key(&group_key);
            let file_name = format!("{:03}-{}.md", idx * 10, slug);
            let mut content = format!("# Structure Group: {}\n\n", group_key);

            for (file_path, mut elements) in entries {
                content.push_str(&format!("## File: {}\n\n", file_path));

                let layers: BTreeSet<String> = elements.iter().map(|e| e.layer.clone()).collect();
                let layer_summary = if layers.is_empty() {
                    "root".to_string()
                } else {
                    layers.iter().cloned().collect::<Vec<_>>().join(", ")
                };

                let mut language_counts: BTreeMap<String, usize> = BTreeMap::new();
                let mut type_counts: BTreeMap<String, usize> = BTreeMap::new();
                for elem in &elements {
                    *language_counts
                        .entry(language_label(&elem.language).to_string())
                        .or_insert(0) += 1;
                    *type_counts
                        .entry(format!("{:?}", elem.element_type))
                        .or_insert(0) += 1;
                }

                let lang_summary = if language_counts.is_empty() {
                    "n/a".to_string()
                } else {
                    language_counts
                        .iter()
                        .map(|(lang, count)| format!("{} ({})", lang, count))
                        .collect::<Vec<_>>()
                        .join(", ")
                };

                let type_summary = if type_counts.is_empty() {
                    "n/a".to_string()
                } else {
                    type_counts
                        .iter()
                        .map(|(ty, count)| format!("{} ({})", ty, count))
                        .collect::<Vec<_>>()
                        .join(", ")
                };

                content.push_str(&format!("- Layer(s): {}\n", layer_summary));
                content.push_str(&format!("- Language coverage: {}\n", lang_summary));
                content.push_str(&format!("- Element types: {}\n", type_summary));
                content.push_str(&format!("- Total elements: {}\n\n", elements.len()));

                content.push_str("### Elements\n\n");
                elements.sort_by(|a, b| {
                    a.line_number
                        .cmp(&b.line_number)
                        .then_with(|| a.name.cmp(&b.name))
                });
                for elem in elements {
                    content.push_str(&self.format_element_entry(elem));
                }
                content.push('\n');
            }

            fs::write(dir.join(file_name), content)?;
        }

        // Summary statistics
        index.push_str("\n## Summary Statistics\n\n");
        index.push_str(&format!("- Total elements: {}\n", result.elements.len()));
        index.push_str(&format!(
            "- Rust elements: {}\n",
            result
                .elements
                .iter()
                .filter(|e| matches!(e.language, Language::Rust))
                .count()
        ));
        index.push_str(&format!(
            "- Julia elements: {}\n",
            result
                .elements
                .iter()
                .filter(|e| matches!(e.language, Language::Julia))
                .count()
        ));

        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for elem in &result.elements {
            let key = format!("{:?}_{:?}", elem.language, elem.element_type);
            *type_counts.entry(key).or_insert(0) += 1;
        }

        index.push_str("\n### Elements by Type\n\n");
        let mut sorted_types: Vec<_> = type_counts.iter().collect();
        sorted_types.sort_by_key(|(k, _)| k.as_str());
        for (type_name, count) in sorted_types {
            index.push_str(&format!("- {}: {}\n", type_name, count));
        }

        fs::write(dir.join("index.md"), index)?;
        Ok(())
    }

    fn format_element_entry(&self, elem: &CodeElement) -> String {
        let mut entry = format!(
            "- [{} | {:?}] `{}` (line {}, {})\n",
            language_label(&elem.language),
            elem.element_type,
            elem.name,
            elem.line_number,
            visibility_label(&elem.visibility),
        );

        if !elem.signature.is_empty()
            && matches!(
                elem.element_type,
                ElementType::Function | ElementType::Struct
            )
        {
            entry.push_str(&format!(
                "  - Signature: `{}`\n",
                short_signature(&elem.signature)
            ));
        }

        if !elem.generic_params.is_empty() {
            entry.push_str(&format!(
                "  - Generics: {}\n",
                elem.generic_params.join(", ")
            ));
        }

        if matches!(elem.element_type, ElementType::Function) && !elem.calls.is_empty() {
            entry.push_str(&format!("  - Calls: {}\n", elem.calls.join(", ")));
        }

        entry
    }

    fn generate_call_graph_report(&self, cf_analyzer: &ControlFlowAnalyzer) -> Result<()> {
        let dir = self.prepare_report_dir("call_graph")?;
        let path = dir.join("index.md");
        let mut content = String::from("# Call Graph Analysis\n\n");
        content.push_str("This document shows the **interprocedural call graph** - which functions call which other functions.\n\n");
        content.push_str("> **Note:** This is NOT a control flow graph (CFG). CFG shows intraprocedural control flow (branches, loops) within individual functions.\n\n");

        let stats = cf_analyzer.get_statistics();

        content.push_str("## Call Graph Statistics\n\n");
        content.push_str(&format!("- Total functions: {}\n", stats.total_functions));
        content.push_str(&format!("- Total function calls: {}\n", stats.total_calls));
        content.push_str(&format!("- Maximum call depth: {}\n", stats.max_depth));
        content.push_str(&format!(
            "- Leaf functions (no outgoing calls): {}\n\n",
            stats.leaf_functions
        ));

        content.push_str("## Call Graph Visualization\n\n");
        content.push_str(&cf_analyzer.generate_mermaid());

        fs::write(path, content)?;
        Ok(())
    }

    fn generate_cfg_report(&self, cf_analyzer: &ControlFlowAnalyzer) -> Result<()> {
        let dir = self.prepare_report_dir("cfg")?;
        let mut index = String::from("# Control Flow Graphs (CFG)\n\n");
        index.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        ));

        if cf_analyzer.cfgs().is_empty() {
            index.push_str("No control flow graphs were captured.\n");
            fs::write(dir.join("index.md"), index)?;
            return Ok(());
        }

        let mut grouped: BTreeMap<String, Vec<(String, &FunctionCfg)>> = BTreeMap::new();
        for cfg in cf_analyzer.cfgs() {
            let compressed = compress_path(&cfg.file_path);
            let key = prefix_key_from_path(&compressed);
            grouped
                .entry(key)
                .or_insert_with(Vec::new)
                .push((compressed, cfg));
        }

        let mut grouped: Vec<_> = grouped.into_iter().collect();
        grouped.sort_by(|a, b| group_key_cmp(&a.0, &b.0));

        index.push_str(&format!("- Total CFGs: {}\n", cf_analyzer.cfgs().len()));
        index.push_str(
            "- Files are grouped by MMSB directory prefix; numeric prefixes match lexical ordering.\n\n",
        );

        index.push_str("## Group Files\n\n");
        for (idx, (group_key, _)) in grouped.iter().enumerate() {
            let file_name = format!("{:03}-{}.md", idx * 10, slugify_key(group_key));
            index.push_str(&format!("- `{}` → `{}`\n", group_key, file_name));
        }

        for (idx, (group_key, mut entries)) in grouped.into_iter().enumerate() {
            entries.sort_by(|a, b| a.1.function.cmp(&b.1.function));
            let slug = slugify_key(&group_key);
            let file_name = format!("{:03}-{}.md", idx * 10, slug);
            let mut content = format!("# CFG Group: {}\n\n", group_key);

            for (compressed, cfg) in entries {
                content.push_str(&format!("## Function: `{}`\n\n", cfg.function));
                content.push_str(&format!(
                    "- File: {}\n- Branches: {}\n- Loops: {}\n- Nodes: {}\n- Edges: {}\n\n",
                    compressed,
                    cfg.branch_count,
                    cfg.loop_count,
                    cfg.nodes.len(),
                    cfg.edges.len(),
                ));
                if let Some(dot_rel) = self.dot_path_for(&compressed) {
                    content.push_str(&format!("- DOT call graph: `{}`\n\n", dot_rel));
                }

                content.push_str("```mermaid\nflowchart TD\n");
                let mut id_map = HashMap::new();
                let prefix = sanitize_mermaid_id(&cfg.function);
                for node in &cfg.nodes {
                    let raw_id = format!("{}_{}", prefix, node.id);
                    let safe_id = sanitize_mermaid_id(&raw_id);
                    id_map.insert(node.id, safe_id.clone());
                    content.push_str(&format!(
                        "    {}[\"{}\"]\n",
                        safe_id,
                        sanitize_mermaid_label(&node.label)
                    ));
                }
                for edge in &cfg.edges {
                    if let (Some(src), Some(dst)) = (id_map.get(&edge.from), id_map.get(&edge.to)) {
                        content.push_str(&format!("    {} --> {}\n", src, dst));
                    }
                }
                content.push_str("```\n\n");
            }

            fs::write(dir.join(file_name), content)?;
        }

        fs::write(dir.join("index.md"), index)?;
        Ok(())
    }

    fn generate_layer_dependency_report(
        &self,
        rust_layers: &LayerGraph,
        julia_layers: &LayerGraph,
        root_path: &Path,
    ) -> Result<()> {
        let dir = self.prepare_report_dir("layer_dependencies")?;
        let path = dir.join("index.md");
        let mut content = String::from("# Layer Dependency Report\n\n");
        content.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        ));

        self.write_layer_section(&mut content, "Rust", rust_layers, root_path);
        self.write_layer_section(&mut content, "Julia", julia_layers, root_path);

        fs::write(path, content)?;
        Ok(())
    }

    fn generate_file_ordering_report(
        &self,
        rust_ordering: &FileOrderingResult,
        julia_ordering: &FileOrderingResult,
        root_path: &Path,
    ) -> Result<()> {
        let dir = self.prepare_report_dir("file_ordering")?;
        let path = dir.join("index.md");
        let mut content = String::from("# File Ordering Report\n\n");
        content.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        ));

        self.write_ordering_section(&mut content, "Rust", rust_ordering, root_path);
        self.write_ordering_section(&mut content, "Julia", julia_ordering, root_path);

        fs::write(path, content)?;
        Ok(())
    }

    fn write_ordering_section(
        &self,
        content: &mut String,
        label: &str,
        ordering: &FileOrderingResult,
        root_path: &Path,
    ) {
        content.push_str(&format!("## {} File Ordering\n\n", label));
        if ordering.ordered_files.is_empty() {
            content.push_str("No files analyzed.\n\n");
            return;
        }

        let rename_count = ordering
            .ordered_files
            .iter()
            .filter(|entry| entry.needs_rename)
            .count();
        content.push_str("### Metrics\n\n");
        content.push_str(&format!(
            "- Total files: {}\n- Rename suggestions: {}\n- Ordering violations: {}\n- Layer violations: {}\n- Directories: {}\n\n",
            ordering.ordered_files.len(),
            rename_count,
            ordering.violations.len(),
            ordering.layer_violations.len(),
            ordering.ordered_directories.len()
        ));

        if !ordering.cycles.is_empty() {
            content.push_str("### Cycles Detected\n");
            for cycle in &ordering.cycles {
                let listing = cycle
                    .iter()
                    .map(|p| compress_path(p.to_string_lossy().as_ref()))
                    .collect::<Vec<_>>()
                    .join(", ");
                content.push_str(&format!("- {}\n", listing));
            }
            content.push('\n');
        }

        content.push_str("### Canonical Order\n\n");
        content.push_str("| Order | Current | Suggested | Rename |\n");
        content.push_str("| --- | --- | --- | --- |\n");
        for entry in &ordering.ordered_files {
            let current = display_path(&entry.current_path, root_path);
            let rename = if entry.needs_rename { "yes" } else { "no" };
            content.push_str(&format!(
                "| {} | `{}` | `{}` | {} |\n",
                entry.canonical_order, current, entry.suggested_name, rename
            ));
        }
        content.push('\n');

        content.push_str("### Ordering Violations\n");
        if ordering.violations.is_empty() {
            content.push_str("- None detected.\n\n");
        } else {
            for violation in &ordering.violations {
                let file = display_path(&violation.file, root_path);
                content.push_str(&format!(
                    "- `{}`: alphabetical position {}, required position {}\n",
                    file, violation.current_position, violation.required_position
                ));
                if !violation.blocking_dependencies.is_empty() {
                    for dep in &violation.blocking_dependencies {
                        let dep_path = display_path(dep, root_path);
                        content.push_str(&format!("  - depends on `{}`\n", dep_path));
                    }
                }
            }
            content.push('\n');
        }

        content.push_str("### Layer Violations\n");
        if ordering.layer_violations.is_empty() {
            content.push_str("- None detected.\n\n");
        } else {
            for violation in &ordering.layer_violations {
                let from = display_path(&violation.from, root_path);
                let to = display_path(&violation.to, root_path);
                content.push_str(&format!(
                    "- `{}` ({}) depends on `{}` ({})\n",
                    to, violation.to_layer, from, violation.from_layer
                ));
            }
            content.push('\n');
        }

        content.push_str("### Directory Order\n");
        if ordering.ordered_directories.is_empty() {
            content.push_str("- None detected.\n\n");
        } else {
            for dir in &ordering.ordered_directories {
                let path = display_path(dir, root_path);
                content.push_str(&format!("- `{}`\n", path));
            }
            content.push('\n');
        }
    }

    fn generate_cohesion_report(
        &self,
        placements: &[FunctionPlacement],
        clusters: &[FunctionCluster],
    ) -> Result<()> {
        let dir = self.prepare_report_dir("cohesion_analysis")?;
        let path = dir.join("index.md");
        let mut content = String::from("# Function Cohesion Analysis\n\n");
        content.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        ));

        if placements.is_empty() {
            content.push_str("No function placement data recorded.\n");
            fs::write(path, content)?;
            return Ok(());
        }

        let mut by_file: BTreeMap<String, Vec<&FunctionPlacement>> = BTreeMap::new();
        let avg_cohesion = if placements.is_empty() {
            0.0
        } else {
            placements
                .iter()
                .map(|p| p.cohesion_score)
                .sum::<f64>()
                / placements.len() as f64
        };
        let move_count = placements
            .iter()
            .filter(|p| matches!(p.placement_status, PlacementStatus::ShouldMove { .. }))
            .count();
        let orphaned_count = placements
            .iter()
            .filter(|p| matches!(p.placement_status, PlacementStatus::Orphaned { .. }))
            .count();
        let layer_violation_count = placements
            .iter()
            .filter(|p| matches!(p.placement_status, PlacementStatus::LayerViolation { .. }))
            .count();
        content.push_str("## Metrics\n\n");
        content.push_str(&format!(
            "- Avg cohesion: {:.2}\n- Move suggestions: {}\n- Orphaned functions: {}\n- Layer violations: {}\n\n",
            avg_cohesion, move_count, orphaned_count, layer_violation_count
        ));
        for placement in placements {
            by_file
                .entry(placement.current_file.to_string_lossy().to_string())
                .or_default()
                .push(placement);
        }

        for (file, mut entries) in by_file {
            entries.sort_by(|a, b| {
                a.cohesion_score
                    .partial_cmp(&b.cohesion_score)
                    .unwrap_or(Ordering::Equal)
            });
            let compressed = compress_path(&file);
            content.push_str(&format!("## File: {}\n\n", compressed));
            content.push_str("| Function | Signature | Cohesion | Calls | Type refs | Status | Suggestion |\n");
            content.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
            for entry in entries {
                let status = placement_status_label(&entry.placement_status);
                let mut suggestion = entry
                    .suggested_file
                    .as_ref()
                    .map(|p| compress_path(p.to_string_lossy().as_ref()))
                    .unwrap_or_else(|| "-".to_string());
                let notes = placement_status_notes(&entry.placement_status);
                if !notes.is_empty() {
                    suggestion = format!("{} ({})", suggestion, notes);
                }
                let call_summary = format!(
                    "intra {}, inter {}",
                    entry.call_analysis.intra_file_calls,
                    entry.call_analysis.inter_file_calls.len()
                );
                let type_summary = format!(
                    "same {}, other {}",
                    entry.call_analysis.same_file_type_refs,
                    entry.call_analysis.other_file_type_refs
                );
                content.push_str(&format!(
                    "| `{}` | `{}` | {:.2} | {} | {} | {} | {} |\n",
                    entry.name,
                    entry.signature.replace('|', "\\|"),
                    entry.cohesion_score,
                    call_summary,
                    type_summary,
                    status,
                    suggestion
                ));
            }
            content.push('\n');
        }

        let orphaned: Vec<_> = placements
            .iter()
            .filter(|p| matches!(p.placement_status, PlacementStatus::Orphaned { .. }))
            .collect();
        content.push_str("## Orphaned Functions\n\n");
        if orphaned.is_empty() {
            content.push_str("- None detected.\n");
        } else {
            for entry in orphaned {
                let file = compress_path(entry.current_file.to_string_lossy().as_ref());
                content.push_str(&format!("- `{}` in `{}`\n", entry.name, file));
            }
        }
        content.push('\n');

        let utility_candidates = collect_utility_candidates(placements);
        content.push_str("## Utility Module Candidates\n\n");
        if utility_candidates.is_empty() {
            content.push_str("- None detected.\n\n");
        } else {
            for candidate in utility_candidates {
                content.push_str(&format!("- {}\n", candidate));
            }
            content.push('\n');
        }

        content.push_str("## Function Clusters\n\n");
        if clusters.is_empty() {
            content.push_str("- None detected.\n");
        } else {
            for cluster in clusters {
                let suggested = cluster
                    .suggested_file
                    .as_ref()
                    .map(|p| compress_path(p.to_string_lossy().as_ref()))
                    .unwrap_or_else(|| "-".to_string());
                let members = cluster
                    .members
                    .iter()
                    .map(|m| compress_path(m))
                    .collect::<Vec<_>>()
                    .join(", ");
                content.push_str(&format!(
                    "- cohesion {:.2}, suggested `{}`\n  - {}\n",
                    cluster.cohesion, suggested, members
                ));
            }
        }
        content.push('\n');

        fs::write(path, content)?;
        Ok(())
    }

    fn generate_refactoring_plan(
        &self,
        rust_ordering: &FileOrderingResult,
        julia_ordering: &FileOrderingResult,
        placements: &[FunctionPlacement],
        clusters: &[FunctionCluster],
        directory: &DirectoryAnalysis,
        root_path: &Path,
    ) -> Result<()> {
        let dir = self.prepare_report_dir("refactoring_plan")?;
        let path = dir.join("index.md");
        let mut content = String::from("# Refactoring Plan\n\n");
        content.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        ));

        let mut renames = collect_rename_items(rust_ordering, "Rust")
            .into_iter()
            .chain(collect_rename_items(julia_ordering, "Julia"))
            .collect::<Vec<_>>();
        renames.extend(collect_directory_moves(rust_ordering, "Rust", root_path));
        renames.extend(collect_directory_moves(julia_ordering, "Julia", root_path));
        let moves = collect_move_items(placements);
        let orphaned = placements
            .iter()
            .filter(|p| matches!(p.placement_status, PlacementStatus::Orphaned { .. }))
            .collect::<Vec<_>>();

        let mut all_items = Vec::new();
        all_items.extend(renames.iter().cloned());
        all_items.extend(moves.iter().cloned());

        let mut correctness = Vec::new();
        let mut structural = Vec::new();
        let mut cohesion = Vec::new();
        let mut ordering = Vec::new();

        for item in all_items {
            match item.kind {
                ActionKind::Correctness => correctness.push(item),
                ActionKind::Structural => structural.push(item),
                ActionKind::Cohesion => cohesion.push(item),
                ActionKind::Ordering => ordering.push(item),
            }
        }

        sort_plan_items(&mut correctness);
        sort_plan_items(&mut structural);
        sort_plan_items(&mut cohesion);
        sort_plan_items(&mut ordering);

        let config = load_report_config(&self.output_dir);
        let dir_cohesion = compute_directory_cohesion(placements);
        let avg_cohesion = if placements.is_empty() {
            0.0
        } else {
            placements
                .iter()
                .map(|p| p.cohesion_score)
                .sum::<f64>()
                / placements.len() as f64
        };
        let ordering_correctness = compute_ordering_correctness(rust_ordering, julia_ordering);
        let relocations = placements
            .iter()
            .filter(|p| matches!(p.placement_status, PlacementStatus::ShouldMove { .. }
                | PlacementStatus::LayerViolation { .. }))
            .count();

        write_baseline_metrics(
            &config,
            &self.output_dir,
            dir_cohesion,
            ordering_correctness,
            avg_cohesion,
            renames.len(),
            relocations,
        );

        content.push_str("## Summary\n\n");
        content.push_str(&format!(
            "- File/dir renames: {}\n- Function moves: {}\n- Orphaned functions: {}\n- Clusters: {}\n\n",
            renames.len(),
            moves.len(),
            orphaned.len(),
            clusters.len()
        ));
        content.push_str("## Metrics\n\n");
        content.push_str(&format!(
            "- Directory cohesion: {:.2}\n- Ordering correctness: {:.1}%\n- Avg function cohesion: {:.2}\n- Rename ops needed: {}\n- Function relocations suggested: {}\n\n",
            dir_cohesion,
            ordering_correctness * 100.0,
            avg_cohesion,
            renames.len(),
            relocations
        ));

        if let Some(baseline) = load_baseline_metrics(&config, &self.output_dir) {
            let mut regression_warnings = Vec::new();
            let epsilon = 0.005;
            if let Some(prev) = baseline.get("directory_cohesion") {
                if dir_cohesion + epsilon < *prev {
                    regression_warnings.push(format!(
                        "Directory cohesion dropped from {:.2} to {:.2}.",
                        prev, dir_cohesion
                    ));
                }
            }
            if let Some(prev) = baseline.get("ordering_correctness") {
                let current = ordering_correctness * 100.0;
                if current + epsilon < *prev {
                    regression_warnings.push(format!(
                        "Ordering correctness dropped from {:.1}% to {:.1}%.",
                        prev, current
                    ));
                }
            }
            if let Some(prev) = baseline.get("avg_function_cohesion") {
                if avg_cohesion + epsilon < *prev {
                    regression_warnings.push(format!(
                        "Avg function cohesion dropped from {:.2} to {:.2}.",
                        prev, avg_cohesion
                    ));
                }
            }
            if let Some(prev) = baseline.get("rename_ops_needed") {
                if (renames.len() as f64) > *prev + epsilon {
                    regression_warnings.push(format!(
                        "Rename ops needed increased from {:.0} to {}.",
                        prev,
                        renames.len()
                    ));
                }
            }
            if let Some(prev) = baseline.get("function_relocations") {
                if (relocations as f64) > *prev + epsilon {
                    regression_warnings.push(format!(
                        "Function relocations suggested increased from {:.0} to {}.",
                        prev,
                        relocations
                    ));
                }
            }

            let deltas = baseline_deltas(
                &baseline,
                dir_cohesion,
                ordering_correctness,
                avg_cohesion,
                renames.len(),
                relocations,
            );
            if !deltas.is_empty() {
                println!("Baseline deltas:");
                for line in &deltas {
                    println!("  {}", line);
                }
            }

            content.push_str("## Baseline Regression Warnings\n\n");
            if regression_warnings.is_empty() {
                content.push_str("- None.\n\n");
            } else {
                for warning in regression_warnings {
                    content.push_str(&format!("- {}\n", warning));
                }
                content.push('\n');
            }
        }

        write_priority_section(&mut content, "Phase 1: Correctness Blockers", &correctness);
        write_priority_section(&mut content, "Phase 2: Structural Constraints", &structural);
        write_priority_section(&mut content, "Phase 3: Cohesion Improvements", &cohesion);
        write_priority_section(&mut content, "Phase 4: Ordering & Renames", &ordering);

        content.push_str("## Orphaned Functions\n\n");
        if orphaned.is_empty() {
            content.push_str("- None detected.\n\n");
        } else {
            for entry in orphaned {
                let file = compress_path(entry.current_file.to_string_lossy().as_ref());
                content.push_str(&format!("- `{}` in `{}`\n", entry.name, file));
            }
            content.push('\n');
        }

        content.push_str("## Suggested New Files (Clusters)\n\n");
        if clusters.is_empty() {
            content.push_str("- None detected.\n\n");
        } else {
            for cluster in clusters {
                let suggested = cluster
                    .suggested_file
                    .as_ref()
                    .map(|p| compress_path(p.to_string_lossy().as_ref()))
                    .unwrap_or_else(|| "new module".to_string());
                let members = cluster
                    .members
                    .iter()
                    .map(|m| compress_path(m))
                    .collect::<Vec<_>>()
                    .join(", ");
                content.push_str(&format!(
                    "- cohesion {:.2}, suggested `{}`\n  - {}\n",
                    cluster.cohesion, suggested, members
                ));
            }
            content.push('\n');
        }

        let utility_candidates = collect_utility_candidates(placements);
        content.push_str("## Utility Module Candidates\n\n");
        if utility_candidates.is_empty() {
            content.push_str("- None detected.\n\n");
        } else {
            for candidate in utility_candidates {
                content.push_str(&format!("- {}\n", candidate));
            }
            content.push('\n');
        }

        let mut naming_warnings = Vec::new();
        let _ = collect_naming_warnings(directory, &config, &mut naming_warnings);
        content.push_str("## Naming Warnings\n\n");
        if naming_warnings.is_empty() {
            content.push_str("- None.\n\n");
        } else {
            for warning in naming_warnings {
                content.push_str(&format!("- {}\n", warning));
            }
            content.push('\n');
        }

        let mut size_warnings = Vec::new();
        collect_size_warnings(directory, &config, &mut size_warnings);
        content.push_str("## Size Warnings\n\n");
        if size_warnings.is_empty() {
            content.push_str("- None.\n\n");
        } else {
            for warning in size_warnings {
                content.push_str(&format!("- {}\n", warning));
            }
            content.push('\n');
        }

        if let Some(warnings) = load_cargo_warnings(&self.output_dir) {
            content.push_str("## Cargo Warnings\n\n");
            if warnings.trim().is_empty() {
                content.push_str("- None.\n\n");
            } else {
                content.push_str("```text\n");
                content.push_str(warnings.trim());
                content.push_str("\n```\n\n");
            }
        }

        fs::write(path, content)?;
        Ok(())
    }

    fn generate_file_organization_report(
        &self,
        directory: &DirectoryAnalysis,
        _rust_ordering: &FileOrderingResult,
        _julia_ordering: &FileOrderingResult,
        root_path: &Path,
    ) -> Result<()> {
        let dir = self.prepare_report_dir("file_organization")?;
        let mut index = String::from("# File Organization Report\n\n");
        index.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        ));

        let mut entries = Vec::new();
        collect_directories(directory, &mut entries);
        for entry in &entries {
            if entry.files.is_empty() {
                continue;
            }
            let file_map = build_directory_entry_map(&entry.files)?;
            let relative = entry
                .path
                .strip_prefix(root_path)
                .unwrap_or(&entry.path)
                .to_path_buf();
            let slug = slugify_path(&relative);
            let file_name = format!("{}.md", slug);
            index.push_str(&format!(
                "- `{}` → `{}`\n",
                compress_path(entry.path.to_string_lossy().as_ref()),
                file_name
            ));

            let mut content = format!(
                "# Directory: {}\n\n",
                compress_path(entry.path.to_string_lossy().as_ref())
            );
            content.push_str(&format!("- Layer: `{}`\n\n", entry.layer));
            content.push_str("## Files\n\n");
            content.push_str("| File | Suggested | Rename |\n");
            content.push_str("| --- | --- | --- |\n");
            let mut files = entry.files.clone();
            files.sort();
            for file in files {
                let entry_info = file_map.get(&file);
                let suggested = entry_info
                    .map(|info| info.suggested_name.as_str())
                    .unwrap_or("-");
                let rename = entry_info
                    .map(|info| if info.needs_rename { "yes" } else { "no" })
                    .unwrap_or("no");
                content.push_str(&format!(
                    "| `{}` | `{}` | {} |\n",
                    compress_path(file.to_string_lossy().as_ref()),
                    suggested,
                    rename
                ));
            }
            content.push('\n');

            content.push_str("## Dependency Graph\n\n");
            if entry.files.is_empty() {
                content.push_str("No source files.\n\n");
            } else {
                let graph = build_file_dependency_graph(&entry.files)?;
                content.push_str(&render_mermaid_graph(&graph));
                content.push('\n');
            }

            fs::write(dir.join(file_name), content)?;
        }

        fs::write(dir.join("index.md"), index)?;
        Ok(())
    }

    fn write_layer_section(
        &self,
        content: &mut String,
        label: &str,
        graph: &LayerGraph,
        root_path: &Path,
    ) {
        content.push_str(&format!("## {} Layer Graph\n\n", label));

        if graph.ordered_layers.is_empty() {
            content.push_str("No layers discovered.\n\n");
            return;
        }

        content.push_str("### Layer Order\n");
        for (idx, layer) in graph.ordered_layers.iter().enumerate() {
            let cycle_tag = if graph.cycles.contains(layer) {
                " (cycle)"
            } else {
                ""
            };
            content.push_str(&format!("{}. `{}`{}\n", idx + 1, layer, cycle_tag));
        }
        content.push('\n');

        if !graph.cycles.is_empty() {
            content.push_str("### Cycles Detected\n");
            for cycle in &graph.cycles {
                content.push_str(&format!("- `{}`\n", cycle));
            }
            content.push('\n');
        }

        let violations: Vec<_> = graph.edges.iter().filter(|e| e.violation).collect();
        content.push_str("### Layer Violations\n");
        if violations.is_empty() {
            content.push_str("- None detected.\n\n");
        } else {
            for edge in violations {
                content.push_str(&format!(
                    "- `{}` depends on `{}` ({} references)\n",
                    edge.to,
                    edge.from,
                    edge.references.len()
                ));
                for reference in &edge.references {
                    let compressed = display_path(&reference.file, root_path);
                    content.push_str(&format!("  - {} :: {}\n", compressed, reference.reference));
                }
            }
            content.push('\n');
        }

        content.push_str("### Dependency Edges\n");
        if graph.edges.is_empty() {
            content.push_str("- No cross-layer dependencies recorded.\n\n");
        } else {
            for edge in &graph.edges {
                content.push_str(&format!(
                    "- `{}` → `{}` ({} references{})\n",
                    edge.from,
                    edge.to,
                    edge.references.len(),
                    if edge.violation { ", VIOLATION" } else { "" }
                ));
                for reference in &edge.references {
                    let compressed = display_path(&reference.file, root_path);
                    content.push_str(&format!("  - {} :: {}\n", compressed, reference.reference));
                }
            }
            content.push('\n');
        }

        content.push_str("### Unresolved References\n");
        if graph.unresolved.is_empty() {
            content.push_str("- None.\n\n");
        } else {
            for unresolved in &graph.unresolved {
                let compressed = display_path(&unresolved.file, root_path);
                content.push_str(&format!("- {} → `{}`\n", compressed, unresolved.reference));
            }
            content.push('\n');
        }
    }

    fn generate_module_dependencies(&self, result: &AnalysisResult) -> Result<()> {
        let dir = self.prepare_report_dir("module_dependencies")?;
        let index_path = dir.join("index.md");
        let mut index = String::from("# Module Dependencies\n\n");

        if result.modules.is_empty() {
            index.push_str("No module metadata captured yet.\n");
            fs::write(index_path, index)?;
            return Ok(());
        }

        let mut modules_by_file: BTreeMap<String, ModuleAggregate> = BTreeMap::new();
        for module in &result.modules {
            let layer = self.extract_layer_from_path(&module.file_path);
            let entry = modules_by_file
                .entry(module.file_path.clone())
                .or_insert_with(|| ModuleAggregate::new(module.name.clone(), layer.clone()));

            if entry.name == "unknown" && !module.name.is_empty() {
                entry.name = module.name.clone();
            }

            entry.layer = layer;
            for import in &module.imports {
                entry.imports.insert(normalize_use_stmt(import));
            }
            for export in &module.exports {
                entry.exports.insert(normalize_use_stmt(export));
            }
            for sub in &module.submodules {
                entry.submodules.insert(sub.clone());
            }
        }

        let total_imports: usize = modules_by_file.values().map(|m| m.imports.len()).sum();
        let total_exports: usize = modules_by_file.values().map(|m| m.exports.len()).sum();
        let total_submodules: usize = modules_by_file.values().map(|m| m.submodules.len()).sum();

        let mut modules: Vec<_> = modules_by_file.into_iter().collect();
        modules.sort_by(|a, b| a.0.cmp(&b.0));

        index.push_str(&format!("- Module files analyzed: {}\n", modules.len()));
        index.push_str(&format!("- Unique imports captured: {}\n", total_imports));
        index.push_str(&format!("- Unique exports captured: {}\n", total_exports));
        index.push_str(&format!(
            "- Submodule declarations captured: {}\n\n",
            total_submodules
        ));
        index.push_str("## Per-file Summary\n\n");
        for (file_path, module) in &modules {
            let compressed = compress_path(file_path);
            index.push_str(&format!(
                "- `{}` → module `{}` (layer {}, {} imports / {} exports / {} submodules)\n",
                compressed,
                module.name,
                module.layer,
                module.imports.len(),
                module.exports.len(),
                module.submodules.len()
            ));
        }
        index.push_str(
            "\n## Detailed Files\n\n- `010-imports.md` – expanded import lists\n- `020-exports.md` – export statements\n- `030-submodules.md` – nested module declarations\n- `040-violations.md` – placeholder for future per-module violations\n",
        );
        fs::write(&index_path, index)?;

        let mut imports_doc = String::from("# Module Imports\n\n");
        let mut has_imports = false;
        for (file_path, module) in &modules {
            if module.imports.is_empty() {
                continue;
            }
            has_imports = true;
            let compressed = compress_path(file_path);
            imports_doc.push_str(&format!("## {} ({})\n\n", compressed, module.layer));
            imports_doc.push_str(&format!("Module `{}`\n\n", module.name));
            for import in &module.imports {
                imports_doc.push_str(&format!("- `{}`\n", import));
            }
            imports_doc.push('\n');
        }
        if !has_imports {
            imports_doc.push_str("No imports captured across modules.\n");
        }
        fs::write(dir.join("010-imports.md"), imports_doc)?;

        let mut exports_doc = String::from("# Module Exports\n\n");
        let mut has_exports = false;
        for (file_path, module) in &modules {
            if module.exports.is_empty() {
                continue;
            }
            has_exports = true;
            let compressed = compress_path(file_path);
            exports_doc.push_str(&format!("## {} ({})\n\n", compressed, module.layer));
            exports_doc.push_str(&format!("Module `{}`\n\n", module.name));
            for export in &module.exports {
                exports_doc.push_str(&format!("- `{}`\n", export));
            }
            exports_doc.push('\n');
        }
        if !has_exports {
            exports_doc.push_str("No exports captured across modules.\n");
        }
        fs::write(dir.join("020-exports.md"), exports_doc)?;

        let mut subs_doc = String::from("# Submodules\n\n");
        let mut has_submodules = false;
        for (file_path, module) in &modules {
            if module.submodules.is_empty() {
                continue;
            }
            has_submodules = true;
            let compressed = compress_path(file_path);
            subs_doc.push_str(&format!("## {} ({})\n\n", compressed, module.layer));
            subs_doc.push_str(&format!("Module `{}`\n\n", module.name));
            for sub in &module.submodules {
                subs_doc.push_str(&format!("- `{}`\n", sub));
            }
            subs_doc.push('\n');
        }
        if !has_submodules {
            subs_doc.push_str("No nested modules recorded.\n");
        }
        fs::write(dir.join("030-submodules.md"), subs_doc)?;

        let mut violations_doc = String::from("# Module Violations\n\n");
        violations_doc.push_str(
            "Per-module import/export violations are not computed yet.\n\
Refer to `layer_dependencies/index.md` for cross-layer problems.\n",
        );
        fs::write(dir.join("040-violations.md"), violations_doc)?;

        Ok(())
    }

    fn generate_function_analysis(&self, result: &AnalysisResult) -> Result<()> {
        let dir = self.prepare_report_dir("function_analysis")?;
        let mut index = String::from("# Function Analysis\n\n");

        let functions: Vec<_> = result
            .elements
            .iter()
            .filter(|e| matches!(e.element_type, ElementType::Function))
            .collect();

        index.push_str(&format!("## Total Functions: {}\n\n", functions.len()));
        index.push_str(
            "Functions are bucketed alphabetically so `ls function_analysis/` advertises the range.\n\n",
        );

        if functions.is_empty() {
            fs::write(dir.join("index.md"), index)?;
            return Ok(());
        }

        let bucket_labels = ["A-F", "G-M", "N-S", "T-Z", "Other"];
        let mut buckets: HashMap<&'static str, Vec<&CodeElement>> = HashMap::new();
        for label in bucket_labels {
            buckets.insert(label, Vec::new());
        }

        for func in &functions {
            let label = function_bucket_label(&func.name);
            buckets.entry(label).or_insert_with(Vec::new).push(func);
        }

        index.push_str("## Bucket Files\n\n");
        for (idx, label) in bucket_labels.iter().enumerate() {
            let file_name = format!("{:03}-functions_{}.md", (idx + 1) * 10, label);
            let count = buckets.get(label).map(|v| v.len()).unwrap_or(0);
            index.push_str(&format!(
                "- `{}` → `{}` ({} functions)\n",
                label, file_name, count
            ));
        }
        fs::write(dir.join("index.md"), index)?;

        for (idx, label) in bucket_labels.iter().enumerate() {
            let mut funcs = buckets.remove(label).unwrap_or_default();
            funcs.sort_by_key(|f| (&f.layer, &f.name));
            let file_name = format!("{:03}-functions_{}.md", (idx + 1) * 10, label);
            let mut content = format!("# Functions {}\n\n", label);

            if funcs.is_empty() {
                content.push_str("No functions fell into this range.\n");
                fs::write(dir.join(file_name), content)?;
                continue;
            }

            let mut layer_map: BTreeMap<String, Vec<&CodeElement>> = BTreeMap::new();
            for func in funcs {
                layer_map
                    .entry(func.layer.clone())
                    .or_insert_with(Vec::new)
                    .push(func);
            }

            for (layer, entries) in layer_map {
                content.push_str(&format!("## Layer: {}\n\n", layer));

                let mut rust_funcs: Vec<_> = entries
                    .iter()
                    .filter(|f| matches!(f.language, Language::Rust))
                    .collect();
                let mut julia_funcs: Vec<_> = entries
                    .iter()
                    .filter(|f| matches!(f.language, Language::Julia))
                    .collect();

                rust_funcs.sort_by_key(|f| &f.name);
                julia_funcs.sort_by_key(|f| &f.name);

                if !rust_funcs.is_empty() {
                    content.push_str("### Rust Functions\n\n");
                    for func in rust_funcs {
                        content.push_str(&format!("#### `{}`\n\n", func.name));
                        let compressed = compress_path(&func.file_path);
                        content.push_str(&format!(
                            "- **File:** {}:{}\n",
                            compressed, func.line_number
                        ));
                        content.push_str(&format!("- **Visibility:** {:?}\n", func.visibility));

                        if !func.generic_params.is_empty() {
                            content.push_str(&format!(
                                "- **Generics:** {}\n",
                                func.generic_params.join(", ")
                            ));
                        }

                        if !func.calls.is_empty() {
                            content.push_str("- **Calls:**\n");
                            for call in &func.calls {
                                content.push_str(&format!("  - `{}`\n", call));
                            }
                        }
                        content.push_str("\n");
                    }
                }

                if !julia_funcs.is_empty() {
                    content.push_str("### Julia Functions\n\n");
                    for func in julia_funcs {
                        content.push_str(&format!("#### `{}`\n\n", func.name));
                        let compressed = compress_path(&func.file_path);
                        content.push_str(&format!(
                            "- **File:** {}:{}\n",
                            compressed, func.line_number
                        ));
                        content.push_str(&format!("- **Signature:** `{}`\n", func.signature));

                        if !func.calls.is_empty() {
                            content.push_str("- **Calls:**\n");
                            for call in &func.calls {
                                content.push_str(&format!("  - `{}`\n", call));
                            }
                        }
                        content.push_str("\n");
                    }
                }
            }

            fs::write(dir.join(file_name), content)?;
        }

        Ok(())
    }

    fn extract_layer_from_path(&self, path: &str) -> String {
        for component in path.split('/') {
            if component
                .chars()
                .next()
                .map_or(false, |c| c.is_ascii_digit())
            {
                if let Some(pos) = component.find('_') {
                    if component[..pos].chars().all(|c| c.is_ascii_digit()) {
                        return component.to_string();
                    }
                }
            }
        }
        "root".to_string()
    }

    fn dot_path_for(&self, compressed_path: &str) -> Option<String> {
        let slug = slugify_file_path(compressed_path);
        let rel = format!("cfg/dots/{}/call_graph.dot", slug);
        let absolute = Path::new(&self.output_dir).join(&rel);
        if absolute.exists() {
            Some(rel)
        } else {
            None
        }
    }
}

fn prefix_key_from_path(path: &str) -> String {
    let relative = path.strip_prefix("MMSB/").unwrap_or(path);
    if relative.is_empty() {
        return "root".to_string();
    }
    let parts: Vec<&str> = relative.split('/').collect();
    if parts.len() == 1 {
        return "root".to_string();
    }
    if parts[0] == "src" && parts.len() >= 2 {
        return format!("{}/{}", parts[0], parts[1]);
    }
    parts[0].to_string()
}

fn slugify_key(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '/' => '-',
            ' ' => '_',
            _ if c.is_ascii_alphanumeric() || c == '-' => c.to_ascii_lowercase(),
            _ => '_',
        })
        .collect()
}

fn group_key_cmp(a: &str, b: &str) -> Ordering {
    match (a == "root", b == "root") {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => a.cmp(b),
    }
}

fn function_bucket_label(name: &str) -> &'static str {
    let first = name
        .chars()
        .find(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .unwrap_or('#');

    match first {
        'A'..='F' => "A-F",
        'G'..='M' => "G-M",
        'N'..='S' => "N-S",
        'T'..='Z' => "T-Z",
        _ => "Other",
    }
}

fn slugify_file_path(path: &str) -> String {
    path.trim_start_matches("MMSB/")
        .replace('/', "-")
        .replace('.', "_")
        .to_lowercase()
}

fn language_label(language: &Language) -> &'static str {
    match language {
        Language::Rust => "Rust",
        Language::Julia => "Julia",
    }
}

fn visibility_label(vis: &Visibility) -> &'static str {
    match vis {
        Visibility::Public => "pub",
        Visibility::Crate => "pub(crate)",
        Visibility::Private => "priv",
    }
}

fn short_signature(input: &str) -> String {
    let collapsed = input.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.len() > 120 {
        let mut truncated = collapsed.chars().take(117).collect::<String>();
        truncated.push_str("...");
        truncated
    } else {
        collapsed
    }
}

struct ModuleAggregate {
    name: String,
    layer: String,
    imports: BTreeSet<String>,
    exports: BTreeSet<String>,
    submodules: BTreeSet<String>,
}

impl ModuleAggregate {
    fn new(name: String, layer: String) -> Self {
        Self {
            name: if name.is_empty() {
                "unknown".to_string()
            } else {
                name
            },
            layer,
            imports: BTreeSet::new(),
            exports: BTreeSet::new(),
            submodules: BTreeSet::new(),
        }
    }
}

fn normalize_use_stmt(stmt: &str) -> String {
    let collapsed = stmt.replace('\n', " ");
    let mut cleaned = collapsed.split_whitespace().collect::<Vec<_>>().join(" ");
    if let Some(idx) = cleaned.find(';') {
        cleaned.truncate(idx);
    }
    cleaned = cleaned.trim().to_string();
    if cleaned.starts_with("pub") {
        if let Some(pos) = cleaned.find(' ') {
            cleaned = cleaned[pos + 1..].trim().to_string();
        }
    }
    if let Some(stripped) = cleaned.strip_prefix("use ") {
        cleaned = stripped.trim().to_string();
    }
    cleaned
}

fn sanitize_mermaid_id(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn sanitize_mermaid_label(label: &str) -> String {
    label.replace('"', "'").replace('`', "'")
}
