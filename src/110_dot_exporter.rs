use std::collections::HashMap;
use std::fmt::Write;

use crate::types::{FunctionCfg, NodeType, ProgramCFG};

fn escape_dot(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn node_style(node_type: &NodeType) -> (&str, &str, &str) {
    match node_type {
        NodeType::Entry => ("ellipse", "lightgreen", "\"filled,bold\""),
        NodeType::Exit => ("doubleoctagon", "lightcoral", "\"filled,bold\""),
        NodeType::BasicBlock => ("box", "lightblue", "filled"),
        NodeType::Branch => ("diamond", "yellow", "filled"),
        NodeType::LoopHeader => ("box", "orange", "\"filled,rounded\""),
    }
}

fn cyclomatic_complexity(cfg: &FunctionCfg) -> usize {
    let edges = cfg.edges.len() as isize;
    let nodes = cfg.nodes.len() as isize;
    let exits = 1isize; // assume one exit
    let cc = edges - nodes + 2 * exits;
    if cc <= 0 { 1 } else { cc as usize }
}

pub fn export_complete_program_dot(program: &ProgramCFG, path: &str) -> std::io::Result<()> {
    let mut dot = String::new();

    writeln!(dot, "digraph ProgramCFG {{").unwrap();
    writeln!(dot, "  rankdir=TB;").unwrap();
    writeln!(dot, "  compound=true;").unwrap();
    writeln!(dot, "  newrank=true;").unwrap();
    writeln!(dot, "  label=\"Complete Program CFG - {} functions\";", program.functions.len()).unwrap();
    writeln!(dot, "  labelloc=t;").unwrap();
    writeln!(dot, "  fontsize=16;").unwrap();
    writeln!(dot, "").unwrap();

    // Sort functions for stable output
    let mut funcs: Vec<_> = program.functions.iter().collect();
    funcs.sort_by_key(|(fid, _)| fid.as_str());

    let mut func_to_cluster: HashMap<&String, usize> = HashMap::new();

    for (cluster_idx, (func_id, cfg)) in funcs.iter().enumerate() {
        let safe_name = func_id.replace(['!', '?', '*'], "_");
        let cc = cyclomatic_complexity(cfg);
        func_to_cluster.insert(func_id, cluster_idx);

        writeln!(dot, "  subgraph cluster_{} {{", cluster_idx).unwrap();
        writeln!(dot, "    label=\"{} (CC={})\";", safe_name, cc).unwrap();
        writeln!(dot, "    style=filled;").unwrap();
        writeln!(dot, "    fillcolor=lightgray;").unwrap();
        writeln!(dot, "    color=black;").unwrap();
        writeln!(dot, "").unwrap();

        // Nodes
        for node in &cfg.nodes {
            let (shape, color, style) = node_style(&node.node_type);

            let mut label = node.label.clone();
            if !node.lines.is_empty() {
                let lines_str: String = node.lines.iter()
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
            ).unwrap();
        }

        writeln!(dot, "").unwrap();

        // Intra-function edges
        for edge in &cfg.edges {
            let mut attrs = Vec::new();
            if let Some(cond) = edge.condition {
                let label = if cond { "T" } else { "F" };
                let color = if cond { "darkgreen" } else { "red" };
                attrs.push(format!("label=\"{}\"", label));
                attrs.push(format!("color=\"{}\"", color));
            }
            let attr_str = if attrs.is_empty() { "".to_string() } else { format!(" [{}]", attrs.join(", ")) };

            writeln!(
                dot,
                "    f{}_n{} -> f{}_n{}{};",
                cluster_idx, edge.from,
                cluster_idx, edge.to,
                attr_str
            ).unwrap();
        }

        writeln!(dot, "  }}").unwrap();
        writeln!(dot, "").unwrap();
    }

    // Inter-function call edges
    writeln!(dot, "  // Inter-function calls").unwrap();
    writeln!(dot, "  edge [style=dashed, color=blue, penwidth=2];").unwrap();
    writeln!(dot, "").unwrap();

    for (caller, callee) in &program.call_edges {
        if let (Some(&caller_idx), Some(&callee_idx)) = (func_to_cluster.get(caller), func_to_cluster.get(callee)) {
            if let (Some(caller_cfg), Some(callee_cfg)) = (program.functions.get(caller), program.functions.get(callee)) {
                writeln!(
                    dot,
                    "  f{}_n{} -> f{}_n{} [ltail=cluster_{}, lhead=cluster_{}, label=\"call\"];",
                    caller_idx, caller_cfg.exit_id,
                    callee_idx, callee_cfg.entry_id,
                    caller_idx, callee_idx
                ).unwrap();
            }
        }
    }

    writeln!(dot, "}}").unwrap();

    std::fs::write(path, dot)?;
    println!("Complete program CFG exported to {}", path);
    Ok(())
}

pub fn export_program_cfg_to_path(
    result: &crate::types::AnalysisResult,
    call_edges: &[(String, String)],
    output_path: &std::path::Path,
) -> std::io::Result<()> {
    let mut program_cfg = ProgramCFG {
        functions: HashMap::new(),
        call_edges: Vec::new(),
    };

    for cfg in &result.cfgs {
        program_cfg
            .functions
            .insert(cfg.function.clone(), cfg.clone());
    }

    for (caller, callee) in call_edges {
        let caller_name = caller.split("::").last().unwrap_or(caller).to_string();
        let callee_name = callee.split("::").last().unwrap_or(callee).to_string();
        if program_cfg.functions.contains_key(&caller_name)
            && program_cfg.functions.contains_key(&callee_name)
        {
            program_cfg.call_edges.push((caller_name, callee_name));
        }
    }

    let cfg_dir = output_path.join("cfg");
    std::fs::create_dir_all(&cfg_dir)?;
    let dot_path = cfg_dir.join("complete_program.dot");
    export_complete_program_dot(&program_cfg, dot_path.to_string_lossy().as_ref())?;

    #[cfg(feature = "png")]
    {
        let png_path = cfg_dir.join("complete_program.png");
        if let (Some(dot_path_str), Some(png_path_str)) =
            (dot_path.to_str(), png_path.to_str())
        {
            let _ = std::process::Command::new("dot")
                .args(["-Tpng", dot_path_str, "-o", png_path_str])
                .status();
        }
    }

    Ok(())
}
