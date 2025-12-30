//! File-level dependency ordering using a DAG.

use anyhow::{Context, Result};
use petgraph::algo::tarjan_scc;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use petgraph::visit::EdgeRef;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use syn::visit::Visit;
use syn::{ItemUse, UseTree};
use walkdir::WalkDir;

use crate::dependency::detect_layer;
use crate::types::{FileLayerViolation, FileOrderEntry, FileOrderingResult, OrderViolation};

const DEFAULT_STEP: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RootState {
    Start,
    AfterRoot,
}

pub fn analyze_file_ordering(
    files: &[PathBuf],
    step: Option<usize>,
) -> Result<FileOrderingResult> {
    let step = step.unwrap_or(DEFAULT_STEP);
    let file_set: HashSet<PathBuf> = files.iter().cloned().collect();
    let module_map = build_module_map(files);
    let dep_map = build_dependency_map(files, &file_set, &module_map)?;
    let file_layers = build_file_layers(files);
    let ordered_directories = order_directories(files, &dep_map);

    let (graph, node_map) = build_file_dag(files, &dep_map);
    let layer_violations = detect_layer_violations(&graph, &file_layers);
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

    let file_entries = build_entries(&ordered_files, step);
    let violations = detect_violations(&file_entries, &dep_map);

    Ok(FileOrderingResult {
        ordered_files: file_entries,
        violations,
        layer_violations,
        ordered_directories,
        cycles,
    })
}

fn build_dependency_map(
    files: &[PathBuf],
    file_set: &HashSet<PathBuf>,
    module_map: &HashMap<String, PathBuf>,
) -> Result<HashMap<PathBuf, Vec<PathBuf>>> {
    let mut dep_map: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    for file in files {
        let deps = extract_dependencies(file, file_set, module_map)
            .with_context(|| format!("Failed to extract dependencies for {:?}", file))?;
        dep_map.insert(file.clone(), deps);
    }
    Ok(dep_map)
}

pub fn build_file_dependency_graph(files: &[PathBuf]) -> Result<DiGraph<PathBuf, ()>> {
    let file_set: HashSet<PathBuf> = files.iter().cloned().collect();
    let module_map = build_module_map(files);
    let dep_map = build_dependency_map(files, &file_set, &module_map)?;
    let (graph, _) = build_file_dag(files, &dep_map);
    Ok(graph)
}

fn order_directories(
    files: &[PathBuf],
    dep_map: &HashMap<PathBuf, Vec<PathBuf>>,
) -> Vec<PathBuf> {
    let root = common_root(files);
    let mut dirs: HashSet<PathBuf> = HashSet::new();
    for file in files {
        let mut current = file.parent().map(Path::to_path_buf);
        while let Some(dir) = current {
            if let Some(ref root_path) = root {
                if !dir.starts_with(root_path) {
                    break;
                }
            }
            dirs.insert(dir.clone());
            current = dir.parent().map(Path::to_path_buf);
        }
    }

    let mut ordered: Vec<PathBuf> = dirs.into_iter().collect();
    ordered.sort_by(|a, b| compare_path_components(a, b));

    let mut node_map = HashMap::new();
    for (idx, dir) in ordered.iter().enumerate() {
        node_map.insert(dir.clone(), idx);
    }

    let mut adjacency: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); ordered.len()];
    let mut indegree = vec![0usize; ordered.len()];

    for (file, deps) in dep_map {
        let Some(from_dir) = file.parent().map(Path::to_path_buf) else {
            continue;
        };
        let Some(&from_idx) = node_map.get(&from_dir) else {
            continue;
        };
        for dep in deps {
            let Some(to_dir) = dep.parent().map(Path::to_path_buf) else {
                continue;
            };
            if to_dir == from_dir {
                continue;
            }
            let Some(&to_idx) = node_map.get(&to_dir) else {
                continue;
            };
            if adjacency[to_idx].insert(from_idx) {
                indegree[from_idx] += 1;
            }
        }
    }

    let mut queue: BTreeSet<usize> = indegree
        .iter()
        .enumerate()
        .filter_map(|(idx, &deg)| if deg == 0 { Some(idx) } else { None })
        .collect();

    let mut result = Vec::with_capacity(ordered.len());
    while let Some(&idx) = queue.iter().next() {
        queue.remove(&idx);
        result.push(ordered[idx].clone());
        let neighbors = adjacency[idx].clone();
        for neighbor in neighbors {
            let entry = &mut indegree[neighbor];
            if *entry > 0 {
                *entry -= 1;
                if *entry == 0 {
                    queue.insert(neighbor);
                }
            }
        }
    }

    if result.len() < ordered.len() {
        for (idx, dir) in ordered.iter().enumerate() {
            if indegree[idx] > 0 {
                result.push(dir.clone());
            }
        }
    }

    result
}

fn common_root(files: &[PathBuf]) -> Option<PathBuf> {
    let mut iter = files.iter();
    let first = iter.next()?.components().collect::<Vec<_>>();
    let mut prefix_len = first.len();

    for path in iter {
        let comps = path.components().collect::<Vec<_>>();
        let mut idx = 0;
        while idx < prefix_len && idx < comps.len() && comps[idx] == first[idx] {
            idx += 1;
        }
        prefix_len = idx;
    }

    if prefix_len == 0 {
        None
    } else {
        let mut root = PathBuf::new();
        for comp in first.into_iter().take(prefix_len) {
            root.push(comp.as_os_str());
        }
        Some(root)
    }
}

fn compare_path_components(a: &Path, b: &Path) -> std::cmp::Ordering {
    let a_components: Vec<_> = a.components().collect();
    let b_components: Vec<_> = b.components().collect();
    let min_len = a_components.len().min(b_components.len());

    for idx in 0..min_len {
        let a_name = a_components[idx].as_os_str().to_string_lossy();
        let b_name = b_components[idx].as_os_str().to_string_lossy();
        let a_prefix = layer_prefix_value(&a_name);
        let b_prefix = layer_prefix_value(&b_name);
        let cmp = match (a_prefix, b_prefix) {
            (Some(a_val), Some(b_val)) => a_val.cmp(&b_val),
            _ => a_name.cmp(&b_name),
        };
        if cmp != std::cmp::Ordering::Equal {
            return cmp;
        }
    }

    a_components.len().cmp(&b_components.len())
}

fn build_file_layers(files: &[PathBuf]) -> HashMap<PathBuf, String> {
    let mut layers = HashMap::new();
    for file in files {
        layers.insert(file.clone(), detect_layer(file));
    }
    layers
}

#[allow(dead_code)]
pub struct DagCache {
    graph: DiGraph<PathBuf, ()>,
    node_map: HashMap<PathBuf, NodeIndex>,
    topo_order: Vec<NodeIndex>,
    last_modified: HashMap<PathBuf, SystemTime>,
    file_set: HashSet<PathBuf>,
    module_map: HashMap<String, PathBuf>,
}

#[allow(dead_code)]
impl DagCache {
    pub fn new(files: &[PathBuf]) -> Result<Self> {
        let file_set: HashSet<PathBuf> = files.iter().cloned().collect();
        let module_map = build_module_map(files);
        let dep_map = build_dependency_map(files, &file_set, &module_map)?;
        let (graph, node_map) = build_file_dag(files, &dep_map);
        let topo_order = topological_sort(&graph).unwrap_or_else(|_| {
            graph
                .node_indices()
                .collect::<Vec<_>>()
        });

        let mut last_modified = HashMap::new();
        for file in files {
            if let Ok(meta) = fs::metadata(file) {
                if let Ok(modified) = meta.modified() {
                    last_modified.insert(file.clone(), modified);
                }
            }
        }

        Ok(Self {
            graph,
            node_map,
            topo_order,
            last_modified,
            file_set,
            module_map,
        })
    }

    pub fn incremental_update(&mut self, changed_files: &[PathBuf]) -> Result<()> {
        for file in changed_files {
            if !self.file_set.contains(file) {
                continue;
            }
            let Some(&node) = self.node_map.get(file) else {
                continue;
            };
            let old_edges: Vec<_> = self.graph.edges(node).map(|e| e.id()).collect();
            for edge in old_edges {
                self.graph.remove_edge(edge);
            }

            let deps = extract_dependencies(file, &self.file_set, &self.module_map)?;
            for dep in deps {
                if let Some(&dep_node) = self.node_map.get(&dep) {
                    self.graph.add_edge(dep_node, node, ());
                }
            }

            if let Ok(meta) = fs::metadata(file) {
                if let Ok(modified) = meta.modified() {
                    self.last_modified.insert(file.clone(), modified);
                }
            }
        }

        if let Ok(order) = topological_sort(&self.graph) {
            self.topo_order = order;
        }
        Ok(())
    }

    pub fn topo_files(&self) -> Vec<PathBuf> {
        self.topo_order
            .iter()
            .map(|idx| self.graph[*idx].clone())
            .collect()
    }
}

#[allow(dead_code)]
pub fn parallel_build_file_dag(directories: &[PathBuf]) -> Result<DiGraph<PathBuf, ()>> {
    let subgraphs: Vec<DiGraph<PathBuf, ()>> = directories
        .par_iter()
        .map(build_directory_dag)
        .collect::<Result<_>>()?;

    let mut merged = DiGraph::new();
    let mut node_map: HashMap<PathBuf, NodeIndex> = HashMap::new();

    for subgraph in subgraphs {
        for node in subgraph.node_indices() {
            let file = subgraph[node].clone();
            node_map.entry(file.clone()).or_insert_with(|| merged.add_node(file));
        }
        for edge in subgraph.edge_indices() {
            if let Some((src, dst)) = subgraph.edge_endpoints(edge) {
                let src_file = subgraph[src].clone();
                let dst_file = subgraph[dst].clone();
                let src_node = *node_map.get(&src_file).expect("missing source node");
                let dst_node = *node_map.get(&dst_file).expect("missing target node");
                merged.add_edge(src_node, dst_node, ());
            }
        }
    }

    Ok(merged)
}

#[allow(dead_code)]
fn build_directory_dag(dir: &PathBuf) -> Result<DiGraph<PathBuf, ()>> {
    let files: Vec<PathBuf> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "rs" || ext == "jl")
                .unwrap_or(false)
        })
        .map(|entry| entry.into_path())
        .collect();

    let file_set: HashSet<PathBuf> = files.iter().cloned().collect();
    let module_map = build_module_map(&files);
    let dep_map = build_dependency_map(&files, &file_set, &module_map)?;
    let (graph, _) = build_file_dag(&files, &dep_map);
    Ok(graph)
}

fn build_entries(ordered: &[PathBuf], step: usize) -> Vec<FileOrderEntry> {
    ordered
        .iter()
        .enumerate()
        .map(|(idx, path)| {
            let canonical_order = idx * step;
            let suggested_name = generate_canonical_name(path, canonical_order);
            let needs_rename = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| name != suggested_name)
                .unwrap_or(false);
            FileOrderEntry {
                current_path: path.clone(),
                canonical_order,
                suggested_name,
                needs_rename,
            }
        })
        .collect()
}

fn build_file_dag(
    files: &[PathBuf],
    dep_map: &HashMap<PathBuf, Vec<PathBuf>>,
) -> (DiGraph<PathBuf, ()>, HashMap<PathBuf, NodeIndex>) {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();

    for file in files {
        let node = graph.add_node(file.clone());
        node_map.insert(file.clone(), node);
    }

    for (file, deps) in dep_map {
        if let Some(&file_node) = node_map.get(file) {
            for dep in deps {
                if let Some(&dep_node) = node_map.get(dep) {
                    graph.add_edge(dep_node, file_node, ());
                }
            }
        }
    }

    (graph, node_map)
}

fn topological_sort(graph: &DiGraph<PathBuf, ()>) -> Result<Vec<NodeIndex>> {
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

fn layer_constrained_sort(
    graph: &DiGraph<PathBuf, ()>,
    file_layers: &HashMap<PathBuf, String>,
) -> Result<Vec<NodeIndex>> {
    let mut layer_nodes: BTreeMap<i32, Vec<NodeIndex>> = BTreeMap::new();
    for node in graph.node_indices() {
        let file = &graph[node];
        let layer_name = file_layers.get(file).cloned().unwrap_or_else(|| "root".to_string());
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

fn topo_sort_within(
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
            .neighbors_directed(node, Direction::Incoming)
            .filter(|n| node_set.contains(n))
            .count();
        indegree.insert(node, incoming);
    }

    let mut queue = VecDeque::new();
    for &node in nodes {
        if indegree.get(&node).copied().unwrap_or(0) == 0 {
            queue.push_back(node);
        }
    }

    let mut ordered = Vec::new();
    while let Some(node) = queue.pop_front() {
        ordered.push(node);
        for neighbor in graph.neighbors_directed(node, Direction::Outgoing) {
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

fn detect_cycles(graph: &DiGraph<PathBuf, ()>, files: &[PathBuf]) -> Vec<Vec<PathBuf>> {
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

fn ordered_by_name(
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

fn detect_violations(
    ordered_files: &[FileOrderEntry],
    dep_map: &HashMap<PathBuf, Vec<PathBuf>>,
) -> Vec<OrderViolation> {
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
            violations.push(OrderViolation {
                file: entry.current_path.clone(),
                current_position: alpha_pos,
                required_position: required_pos,
                blocking_dependencies,
            });
        }
    }

    violations
}

fn detect_layer_violations(
    graph: &DiGraph<PathBuf, ()>,
    file_layers: &HashMap<PathBuf, String>,
) -> Vec<FileLayerViolation> {
    let mut violations = Vec::new();
    for edge in graph.edge_references() {
        let from = &graph[edge.source()];
        let to = &graph[edge.target()];
        let from_layer = file_layers
            .get(from)
            .cloned()
            .unwrap_or_else(|| "root".to_string());
        let to_layer = file_layers
            .get(to)
            .cloned()
            .unwrap_or_else(|| "root".to_string());
        if let (Some(from_val), Some(to_val)) =
            (layer_prefix_value(&from_layer), layer_prefix_value(&to_layer))
        {
            if from_val > to_val {
                violations.push(FileLayerViolation {
                    from: from.clone(),
                    to: to.clone(),
                    from_layer,
                    to_layer,
                });
            }
        }
    }
    violations
}

fn generate_canonical_name(path: &Path, number: usize) -> String {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let clean_stem = strip_numeric_prefix(stem);
    if ext.is_empty() {
        format!("{:03}_{}", number, clean_stem)
    } else {
        format!("{:03}_{}.{}", number, clean_stem, ext)
    }
}

fn strip_numeric_prefix(name: &str) -> &str {
    static PREFIX_RE: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"^\d+_(.*)$").unwrap());
    PREFIX_RE
        .captures(name)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or(name)
}

fn layer_prefix_value(layer: &str) -> Option<i32> {
    let mut digits = String::new();
    for ch in layer.chars() {
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

fn extract_dependencies(
    file: &Path,
    file_set: &HashSet<PathBuf>,
    module_map: &HashMap<String, PathBuf>,
) -> Result<Vec<PathBuf>> {
    let ext = file.extension().and_then(|s| s.to_str()).unwrap_or("");
    match ext {
        "rs" => extract_rust_dependencies(file, file_set, module_map),
        "jl" => extract_julia_dependencies(file, file_set, module_map),
        _ => Ok(Vec::new()),
    }
}

fn extract_rust_dependencies(
    file: &Path,
    file_set: &HashSet<PathBuf>,
    module_map: &HashMap<String, PathBuf>,
) -> Result<Vec<PathBuf>> {
    let content =
        fs::read_to_string(file).with_context(|| format!("Unable to read {:?}", file))?;
    let syntax = syn::parse_file(&content)
        .with_context(|| format!("Unable to parse Rust file {:?}", file))?;
    let mut collector = UseCollector::default();
    collector.visit_file(&syntax);
    let mut deps = Vec::new();
    for root in collector.roots {
        if let Some(path) = resolve_module(&root, file_set, module_map) {
            deps.push(path);
        }
    }
    for module in collector.mods {
        if let Some(path) = resolve_module(&module, file_set, module_map) {
            deps.push(path);
        }
    }
    Ok(deps)
}

fn extract_julia_dependencies(
    file: &Path,
    file_set: &HashSet<PathBuf>,
    module_map: &HashMap<String, PathBuf>,
) -> Result<Vec<PathBuf>> {
    static INCLUDE_RE: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r#"include\s*\(\s*["']([^"']+)["']"#).unwrap());
    static MMSB_USING_RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r#"(?m)^\s*(?:using|import)\s+MMSB\.([A-Za-z0-9_\.]+)"#).unwrap()
    });
    static MMSB_SYMBOL_RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r#"(?m)^\s*(?:using|import)\s+MMSB\s*:\s*([A-Za-z0-9_,\s]+)"#).unwrap()
    });
    static LOCAL_USING_RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r#"(?m)^\s*(?:using|import)\s+\.\s*([A-Za-z0-9_\.]+)"#).unwrap()
    });
    static PLAIN_USING_RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r#"(?m)^\s*(?:using|import)\s+([A-Za-z_][A-Za-z0-9_\.]*)"#).unwrap()
    });

    let content =
        fs::read_to_string(file).with_context(|| format!("Unable to read {:?}", file))?;
    let mut deps = Vec::new();

    for cap in INCLUDE_RE.captures_iter(&content) {
        if let Some(path_match) = cap.get(1) {
            let raw = path_match.as_str();
            let mut candidate = PathBuf::from(raw);
            if candidate.extension().is_none() {
                candidate.set_extension("jl");
            }
            let resolved = if candidate.is_absolute() {
                candidate
            } else {
                file.parent()
                    .map(|p| p.join(&candidate))
                    .unwrap_or(candidate)
            };
            if let Some(path) = resolve_path(&resolved, file_set, module_map) {
                deps.push(path);
            }
        }
    }

    for cap in MMSB_USING_RE.captures_iter(&content) {
        if let Some(module_match) = cap.get(1) {
            if let Some(path) = resolve_module_name(module_match.as_str(), file_set, module_map) {
                deps.push(path);
            }
        }
    }

    for cap in MMSB_SYMBOL_RE.captures_iter(&content) {
        if let Some(symbols) = cap.get(1) {
            for symbol in symbols
                .as_str()
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                if let Some(path) = resolve_module_name(symbol, file_set, module_map) {
                    deps.push(path);
                }
            }
        }
    }

    for cap in LOCAL_USING_RE.captures_iter(&content) {
        if let Some(module_match) = cap.get(1) {
            if let Some(path) = resolve_module_name(module_match.as_str(), file_set, module_map) {
                deps.push(path);
            }
        }
    }

    for cap in PLAIN_USING_RE.captures_iter(&content) {
        if let Some(module_match) = cap.get(1) {
            let module = module_match.as_str();
            if module.starts_with("MMSB") {
                continue;
            }
            if let Some(path) = resolve_module_name(module, file_set, module_map) {
                deps.push(path);
            }
        }
    }

    Ok(deps)
}

fn resolve_module_name(
    module: &str,
    file_set: &HashSet<PathBuf>,
    module_map: &HashMap<String, PathBuf>,
) -> Option<PathBuf> {
    let primary = module.split('.').next().unwrap_or(module);
    resolve_module(primary, file_set, module_map)
}

fn resolve_module(
    root: &str,
    file_set: &HashSet<PathBuf>,
    module_map: &HashMap<String, PathBuf>,
) -> Option<PathBuf> {
    let key = normalize_module_name(root);
    if let Some(path) = module_map.get(&key) {
        return Some(path.clone());
    }
    module_map
        .iter()
        .find(|(name, _)| name == &&key)
        .map(|(_, path)| path.clone())
        .or_else(|| {
            module_map
                .iter()
                .find(|(name, _)| key.starts_with(name.as_str()))
                .map(|(_, path)| path.clone())
        })
        .or_else(|| resolve_path(&PathBuf::from(root), file_set, module_map))
}

fn resolve_path(
    candidate: &Path,
    file_set: &HashSet<PathBuf>,
    module_map: &HashMap<String, PathBuf>,
) -> Option<PathBuf> {
    if file_set.contains(candidate) {
        return Some(candidate.to_path_buf());
    }
    if let Some(file_name) = candidate.file_stem().and_then(|s| s.to_str()) {
        let key = normalize_module_name(file_name);
        if let Some(path) = module_map.get(&key) {
            return Some(path.clone());
        }
    }
    None
}

fn build_module_map(files: &[PathBuf]) -> HashMap<String, PathBuf> {
    let mut map = HashMap::new();
    for file in files {
        if let Some(stem) = file.file_stem().and_then(|s| s.to_str()) {
            let normalized = normalize_module_name(stem);
            map.insert(normalized.clone(), file.clone());
            if stem == "mod" {
                if let Some(parent) = file.parent().and_then(|p| p.file_name()) {
                    if let Some(name) = parent.to_str() {
                        map.insert(normalize_module_name(name), file.clone());
                    }
                }
            }
        }
    }
    map
}

fn normalize_module_name(name: &str) -> String {
    strip_numeric_prefix(name).to_lowercase()
}

#[derive(Default)]
struct UseCollector {
    roots: BTreeSet<String>,
    mods: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for UseCollector {
    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        collect_roots(&node.tree, RootState::Start, &mut self.roots);
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if node.content.is_none() {
            self.mods.insert(node.ident.to_string());
        }
    }
}

fn collect_roots(tree: &UseTree, state: RootState, acc: &mut BTreeSet<String>) {
    match tree {
        UseTree::Path(path) => {
            let ident = path.ident.to_string();
            if state == RootState::Start
                && matches!(ident.as_str(), "crate" | "self" | "super")
            {
                collect_roots(&path.tree, RootState::AfterRoot, acc);
            } else if state == RootState::AfterRoot {
                acc.insert(ident);
            } else {
                acc.insert(ident);
            }
        }
        UseTree::Group(group) => {
            for tree in &group.items {
                collect_roots(tree, state, acc);
            }
        }
        UseTree::Name(name) => {
            acc.insert(name.ident.to_string());
        }
        UseTree::Rename(rename) => {
            acc.insert(rename.ident.to_string());
        }
        UseTree::Glob(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, write};

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

    #[test]
    fn topo_sort_orders_dependencies() -> Result<()> {
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

    #[test]
    fn detects_cycles() -> Result<()> {
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

    #[test]
    fn generates_canonical_names_and_violations() -> Result<()> {
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
}
