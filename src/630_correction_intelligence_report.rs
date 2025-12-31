#![allow(dead_code)]
//! Correction intelligence report generator.

use crate::correction_plan_types::{
    CorrectionPlan, CorrectionStrategy, ErrorTier, RefactorAction, ViolationPrediction,
};
use crate::quality_delta_calculator::Metrics;
use crate::quality_delta_types::{QualityDelta, RollbackCriteria};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{collections::HashSet, fs};

use crate::invariant_types::InvariantAnalysisResult;
use crate::types::{AnalysisResult, CallGraphNode, CodeElement};

pub use crate::correction_plan_serializer::write_intelligence_outputs_at;
pub use crate::violation_predictor::generate_intelligence_report;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CorrectionIntelligenceReport {
    pub version: String,
    pub timestamp: String,
    pub project_root: PathBuf,
    pub actions_analyzed: usize,
    pub correction_plans: Vec<CorrectionPlan>,
    pub verification_policies: Vec<crate::verification_policy_types::VerificationPolicy>,
    pub rollback_criteria: Vec<RollbackCriteria>,
    pub quality_deltas: Vec<QualityDelta>,
    pub summary: CorrectionSummary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CorrectionSummary {
    pub trivial_count: usize,
    pub moderate_count: usize,
    pub complex_count: usize,
    pub total_predicted_violations: usize,
    pub average_confidence: f64,
    pub estimated_total_fix_time_seconds: u32,
}

#[derive(Clone, Debug)]
pub struct IntelligenceState<'a> {
    pub root: PathBuf,
    pub invariants: &'a InvariantAnalysisResult,
    pub call_graph: &'a HashMap<String, CallGraphNode>,
    pub elements: &'a [CodeElement],
    pub metrics: Metrics,
}

pub fn build_state<'a>(
    root: &'a Path,
    analysis: &'a AnalysisResult,
    metrics: Metrics,
) -> IntelligenceState<'a> {
    IntelligenceState {
        root: root.to_path_buf(),
        invariants: &analysis.invariants,
        call_graph: &analysis.call_graph,
        elements: &analysis.elements,
        metrics,
    }
}



pub fn write_intelligence_outputs(
    report: &CorrectionIntelligenceReport,
    output_dir: &Path,
) -> std::io::Result<()> {
    write_intelligence_outputs_at(report, output_dir, None, None)
}

pub fn filter_path_coherence_report(
    report: &CorrectionIntelligenceReport,
) -> CorrectionIntelligenceReport {
    let mut plans = Vec::new();
    let mut policies = Vec::new();
    let mut criteria = Vec::new();
    let mut deltas = Vec::new();

    for (idx, plan) in report.correction_plans.iter().enumerate() {
        let mut has_path_coherence = false;
        for strategy in &plan.strategies {
            match strategy {
                CorrectionStrategy::UpdatePath { .. } => {
                    has_path_coherence = true;
                    break;
                }
                CorrectionStrategy::UpdateCaller { old_ref, .. } => {
                    let trimmed = old_ref.trim_start();
                    if trimmed.starts_with("mod ")
                        || trimmed.starts_with("pub mod ")
                        || trimmed.starts_with("use ")
                        || trimmed.starts_with("#[path")
                    {
                        has_path_coherence = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        let is_rename_file = plan.action_id.starts_with("rename_file_");

        if !(has_path_coherence || is_rename_file) {
            continue;
        }

        plans.push(plan.clone());
        if let Some(policy) = report.verification_policies.get(idx) {
            policies.push(policy.clone());
        }
        if let Some(rollback) = report.rollback_criteria.get(idx) {
            criteria.push(rollback.clone());
        }
        if let Some(delta) = report.quality_deltas.get(idx) {
            deltas.push(delta.clone());
        }
    }

    let summary = compute_summary(&plans, &deltas);

    CorrectionIntelligenceReport {
        version: report.version.clone(),
        timestamp: report.timestamp.clone(),
        project_root: report.project_root.clone(),
        actions_analyzed: plans.len(),
        correction_plans: plans,
        verification_policies: policies,
        rollback_criteria: criteria,
        quality_deltas: deltas,
        summary,
    }
}

pub fn filter_visibility_report(
    report: &CorrectionIntelligenceReport,
) -> CorrectionIntelligenceReport {
    let mut plans = Vec::new();
    let mut policies = Vec::new();
    let mut criteria = Vec::new();
    let mut deltas = Vec::new();

    for (idx, plan) in report.correction_plans.iter().enumerate() {
        let mut has_visibility = false;
        for strategy in &plan.strategies {
            match strategy {
                CorrectionStrategy::AdjustVisibility { .. } => {
                    has_visibility = true;
                    break;
                }
                CorrectionStrategy::VisibilityPlan { .. } => {
                    has_visibility = true;
                    break;
                }
                CorrectionStrategy::ManualReview { reason, .. }
                    if reason.starts_with("review:") =>
                {
                    has_visibility = true;
                    break;
                }
                _ => {}
            }
        }
        if !has_visibility {
            continue;
        }
        plans.push(plan.clone());
        if let Some(policy) = report.verification_policies.get(idx) {
            policies.push(policy.clone());
        }
        if let Some(rollback) = report.rollback_criteria.get(idx) {
            criteria.push(rollback.clone());
        }
        if let Some(delta) = report.quality_deltas.get(idx) {
            deltas.push(delta.clone());
        }
    }

    let summary = compute_summary(&plans, &deltas);

    CorrectionIntelligenceReport {
        version: report.version.clone(),
        timestamp: report.timestamp.clone(),
        project_root: report.project_root.clone(),
        actions_analyzed: plans.len(),
        correction_plans: plans,
        verification_policies: policies,
        rollback_criteria: criteria,
        quality_deltas: deltas,
        summary,
    }
}

pub(crate) fn augment_path_coherence_strategies(
    plan: &mut CorrectionPlan,
    action: &RefactorAction,
    root: &Path,
) {
    let RefactorAction::RenameFile { from, to } = action else {
        return;
    };
    let Some(old_mod) = module_name_from_path(from) else {
        return;
    };
    let Some(new_mod) = module_name_from_path(to) else {
        return;
    };
    let old_file_name = from.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let new_file_name = to.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let replace_mod = old_mod != new_mod;

    let mod_re = if replace_mod {
        Regex::new(&format!(
            r"^\s*(pub\s+)?mod\s+{}\s*;",
            regex::escape(&old_mod)
        ))
        .ok()
    } else {
        None
    };
    let use_re = if replace_mod {
        Regex::new(&format!(
            r"^\s*use\s+.*\b{}\b",
            regex::escape(&old_mod)
        ))
        .ok()
    } else {
        None
    };
    let path_re = if !old_file_name.is_empty() && !new_file_name.is_empty() {
        Regex::new(r#"^\s*#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]"#).ok()
    } else {
        None
    };

    let mut updates = Vec::new();
    let mut seen = HashSet::new();
    let rust_files = crate::cluster_010::gather_rust_files(root);

    for file in rust_files {
        let Ok(contents) = fs::read_to_string(&file) else {
            continue;
        };
        for line in contents.lines() {
            if let Some(re) = &mod_re {
                if re.is_match(line) {
                    let new_line = line.replace(&old_mod, &new_mod);
                    if new_line != line {
                        let key = (file.clone(), line.to_string(), new_line.clone());
                        if seen.insert(key.clone()) {
                            updates.push(key);
                        }
                    }
                    continue;
                }
            }
            if let Some(re) = &use_re {
                if re.is_match(line) {
                    let new_line = line.replace(&old_mod, &new_mod);
                    if new_line != line {
                        let key = (file.clone(), line.to_string(), new_line.clone());
                        if seen.insert(key.clone()) {
                            updates.push(key);
                        }
                    }
                    continue;
                }
            }
            if let Some(re) = &path_re {
                if re.is_match(line) && line.contains(old_file_name) {
                    let new_line = line.replace(old_file_name, new_file_name);
                    if new_line != line {
                        let key = (file.clone(), line.to_string(), new_line.clone());
                        if seen.insert(key.clone()) {
                            updates.push(key);
                        }
                    }
                }
            }
        }
    }

    updates.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| a.1.cmp(&b.1))
            .then_with(|| a.2.cmp(&b.2))
    });

    for (file, old_ref, new_ref) in updates {
        plan.strategies.push(CorrectionStrategy::UpdateCaller {
            caller_file: file,
            old_ref,
            new_ref,
        });
    }
}

fn module_name_from_path(path: &Path) -> Option<String> {
    let stem = path.file_stem().and_then(|s| s.to_str())?;
    let name = if stem == "mod" {
        path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())?
            .to_string()
    } else {
        stem.to_string()
    };
    Some(crate::cluster_010::normalize_module_name(&name))
}





pub(crate) fn compute_summary(plans: &[CorrectionPlan], deltas: &[QualityDelta]) -> CorrectionSummary {
    let mut trivial = 0;
    let mut moderate = 0;
    let mut complex = 0;
    let mut total_violations = 0;
    let mut total_confidence = 0.0;
    let mut total_time = 0;

    for plan in plans {
        match plan.tier {
            ErrorTier::Trivial => trivial += 1,
            ErrorTier::Moderate => moderate += 1,
            ErrorTier::Complex => complex += 1,
        }
        total_violations += plan.predicted_violations.len();
        total_confidence += plan.confidence;
        total_time += plan.estimated_fix_time_seconds;
    }

    let avg_conf = if plans.is_empty() {
        0.0
    } else {
        total_confidence / plans.len() as f64
    };

    let _ = deltas;

    CorrectionSummary {
        trivial_count: trivial,
        moderate_count: moderate,
        complex_count: complex,
        total_predicted_violations: total_violations,
        average_confidence: avg_conf,
        estimated_total_fix_time_seconds: total_time,
    }
}

pub(crate) fn fill_prediction_confidence(predictions: &mut [ViolationPrediction]) {
    for prediction in predictions {
        if prediction.confidence <= 0.0 {
            prediction.confidence = default_confidence(&prediction.violation_type);
        }
    }
}

fn default_confidence(violation_type: &crate::correction_plan_types::ViolationType) -> f64 {
    match violation_type {
        crate::correction_plan_types::ViolationType::UnresolvedImport => 0.95,
        crate::correction_plan_types::ViolationType::NameCollision => 1.0,
        crate::correction_plan_types::ViolationType::LayerViolation => 0.9,
        crate::correction_plan_types::ViolationType::VisibilityMismatch => 0.8,
        crate::correction_plan_types::ViolationType::BrokenReference => 0.85,
        crate::correction_plan_types::ViolationType::TypeMismatch => 0.6,
        crate::correction_plan_types::ViolationType::OwnershipIssue => 0.5,
    }
}
