use crate::machine::HartState;

use super::Predictor;

pub struct AlwaysTaken;
pub struct NeverTaken;
pub struct ForwardTaken;
pub struct BackwardTaken;

impl Predictor for AlwaysTaken {
  fn prediction_matches_actuality(
    &mut self,
    _: &HartState,
    _: usize,
    actuality: bool,
  ) -> bool {
    actuality
  }

  fn id(&self) -> &'static str {
    "always-taken"
  }
}

impl Predictor for NeverTaken {
  fn prediction_matches_actuality(
    &mut self,
    _: &HartState,
    _: usize,
    actuality: bool,
  ) -> bool {
    !actuality
  }

  fn id(&self) -> &'static str {
    "never-taken"
  }
}

impl Predictor for ForwardTaken {
  fn prediction_matches_actuality(
    &mut self,
    state: &HartState,
    branch_target: usize,
    actuality: bool,
  ) -> bool {
    let pc = state.pc();
    let prediction = branch_target > pc;
    prediction == actuality
  }

  fn id(&self) -> &'static str {
    "forward-taken"
  }
}

impl Predictor for BackwardTaken {
  fn prediction_matches_actuality(
    &mut self,
    state: &HartState,
    branch_target: usize,
    actuality: bool,
  ) -> bool {
    let pc = state.pc();
    let prediction = branch_target < pc;
    prediction == actuality
  }

  fn id(&self) -> &'static str {
    "backward-taken"
  }
}
