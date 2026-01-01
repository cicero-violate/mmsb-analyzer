#![allow(dead_code)]
//! Verification scope planner.

use crate::correction_plan_types::{CorrectionPlan, ErrorTier, RefactorAction};
use crate::verification_policy_types::{
    VerificationCheck, VerificationPolicy, VerificationScope,
};

pub fn plan_verification_scope(
    action: &RefactorAction,
    correction_plan: &CorrectionPlan,
) -> VerificationPolicy {
    let scope = match correction_plan.tier {
        ErrorTier::Trivial if correction_plan.predicted_violations.len() <= 3 => {
            VerificationScope::SyntaxOnly {
                files: affected_files(action),
            }
        }
        ErrorTier::Trivial | ErrorTier::Moderate => VerificationScope::ModuleLocal {
            module: action_module(action),
            transitive_depth: 2,
        },
        ErrorTier::Complex => VerificationScope::FullWorkspace,
    };

    let mut required_checks = vec![VerificationCheck::CargoCheck];
    if matches!(correction_plan.tier, ErrorTier::Moderate | ErrorTier::Complex) {
        required_checks.push(VerificationCheck::CargoTest { filter: None });
    }

    VerificationPolicy {
        action_id: correction_plan.action_id.clone(),
        scope,
        required_checks,
        incremental_eligible: matches!(correction_plan.tier, ErrorTier::Trivial),
        estimated_time_seconds: estimate_verification_time(&correction_plan.tier),
    }
}

fn affected_files(action: &RefactorAction) -> Vec<std::path::PathBuf> {
    match action {
        RefactorAction::MoveFunction { from, to, .. } => vec![from.clone(), to.clone()],
        RefactorAction::RenameFunction { file, .. } => vec![file.clone()],
        RefactorAction::RenameFile { from, to } => vec![from.clone(), to.clone()],
        RefactorAction::CreateFile { path } => vec![path.clone()],
        RefactorAction::AdjustVisibility { file, .. } => vec![file.clone()],
    }
}

fn action_module(action: &RefactorAction) -> String {
    match action {
        RefactorAction::MoveFunction { to, .. } => to.display().to_string(),
        RefactorAction::RenameFunction { file, .. } => file.display().to_string(),
        RefactorAction::RenameFile { to, .. } => to.display().to_string(),
        RefactorAction::CreateFile { path } => path.display().to_string(),
        RefactorAction::AdjustVisibility { file, .. } => file.display().to_string(),
    }
}

fn estimate_verification_time(tier: &ErrorTier) -> u32 {
    match tier {
        ErrorTier::Trivial => 10,
        ErrorTier::Moderate => 60,
        ErrorTier::Complex => 180,
    }
}
