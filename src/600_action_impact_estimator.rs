#![allow(dead_code)]
//! Action impact estimator.

use crate::correction_plan_types::RefactorAction;
use crate::quality_delta_calculator::Metrics;

pub use crate::quality_delta_calculator::estimate_impact;

#[derive(Clone, Debug)]
pub struct AnalysisState {
    pub metrics: Metrics,
}



pub(crate) fn simulate_action(_action: &RefactorAction, state: &AnalysisState) -> AnalysisState {
    state.clone()
}
