//! Cluster 001: Core dependency analysis and file ordering utilities
//!
//! This module contains fundamental functions for:
//! - Module mapping and dependency resolution
//! - Topological sorting and layer-constrained ordering
//! - File gathering and layer construction
//! - Dependency graph building and cycle detection
//! - DOT export for program CFGs
//! - Naming validation and warnings

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use petgraph::algo::tarjan_scc;
use petgraph::graph::{DiGraph, NodeIndex};
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use syn::visit::Visit;
use syn::{ItemUse, UseTree};

use crate::dependency::{LayerGraph, ReferenceDetail, UnresolvedDependency};

// ============================================================================
// From src/000_dependency.rs
// ============================================================================
pub fn build_directory_entry_map(
    files: &[PathBuf],
) -> Result<HashMap<PathBuf, crate::types::FileOrderEntry>> {
    use crate::file_ordering::{
        build_dependency_map, build_entries, build_file_dag, detect_cycles, ordered_by_name,
        topological_sort,
    };
    use crate::layer_core::layer_constrained_sort;
    use crate::layer_utilities::build_file_layers;
    use crate::types::FileOrderingResult;
    use std::collections::HashSet;

    const DEFAULT_STEP: usize = 10;

    if files.is_empty() {
        return Ok(HashMap::new());
    }
    let file_set: HashSet<PathBuf> = files.iter().cloned().collect();
    let module_map = crate::cluster_011::build_module_map(files);
    let dep_map = build_dependency_map(files, &file_set, &module_map)?;
    let file_layers = build_file_layers(files);
    let (graph, node_map) = build_file_dag(files, &dep_map);
    let cycles = detect_cycles(&graph, files);

    let ordered_nodes = if cycles.is_empty() {
        layer_constrained_sort(&graph, &file_layers).unwrap_or_else(|_| {
            topological_sort(&graph).unwrap_or_else(|_| ordered_by_name(files, &node_map))
        })
    } else {
        ordered_by_name(files, &node_map)
    };

    let ordered_files = ordered_nodes
        .into_iter()
        .map(|idx| graph[idx].clone())
        .collect::<Vec<_>>();

    let ordering = FileOrderingResult {
        ordered_files: build_entries(&ordered_files, DEFAULT_STEP),
        violations: Vec::new(),
        layer_violations: Vec::new(),
        ordered_directories: Vec::new(),
        cycles,
    };
    let mut map = HashMap::new();
    for entry in ordering.ordered_files {
        map.insert(entry.current_path.clone(), entry);
    }
    Ok(map)
}

pub fn collect_naming_warnings(
    directory: &crate::types::DirectoryAnalysis,
    config: &crate::report::ReportConfig,
    warnings: &mut Vec<String>,
) -> Result<()> {
    use crate::utilities::compress_path;
    use crate::dependency::naming_score_for_file;
    if directory
        .path
        .components()
        .any(|comp| comp.as_os_str() == "_old")
    {
        return Ok(());
    }
    let file_map = build_directory_entry_map(&directory.files)?;
    for file in &directory.files {
        if file.components().any(|comp| comp.as_os_str() == "_old") {
            continue;
        }
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

#[cfg(test)]
fn temp_dir(name: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!(
        "mmsb_analyzer_{}_{}",
        name,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    dir
}

#[cfg(test)]
fn detects_cycles() -> Result<()> {
    use crate::dependency::analyze_file_ordering;
    use std::fs::{create_dir_all, write};

    let dir = temp_dir("cycle");
    create_dir_all(&dir)?;
    let a = dir.join("a.rs");
    let b = dir.join("b.rs");
    write(&a, "use crate::b; pub fn a() {}")?;
    write(&b, "use crate::a; pub fn b() {}")?;

    let result = analyze_file_ordering(&[a.clone(), b.clone()], Some(10))?;
    assert!(!result.cycles.is_empty());
    Ok(())
}

#[cfg(test)]
fn generates_canonical_names_and_violations() -> Result<()> {
    use crate::dependency::analyze_file_ordering;
    use std::fs::{create_dir_all, write};

    let dir = temp_dir("names");
    create_dir_all(&dir)?;
    let a = dir.join("a.rs");
    let b = dir.join("b.rs");
    write(&a, "use crate::b; pub fn a() {}")?;
    write(&b, "pub fn b() {}")?;

    let result = analyze_file_ordering(&[a.clone(), b.clone()], Some(10))?;
    let entries = &result.ordered_files;
    assert_eq!(entries[0].suggested_name, "000_b.rs");
    assert_eq!(entries[1].suggested_name, "010_a.rs");
    assert!(!result.violations.is_empty());
    Ok(())
}

#[cfg(test)]
#[allow(dead_code)]
fn topo_sort_orders_dependencies() -> Result<()> {
    use crate::dependency::analyze_file_ordering;
    use std::fs::{create_dir_all, write};

    let dir = temp_dir("topo");
    create_dir_all(&dir)?;
    let a = dir.join("a.rs");
    let b = dir.join("b.rs");
    let c = dir.join("c.rs");
    write(&a, "pub fn a() {}")?;
    write(&b, "use crate::a; pub fn b() {}")?;
    write(&c, "use crate::b; pub fn c() {}")?;

    let result = analyze_file_ordering(&[c.clone(), b.clone(), a.clone()], Some(10))?;
    let ordered: Vec<_> = result
        .ordered_files
        .iter()
        .map(|entry| entry.current_path.clone())
        .collect();
    assert_eq!(ordered, vec![a, b, c]);
    assert!(result.cycles.is_empty());
    Ok(())
}

// ============================================================================
// From src/010_layer_core.rs
// ============================================================================

pub fn layer_constrained_sort(
    graph: &DiGraph<PathBuf, ()>,
    file_layers: &HashMap<PathBuf, String>,
) -> Result<Vec<NodeIndex>> {
    use crate::cluster_006::layer_prefix_value;

    let mut layer_nodes: BTreeMap<i32, Vec<NodeIndex>> = BTreeMap::new();
    for node in graph.node_indices() {
        let file = &graph[node];
        let layer_name = file_layers
            .get(file)
            .cloned()
            .unwrap_or_else(|| "root".to_string());
        let layer_value = layer_prefix_value(&layer_name).unwrap_or(0);
        layer_nodes.entry(layer_value).or_default().push(node);
    }

    let mut ordered = Vec::new();
    for (_layer, nodes) in layer_nodes {
        let sorted = topo_sort_within(graph, &nodes)?;
        ordered.extend(sorted);
    }
    Ok(ordered)
}

pub fn topo_sort_within(
    graph: &DiGraph<PathBuf, ()>,
    nodes: &[NodeIndex],
) -> Result<Vec<NodeIndex>> {
    let node_set: HashSet<NodeIndex> = nodes.iter().copied().collect();
    let mut indegree: HashMap<NodeIndex, usize> = HashMap::new();
    for &node in nodes {
        indegree.insert(node, 0);
    }
    for &node in nodes {
        let incoming = graph
            .neighbors_directed(node, petgraph::Direction::Incoming)
            .filter(|n| node_set.contains(n))
            .count();
        indegree.insert(node, incoming);
    }
    let mut queue = std::collections::VecDeque::new();
    for &node in nodes {
        if indegree.get(&node).copied().unwrap_or(0) == 0 {
            queue.push_back(node);
        }
    }
    let mut ordered = Vec::new();
    while let Some(node) = queue.pop_front() {
        ordered.push(node);
        for neighbor in graph.neighbors_directed(node, petgraph::Direction::Outgoing) {
            if !node_set.contains(&neighbor) {
                continue;
            }
            if let Some(entry) = indegree.get_mut(&neighbor) {
                *entry = entry.saturating_sub(1);
                if *entry == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    if ordered.len() != nodes.len() {
        return Err(anyhow::anyhow!("Cycle detected within layer group"));
    }
    Ok(ordered)
}

// ============================================================================
// From src/020_layer_utilities.rs
// ============================================================================

/// Detects the layer identifier from a path by finding first digit-prefixed component
pub fn detect_layer(path: &Path) -> String {
    for component in path.components() {
        if let Some(name) = component.as_os_str().to_str() {
            if let Some(first) = name.chars().next() {
                if first.is_ascii_digit() {
                    if let Some(pos) = name.find('_') {
                        if name[..pos].chars().all(|c| c.is_ascii_digit()) {
                            return name.to_string();
                        }
                    }
                }
            }
        }
    }
    "root".to_string()
}

pub fn rust_entry_paths(root: &Path) -> BTreeSet<PathBuf> {
    let src_dir = crate::layer_utilities::resolve_source_root(root);
    ["lib.rs", "main.rs"]
        .iter()
        .map(|rel| src_dir.join(rel))
        .filter(|p| p.exists())
        .collect()
}

#[derive(Clone)]
struct RustDependency {
    root: String,
    detail: String,
}

fn collect_rust_dependencies(path: &Path) -> Result<Vec<RustDependency>> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Unable to read Rust file {:?}", path))?;
    let syntax = syn::parse_file(&content)
        .with_context(|| format!("Unable to parse Rust file {:?}", path))?;
    let mut collector = UseCollector::default();
    collector.visit_file(&syntax);
    Ok(collector.deps)
}

#[derive(Default)]
struct UseCollector {
    deps: Vec<RustDependency>,
}

impl<'ast> Visit<'ast> for UseCollector {
    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        let mut roots = BTreeSet::new();
        collect_roots_from_crate(&node.tree, CrateRootState::Start, &mut roots);
        let stmt = quote::quote!(#node).to_string();
        for root in roots {
            self.deps.push(RustDependency {
                root,
                detail: stmt.clone(),
            });
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum CrateRootState {
    Start,
    AfterCrate,
}

fn collect_roots_from_crate(tree: &UseTree, state: CrateRootState, acc: &mut BTreeSet<String>) {
    match tree {
        UseTree::Path(path) => {
            let ident = path.ident.to_string();
            if state == CrateRootState::Start && ident == "crate" {
                collect_roots_from_crate(&path.tree, CrateRootState::AfterCrate, acc);
            } else if state == CrateRootState::AfterCrate {
                acc.insert(ident);
            } else {
                collect_roots_from_crate(&path.tree, state, acc);
            }
        }
        UseTree::Group(group) => {
            for tree in &group.items {
                collect_roots_from_crate(tree, state, acc);
            }
        }
        UseTree::Name(name) => {
            if state == CrateRootState::AfterCrate {
                acc.insert(name.ident.to_string());
            }
        }
        UseTree::Rename(rename) => {
            if state == CrateRootState::AfterCrate {
                acc.insert(rename.ident.to_string());
            }
        }
        UseTree::Glob(_) => {}
    }
}

/// Order Rust files by dependency and capture layer graph details.
pub fn order_rust_files_by_dependency(
    files: &[PathBuf],
    root: &Path,
) -> Result<(Vec<PathBuf>, LayerGraph)> {
    let module_map = crate::cluster_010::build_module_root_map(root)?;
    let entry_files = rust_entry_paths(root);
    let mut file_layers: HashMap<PathBuf, String> = HashMap::new();
    let mut nodes: BTreeSet<String> = BTreeSet::new();
    let mut edges_map: BTreeMap<(String, String), BTreeSet<ReferenceDetail>> = BTreeMap::new();
    let mut unresolved = Vec::new();

    for file in files {
        let layer = detect_layer(file);
        nodes.insert(layer.clone());
        file_layers.insert(file.clone(), layer.clone());

        let deps = collect_rust_dependencies(file)
            .with_context(|| format!("Failed to collect dependencies for {:?}", file))?;
        for dep in deps {
            if let Some(info) = module_map.get(&dep.root) {
                nodes.insert(info.layer.clone());
                if info.layer != layer {
                    edges_map
                        .entry((info.layer.clone(), layer.clone()))
                        .or_default()
                        .insert(ReferenceDetail {
                            file: file.clone(),
                            reference: dep.detail.clone(),
                        });
                }
            } else {
                unresolved.push(UnresolvedDependency {
                    file: file.clone(),
                    reference: dep.detail.clone(),
                });
            }
        }
    }

    crate::cluster_008::build_result(
        files,
        file_layers,
        nodes,
        edges_map,
        unresolved,
        &entry_files,
    )
}

// ============================================================================
// Julia Dependency Analysis (from src/000_dependency.rs)
// ============================================================================

#[derive(Clone)]
pub(crate) struct JuliaDependency {
    pub(crate) target: JuliaTarget,
    pub(crate) detail: String,
}

#[derive(Clone)]
pub(crate) enum JuliaTarget {
    Include(PathBuf),
    Module(String),
}

static INCLUDE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)include\s*\(\s*["']([^"'\n]+)["']"#).expect("failed to compile include regex")
});
static USING_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)(?:using|import)\s+MMSB\.([A-Za-z0-9_\.]+)"#)
        .expect("failed to compile using regex")
});
static ROOT_USING_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)(?:using|import)\s+MMSB\s*:\s*([A-Za-z0-9_,\s]+)"#)
        .expect("failed to compile root using regex")
});
static LOCAL_USING_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)(?:using|import)\s+\.\s*([A-Za-z0-9_]+)"#)
        .expect("failed to compile local using regex")
});

pub(crate) fn collect_julia_dependencies(path: &Path) -> Result<Vec<JuliaDependency>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Unable to read Julia file {:?}", path))?;
    let mut deps = Vec::new();

    for cap in INCLUDE_REGEX.captures_iter(&content) {
        if let Some(path_match) = cap.get(1) {
            let relative = PathBuf::from(path_match.as_str());
            let detail = cap
                .get(0)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            deps.push(JuliaDependency {
                target: JuliaTarget::Include(relative),
                detail,
            });
        }
    }

    for cap in USING_REGEX.captures_iter(&content) {
        if let Some(module_match) = cap.get(1) {
            let module = module_match.as_str();
            let primary = module.split('.').next().unwrap_or(module).to_string();
            let detail = cap
                .get(0)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            deps.push(JuliaDependency {
                target: JuliaTarget::Module(primary),
                detail,
            });
        }
    }

    for cap in ROOT_USING_REGEX.captures_iter(&content) {
        if let Some(symbols) = cap.get(1) {
            let detail = cap
                .get(0)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            for symbol in symbols
                .as_str()
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                let primary = symbol.split('.').next().unwrap_or(symbol).to_string();
                deps.push(JuliaDependency {
                    target: JuliaTarget::Module(primary),
                    detail: detail.clone(),
                });
            }
        }
    }

    for cap in LOCAL_USING_REGEX.captures_iter(&content) {
        if let Some(module_match) = cap.get(1) {
            let module = module_match.as_str();
            let detail = cap
                .get(0)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            deps.push(JuliaDependency {
                target: JuliaTarget::Module(module.to_string()),
                detail,
            });
        }
    }

    Ok(deps)
}

pub fn julia_entry_paths(root: &Path) -> BTreeSet<PathBuf> {
    let src_dir = crate::layer_utilities::resolve_source_root(root);
    ["MMSB.jl", "API.jl", "MMSB/API.jl"]
        .iter()
        .map(|rel| src_dir.join(rel))
        .filter(|p| p.exists())
        .collect()
}

pub fn build_file_layers(files: &[PathBuf]) -> HashMap<PathBuf, String> {
    let mut layers = HashMap::new();
    for file in files {
        layers.insert(file.clone(), detect_layer(file));
    }
    layers
}

pub fn gather_julia_files(root: &Path) -> Vec<PathBuf> {
    use walkdir::WalkDir;

    let src_root = crate::layer_utilities::resolve_source_root(root);
    WalkDir::new(&src_root)
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            if !entry.file_type().is_dir() {
                return true;
            }
            crate::layer_utilities::allow_analysis_dir(&src_root, entry.path())
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "jl"))
        .filter(|e| {
            let rel = e.path().strip_prefix(&src_root).unwrap_or(e.path());
            rel.components().count() == 1 || e.path().starts_with(src_root.join("src"))
        })
        .map(|entry| entry.into_path())
    .collect()
}

// ============================================================================
// From src/090_file_ordering.rs
// ============================================================================

pub fn topological_sort(graph: &DiGraph<PathBuf, ()>) -> Result<Vec<NodeIndex>> {
    use petgraph::Direction;
    use std::collections::VecDeque;

    let mut indegree = vec![0usize; graph.node_count()];
    for node in graph.node_indices() {
        indegree[node.index()] = graph
            .neighbors_directed(node, Direction::Incoming)
            .count();
    }

    let mut queue = VecDeque::new();
    for node in graph.node_indices() {
        if indegree[node.index()] == 0 {
            queue.push_back(node);
        }
    }

    let mut ordered = Vec::new();
    while let Some(node) = queue.pop_front() {
        ordered.push(node);
        for neighbor in graph.neighbors_directed(node, Direction::Outgoing) {
            let entry = &mut indegree[neighbor.index()];
            *entry = entry.saturating_sub(1);
            if *entry == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    if ordered.len() != graph.node_count() {
        return Err(anyhow::anyhow!("Cycle detected in dependency graph"));
    }

    Ok(ordered)
}

pub fn ordered_by_name(
    files: &[PathBuf],
    node_map: &HashMap<PathBuf, NodeIndex>,
) -> Vec<NodeIndex> {
    let mut sorted = files.to_vec();
    sorted.sort();
    sorted
        .into_iter()
        .filter_map(|path| node_map.get(&path).copied())
        .collect()
}

/// Builds file ordering entries with canonical names and rename flags
pub fn build_entries(ordered: &[PathBuf], step: usize) -> Vec<crate::types::FileOrderEntry> {
    ordered
        .iter()
        .enumerate()
        .map(|(idx, path)| {
            let canonical_order = idx * step;
            let suggested_name =
                crate::cluster_006::generate_canonical_name(path, canonical_order);
            let needs_rename = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| name != suggested_name)
                .unwrap_or(false);
            crate::types::FileOrderEntry {
                current_path: path.clone(),
                canonical_order,
                suggested_name,
                needs_rename,
            }
        })
        .collect()
}

pub fn analyze_file_ordering(
    files: &[PathBuf],
    step: Option<usize>,
) -> Result<crate::types::FileOrderingResult> {
    let step = step.unwrap_or(10);
    let file_set: HashSet<PathBuf> = files.iter().cloned().collect();
    let module_map = crate::cluster_011::build_module_map(files);
    let dep_map = crate::cluster_010::build_dependency_map(files, &file_set, &module_map)?;
    let file_layers = build_file_layers(files);
    let ordered_directories = crate::layer_core::order_directories(files, &dep_map);

    let (graph, node_map) = crate::cluster_011::build_file_dag(files, &dep_map);
    let layer_violations = crate::cluster_008::detect_layer_violations(&graph, &file_layers);
    let cycles = detect_cycles(&graph, files);

    let ordered_nodes = if cycles.is_empty() {
        crate::layer_core::layer_constrained_sort(&graph, &file_layers).unwrap_or_else(|_| {
            topological_sort(&graph).unwrap_or_else(|_| ordered_by_name(files, &node_map))
        })
    } else {
        ordered_by_name(files, &node_map)
    };

    let ordered_files = ordered_nodes
        .into_iter()
        .map(|idx| graph[idx].clone())
        .collect::<Vec<_>>();

    let file_entries = build_entries(&ordered_files, step);
    let violations = detect_violations(&file_entries, &dep_map);

    Ok(crate::types::FileOrderingResult {
        ordered_files: file_entries,
        violations,
        layer_violations,
        ordered_directories,
        cycles,
    })
}

pub fn naming_score_for_file(
    file: &Path,
    order_entry: Option<&crate::types::FileOrderEntry>,
) -> Option<f64> {
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

pub(crate) fn detect_cycles(
    graph: &DiGraph<PathBuf, ()>,
    files: &[PathBuf],
) -> Vec<Vec<PathBuf>> {
    let sccs = tarjan_scc(graph);
    let mut cycles = Vec::new();
    for scc in sccs {
        if scc.len() > 1 {
            cycles.push(scc.into_iter().map(|idx| graph[idx].clone()).collect());
        }
    }
    if cycles.is_empty() {
        return cycles;
    }
    if cycles.iter().all(|cycle| cycle.is_empty()) {
        let mut fallback = files.to_vec();
        fallback.sort();
        cycles.push(fallback);
    }
    cycles
}

pub(crate) fn detect_violations(
    ordered_files: &[crate::types::FileOrderEntry],
    dep_map: &HashMap<PathBuf, Vec<PathBuf>>,
) -> Vec<crate::types::OrderViolation> {
    let mut alpha = ordered_files.to_vec();
    alpha.sort_by(|a, b| a.current_path.cmp(&b.current_path));
    let alpha_positions: HashMap<PathBuf, usize> = alpha
        .iter()
        .enumerate()
        .map(|(idx, entry)| (entry.current_path.clone(), idx))
        .collect();

    let canonical_positions: HashMap<PathBuf, usize> = ordered_files
        .iter()
        .enumerate()
        .map(|(idx, entry)| (entry.current_path.clone(), idx))
        .collect();

    let mut violations = Vec::new();
    for entry in ordered_files {
        let Some(&alpha_pos) = alpha_positions.get(&entry.current_path) else {
            continue;
        };
        let Some(&required_pos) = canonical_positions.get(&entry.current_path) else {
            continue;
        };
        if alpha_pos != required_pos {
            let blocking_dependencies = dep_map
                .get(&entry.current_path)
                .map(|deps| {
                    deps.iter()
                        .filter(|dep| {
                            let dep_alpha = alpha_positions.get(*dep).copied().unwrap_or(0);
                            dep_alpha > alpha_pos
                        })
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            violations.push(crate::types::OrderViolation {
                file: entry.current_path.clone(),
                current_position: alpha_pos,
                required_position: required_pos,
                blocking_dependencies,
            });
        }
    }

    violations
}

// ============================================================================
// From src/120_dot_exporter.rs
// ============================================================================

/// Exports a complete program CFG to DOT format
pub fn export_complete_program_dot(
    program: &crate::types::ProgramCFG,
    path: &str,
) -> std::io::Result<()> {
    use std::collections::HashMap;
    use std::fmt::Write;

    fn escape_dot(s: &str) -> String {
        s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
    }

    let mut dot = String::new();

    writeln!(dot, "digraph ProgramCFG {{").unwrap();
    writeln!(dot, "  rankdir=TB;").unwrap();
    writeln!(dot, "  compound=true;").unwrap();
    writeln!(dot, "  newrank=true;").unwrap();
    writeln!(
        dot,
        "  label=\"Complete Program CFG - {} functions\";",
        program.functions.len()
    )
    .unwrap();
    writeln!(dot, "  labelloc=t;").unwrap();
    writeln!(dot, "  fontsize=16;").unwrap();
    writeln!(dot, "").unwrap();

    let mut funcs: Vec<_> = program.functions.iter().collect();
    funcs.sort_by_key(|(fid, _)| fid.as_str());

    let mut func_to_cluster: HashMap<&String, usize> = HashMap::new();

    for (cluster_idx, (func_id, cfg)) in funcs.iter().enumerate() {
        let safe_name = func_id.replace(['!', '?', '*'], "_");
        let cc = crate::cluster_008::cyclomatic_complexity(cfg);
        func_to_cluster.insert(func_id, cluster_idx);

        writeln!(dot, "  subgraph cluster_{} {{", cluster_idx).unwrap();
        writeln!(dot, "    label=\"{} (CC={})\";", safe_name, cc).unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    fillcolor=lightgray;").unwrap();
        writeln!(dot, "    color=black;").unwrap();
        writeln!(dot, "").unwrap();

        for node in &cfg.nodes {
            let (shape, color, style) = crate::cluster_008::node_style(&node.node_type);

            let mut label = node.label.clone();
            if !node.lines.is_empty() {
                let lines_str: String = node
                    .lines
                    .iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                label = format!("{} L{}", label, lines_str);
            }

            let url = format!("http://127.0.0.1:8081/run?f={}", func_id);

            writeln!(
                dot,
                "    f{}_n{} [label=\"{}\", shape={}, fillcolor={}, style={}, URL=\"{}\"];",
                cluster_idx,
                node.id,
                escape_dot(&label),
                shape,
                color,
                style,
                url
            )
            .unwrap();
        }

        writeln!(dot, "").unwrap();

        for edge in &cfg.edges {
            let mut attrs = Vec::new();
            if let Some(cond) = edge.condition {
                let label = if cond { "T" } else { "F" };
                let color = if cond { "darkgreen" } else { "red" };
                attrs.push(format!("label=\"{}\"", label));
                attrs.push(format!("color=\"{}\"", color));
            }
            let attr_str = if attrs.is_empty() {
                "".to_string()
            } else {
                format!(" [{}]", attrs.join(", "))
            };

            writeln!(
                dot,
                "    f{}_n{} -> f{}_n{}{};",
                cluster_idx,
                edge.from,
                cluster_idx,
                edge.to,
                attr_str
            )
            .unwrap();
        }

        writeln!(dot, "  }}").unwrap();
        writeln!(dot, "").unwrap();
    }

    writeln!(dot, "  // Inter-function calls").unwrap();
    writeln!(dot, "  edge [style=dashed, color=blue, penwidth=2];").unwrap();
    writeln!(dot, "").unwrap();

    for (caller, callee) in &program.call_edges {
        if let (Some(&caller_idx), Some(&callee_idx)) =
            (func_to_cluster.get(caller), func_to_cluster.get(callee))
        {
            if let (Some(caller_cfg), Some(callee_cfg)) =
                (program.functions.get(caller), program.functions.get(callee))
            {
                writeln!(
                    dot,
                    "  f{}_n{} -> f{}_n{} [ltail=cluster_{}, lhead=cluster_{}, label=\"call\"];",
                    caller_idx,
                    caller_cfg.exit_id,
                    callee_idx,
                    callee_cfg.entry_id,
                    caller_idx,
                    callee_idx
                )
                .unwrap();
            }
        }
    }

    writeln!(dot, "}}").unwrap();

    std::fs::write(path, dot)?;
    println!("Complete program CFG exported to {}", path);
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_cycles() {
        detects_cycles().unwrap();
    }

    #[test]
    fn test_generates_canonical_names_and_violations() {
        generates_canonical_names_and_violations().unwrap();
    }
}
