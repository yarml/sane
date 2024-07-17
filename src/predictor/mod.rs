pub mod simple;
pub mod dynamic;

use crate::machine::HartState;

pub trait Predictor {
  /// Called when a branch is found.
  /// `actuality` is whether the branch was taken in reality.
  /// `branch_target` is the target address of the branch.
  /// Returns whether the prediction was correct.
  fn prediction_matches_actuality(
    &mut self,
    state: &HartState,
    branch_target: usize,
    actuality: bool,
  ) -> bool;

  fn id(&self) -> &'static str;
}
