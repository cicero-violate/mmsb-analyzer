#![allow(dead_code)]
//! Rollback criteria builder.

use crate::correction_plan_types::{CorrectionPlan, ErrorTier, RefactorAction, ViolationType};
use crate::quality_delta_types::{RollbackCondition, RollbackCriteria};

pub fn build_rollback_criteria(
    action: &RefactorAction,
    correction_plan: &CorrectionPlan,
) -> RollbackCriteria {
    let mut mandatory = vec![RollbackCondition::BuildFailed];
    let mut suggested = vec![RollbackCondition::QualityDecreased { threshold: 0.05 }];

    match correction_plan.tier {
        ErrorTier::Complex => {
            mandatory.push(RollbackCondition::Tier3Error {
                error_type: ViolationType::TypeMismatch,
            });
            mandatory.push(RollbackCondition::ManualReviewRequired);
        }
        ErrorTier::Moderate => {
            suggested.push(RollbackCondition::TestsFailed {
                critical_tests: extract_critical_tests(action),
            });
        }
        ErrorTier::Trivial => {}
    }

    for prediction in &correction_plan.predicted_violations {
        if prediction.violation_type == ViolationType::LayerViolation {
            mandatory.push(RollbackCondition::InvariantViolated {
                invariant_ids: vec!["layer_ordering".to_string()],
            });
        }
    }

    RollbackCriteria {
        action_id: correction_plan.action_id.clone(),
        mandatory_rollback_if: mandatory,
        suggested_rollback_if: suggested,
    }
}

fn extract_critical_tests(_action: &RefactorAction) -> Vec<String> {
    Vec::new()
}
