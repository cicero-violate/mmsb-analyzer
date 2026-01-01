#![allow(dead_code)]
//! Quality delta calculator.

use crate::correction_plan_types::RefactorAction;
use crate::quality_delta_types::QualityDelta;
use crate::action_impact_estimator::{AnalysisState, simulate_action};

#[derive(Clone, Debug, Default)]
pub struct Metrics {
    pub cohesion: f64,
    pub violations: usize,
    pub complexity: f64,
}

pub fn calculate_quality_delta(
    action: &RefactorAction,
    current: &Metrics,
    simulated: &Metrics,
) -> QualityDelta {
    let cohesion_delta = simulated.cohesion - current.cohesion;
    let violation_delta = simulated.violations as i32 - current.violations as i32;
    let complexity_delta = simulated.complexity - current.complexity;
    let overall = 0.5 * cohesion_delta - 0.3 * violation_delta as f64 - 0.2 * complexity_delta;
    let acceptable = overall > -0.05 && violation_delta <= 0;
    let reason = if acceptable {
        "Quality improved or maintained".to_string()
    } else if overall < -0.1 {
        "Quality degradation exceeds threshold".to_string()
    } else if violation_delta > 0 {
        format!("Introduced {} new violations", violation_delta)
    } else {
        "Quality barely acceptable".to_string()
    };
    QualityDelta {
        action_id: action.action_id(),
        cohesion_delta,
        violation_delta,
        complexity_delta,
        overall_score_delta: overall,
        acceptable,
        reason,
    }
}

pub fn estimate_impact(action: &RefactorAction, current_state: &AnalysisState) -> QualityDelta {
    let simulated = simulate_action(action, current_state);
    calculate_quality_delta(action, &current_state.metrics, &simulated.metrics)
}
