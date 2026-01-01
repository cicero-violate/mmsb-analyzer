#![allow(dead_code)]
//! Predict violations for refactor actions.

use crate::correction_plan_types::{
    RefactorAction, Severity, ViolationPrediction, ViolationType,
};
use crate::correction_intelligence_report::{
    augment_path_coherence_strategies, compute_summary, fill_prediction_confidence,
    CorrectionIntelligenceReport, IntelligenceState,
};
use crate::correction_plan_generator::generate_correction_plan;
use crate::invariant_types::InvariantAnalysisResult;
use crate::rollback_criteria_builder::build_rollback_criteria;
use crate::types::{CallGraphNode, CodeElement};
use crate::verification_scope_planner::plan_verification_scope;
use crate::action_impact_estimator::{estimate_impact, AnalysisState as ImpactState};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub fn predict_violations(
    action: &RefactorAction,
    invariants: &InvariantAnalysisResult,
    call_graph: &HashMap<String, CallGraphNode>,
    elements: &[CodeElement],
) -> Vec<ViolationPrediction> {
    let mut predictions = Vec::new();
    match action {
        RefactorAction::MoveFunction { function, from, to, required_layer } => {
            let callers = find_callers(function, call_graph, elements);
            if !callers.is_empty() {
                predictions.push(ViolationPrediction {
                    violation_type: ViolationType::UnresolvedImport,
                    affected_files: callers,
                    severity: Severity::Critical,
                    confidence: 0.95,
                });
            }
            if let Some(layer) = required_layer {
                if !layer.is_empty() {
                    predictions.push(ViolationPrediction {
                        violation_type: ViolationType::LayerViolation,
                        affected_files: vec![to.clone()],
                        severity: Severity::High,
                        confidence: 1.0,
                    });
                }
            } else if move_violates_invariant(function, from, to, invariants) {
                predictions.push(ViolationPrediction {
                    violation_type: ViolationType::LayerViolation,
                    affected_files: vec![to.clone()],
                    severity: Severity::High,
                    confidence: 0.9,
                });
            }
        }
        RefactorAction::RenameFunction { old_name, new_name, file } => {
            if symbol_exists(new_name, elements) {
                predictions.push(ViolationPrediction {
                    violation_type: ViolationType::NameCollision,
                    affected_files: vec![file.clone()],
                    severity: Severity::Critical,
                    confidence: 1.0,
                });
            }
            let references = find_reference_files(old_name, call_graph, elements);
            if !references.is_empty() {
                predictions.push(ViolationPrediction {
                    violation_type: ViolationType::BrokenReference,
                    affected_files: references,
                    severity: Severity::Critical,
                    confidence: 0.85,
                });
            }
        }
        RefactorAction::RenameFile { from, to } => {
            predictions.push(ViolationPrediction {
                violation_type: ViolationType::BrokenReference,
                affected_files: vec![from.clone(), to.clone()],
                severity: Severity::High,
                confidence: 0.7,
            });
        }
        RefactorAction::CreateFile { path } => {
            predictions.push(ViolationPrediction {
                violation_type: ViolationType::UnresolvedImport,
                affected_files: vec![path.clone()],
                severity: Severity::Low,
                confidence: 0.5,
            });
        }
        RefactorAction::AdjustVisibility { file, .. } => {
            predictions.push(ViolationPrediction {
                violation_type: ViolationType::VisibilityMismatch,
                affected_files: vec![file.clone()],
                severity: Severity::Low,
                confidence: 0.8,
            });
        }
    }
    predictions
}

fn find_callers(
    function: &str,
    call_graph: &HashMap<String, CallGraphNode>,
    elements: &[CodeElement],
) -> Vec<PathBuf> {
    let mut files = HashSet::new();
    if let Some(node) = call_graph.get(function) {
        for caller in &node.called_by {
            if let Some(file) = find_element_file(caller, elements) {
                files.insert(file);
            }
        }
    }
    files.into_iter().collect()
}

fn find_reference_files(
    function: &str,
    call_graph: &HashMap<String, CallGraphNode>,
    elements: &[CodeElement],
) -> Vec<PathBuf> {
    let mut files = HashSet::new();
    for (caller, node) in call_graph {
        if node.calls.iter().any(|c| c == function) {
            if let Some(file) = find_element_file(caller, elements) {
                files.insert(file);
            }
        }
    }
    files.into_iter().collect()
}

fn find_element_file(function: &str, elements: &[CodeElement]) -> Option<PathBuf> {
    elements
        .iter()
        .find(|el| el.name == function)
        .map(|el| PathBuf::from(&el.file_path))
}

fn symbol_exists(symbol: &str, elements: &[CodeElement]) -> bool {
    elements.iter().any(|el| el.name == symbol)
}

fn move_violates_invariant(
    _function: &str,
    _from: &PathBuf,
    _to: &PathBuf,
    _invariants: &InvariantAnalysisResult,
) -> bool {
    false
}

pub fn generate_intelligence_report(
    actions: &[RefactorAction],
    state: &IntelligenceState<'_>,
) -> CorrectionIntelligenceReport {
    let mut plans = Vec::new();
    let mut policies = Vec::new();
    let mut criteria = Vec::new();
    let mut deltas = Vec::new();

    for action in actions {
        let mut predictions =
            predict_violations(action, state.invariants, state.call_graph, state.elements);
        fill_prediction_confidence(&mut predictions);
        let mut plan = generate_correction_plan(action, &predictions);
        augment_path_coherence_strategies(&mut plan, action, &state.root);
        let policy = plan_verification_scope(action, &plan);
        let rollback = build_rollback_criteria(action, &plan);
        let delta = estimate_impact(action, &ImpactState {
            metrics: state.metrics.clone(),
        });

        plans.push(plan);
        policies.push(policy);
        criteria.push(rollback);
        deltas.push(delta);
    }

    let summary = compute_summary(&plans, &deltas);

    CorrectionIntelligenceReport {
        version: "1.0".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        project_root: state.root.clone(),
        actions_analyzed: actions.len(),
        correction_plans: plans,
        verification_policies: policies,
        rollback_criteria: criteria,
        quality_deltas: deltas,
        summary,
    }
}
