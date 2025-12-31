#![allow(dead_code)]
//! Generate correction plans from predictions.

use crate::correction_plan_types::{
    CorrectionPlan, CorrectionStrategy, ErrorTier, RefactorAction, ViolationPrediction,
    ViolationType, VisibilityPlanOption,
};
use crate::tier_classifier::classify_tier;

pub fn generate_correction_plan(
    action: &RefactorAction,
    predictions: &[ViolationPrediction],
) -> CorrectionPlan {
    let mut strategies = Vec::new();
    for prediction in predictions {
        match prediction.violation_type {
            ViolationType::UnresolvedImport => {
                if let Some(symbol) = action_symbol(action) {
                    strategies.push(CorrectionStrategy::AddImport {
                        module_path: action_module_path(action),
                        symbol,
                    });
                }
            }
            ViolationType::BrokenReference => {
                match action {
                    RefactorAction::RenameFile { .. } => {
                        if let Some((old_ref, new_ref)) = action_refs(action) {
                            strategies.push(CorrectionStrategy::UpdatePath {
                                old_path: old_ref,
                                new_path: new_ref,
                            });
                        }
                    }
                    _ => {
                        if let Some((old_ref, new_ref)) = action_refs(action) {
                            for file in &prediction.affected_files {
                                strategies.push(CorrectionStrategy::UpdateCaller {
                                    caller_file: file.clone(),
                                    old_ref: old_ref.clone(),
                                    new_ref: new_ref.clone(),
                                });
                            }
                        }
                    }
                }
            }
            ViolationType::NameCollision => {
                if let Some(symbol) = action_symbol(action) {
                    strategies.push(CorrectionStrategy::RenameWithSuffix {
                        original: symbol,
                        suffix: "_v2".to_string(),
                    });
                }
            }
            ViolationType::LayerViolation => {
                if let Some(layer) = action_target_layer(action) {
                    if let Some(function) = action_function(action) {
                        strategies.push(CorrectionStrategy::MoveToLayer {
                            function,
                            target_layer: layer,
                        });
                        if let Some(function) = action_function(action) {
                            if let Some(layer) = action_target_layer(action) {
                                strategies.push(CorrectionStrategy::EnsureImports {
                                    function,
                                    target_layer: layer,
                                });
                            }
                        }
                    }
                }
            }
            ViolationType::VisibilityMismatch => {
                if let Some((symbol, file, from, to, reason)) = action_visibility(action) {
                    if from == to || reason.starts_with("review:") {
                        let options = vec![
                            VisibilityPlanOption {
                                policy: "keep_public".to_string(),
                                target: crate::types::Visibility::Public,
                                requires_consent: false,
                                description: "Keep public (treat as external API).".to_string(),
                            },
                            VisibilityPlanOption {
                                policy: "downgrade_pub_crate".to_string(),
                                target: crate::types::Visibility::Crate,
                                requires_consent: true,
                                description:
                                    "Narrow to pub(crate) (internal API only).".to_string(),
                            },
                            VisibilityPlanOption {
                                policy: "downgrade_private".to_string(),
                                target: crate::types::Visibility::Private,
                                requires_consent: true,
                                description: "Narrow to private (file-local).".to_string(),
                            },
                        ];
                        strategies.push(CorrectionStrategy::VisibilityPlan {
                            symbol,
                            file,
                            current: from,
                            default_policy: "review_only".to_string(),
                            options,
                            notes: reason,
                        });
                    } else {
                        strategies.push(CorrectionStrategy::AdjustVisibility {
                            symbol,
                            file,
                            from,
                            to,
                            reason,
                        });
                    }
                }
            }
            ViolationType::TypeMismatch | ViolationType::OwnershipIssue => {
                strategies.push(CorrectionStrategy::ManualReview {
                    reason: format!("{:?} requires semantic analysis", prediction.violation_type),
                    context: format!("{:?}", action),
                });
            }
        }
    }

    let tier = predictions
        .iter()
        .map(classify_tier)
        .max()
        .unwrap_or(ErrorTier::Trivial);

    CorrectionPlan {
        action_id: action.action_id(),
        tier,
        predicted_violations: predictions.to_vec(),
        strategies,
        confidence: average_confidence(predictions),
        estimated_fix_time_seconds: estimate_fix_time(predictions.len()),
    }
}

fn average_confidence(predictions: &[ViolationPrediction]) -> f64 {
    if predictions.is_empty() {
        return 1.0;
    }
    let total: f64 = predictions.iter().map(|p| p.confidence).sum();
    total / predictions.len() as f64
}

fn estimate_fix_time(count: usize) -> u32 {
    10 + (count as u32 * 5)
}

fn action_symbol(action: &RefactorAction) -> Option<String> {
    match action {
        RefactorAction::MoveFunction { function, .. } => Some(function.clone()),
        RefactorAction::RenameFunction { new_name, .. } => Some(new_name.clone()),
        RefactorAction::AdjustVisibility { symbol, .. } => Some(symbol.clone()),
        _ => None,
    }
}

fn action_function(action: &RefactorAction) -> Option<String> {
    match action {
        RefactorAction::MoveFunction { function, .. } => Some(function.clone()),
        _ => None,
    }
}

fn action_module_path(action: &RefactorAction) -> String {
    match action {
        RefactorAction::MoveFunction { to, .. } => to.display().to_string(),
        RefactorAction::RenameFile { to, .. } => to.display().to_string(),
        RefactorAction::CreateFile { path } => path.display().to_string(),
        RefactorAction::AdjustVisibility { file, .. } => file.display().to_string(),
        _ => "crate".to_string(),
    }
}

fn action_refs(action: &RefactorAction) -> Option<(String, String)> {
    match action {
        RefactorAction::RenameFunction { old_name, new_name, .. } => {
            Some((old_name.clone(), new_name.clone()))
        }
        RefactorAction::RenameFile { from, to } => {
            Some((from.display().to_string(), to.display().to_string()))
        }
        _ => None,
    }
}

fn action_target_layer(action: &RefactorAction) -> Option<String> {
    match action {
        RefactorAction::MoveFunction { required_layer, .. } => required_layer.clone(),
        _ => None,
    }
}

fn action_visibility(
    action: &RefactorAction,
) -> Option<(
    String,
    std::path::PathBuf,
    crate::types::Visibility,
    crate::types::Visibility,
    String,
)> {
    match action {
        RefactorAction::AdjustVisibility {
            symbol,
            file,
            from,
            to,
            reason,
        } => Some((symbol.clone(), file.clone(), from.clone(), to.clone(), reason.clone())),
        _ => None,
    }
}
