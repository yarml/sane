use std::collections::HashMap;

use crate::machine::HartState;

use super::Predictor;

enum State1Bit {
  NotTaken,
  Taken,
}
enum State2Bit {
  StronglyNotTaken,
  WeaklyNotTaken,
  WeaklyTaken,
  StronglyTaken,
}

impl State1Bit {
  fn prediction(&self) -> bool {
    match self {
      State1Bit::NotTaken => false,
      State1Bit::Taken => true,
    }
  }
  fn update(&mut self, actuality: bool) {
    if actuality {
      *self = State1Bit::Taken;
    } else {
      *self = State1Bit::NotTaken;
    }
  }
}

impl State2Bit {
  fn prediction(&self) -> bool {
    match self {
      State2Bit::StronglyNotTaken | State2Bit::WeaklyNotTaken => false,
      State2Bit::WeaklyTaken | State2Bit::StronglyTaken => true,
    }
  }
  fn update(&mut self, actuality: bool) {
    let next_state = match self {
      State2Bit::StronglyNotTaken => {
        if actuality {
          State2Bit::WeaklyNotTaken
        } else {
          State2Bit::StronglyNotTaken
        }
      }
      State2Bit::WeaklyNotTaken => {
        if actuality {
          State2Bit::WeaklyTaken
        } else {
          State2Bit::StronglyNotTaken
        }
      }
      State2Bit::WeaklyTaken => {
        if !actuality {
          State2Bit::WeaklyNotTaken
        } else {
          State2Bit::StronglyTaken
        }
      }
      State2Bit::StronglyTaken => {
        if !actuality {
          State2Bit::WeaklyTaken
        } else {
          State2Bit::StronglyTaken
        }
      }
    };
    *self = next_state;
  }
}

pub struct StateMachine1Bit {
  state: HashMap<usize, State1Bit>,
}

impl StateMachine1Bit {
  pub fn new() -> StateMachine1Bit {
    StateMachine1Bit {
      state: HashMap::new(),
    }
  }
}

impl Predictor for StateMachine1Bit {
  fn prediction_matches_actuality(
    &mut self,
    hart: &HartState,
    _: usize,
    actuality: bool,
  ) -> bool {
    let state_machine =
      self.state.entry(hart.pc()).or_insert(State1Bit::NotTaken);
    let correct = state_machine.prediction() == actuality;
    state_machine.update(actuality);
    correct
  }

  fn id(&self) -> &'static str {
    "dyn-1-bit"
  }
}

pub struct StateMachine2Bit {
  state: HashMap<usize, State2Bit>,
}
impl StateMachine2Bit {
  pub fn new() -> StateMachine2Bit {
    StateMachine2Bit {
      state: HashMap::new(),
    }
  }
}
impl Predictor for StateMachine2Bit {
  fn prediction_matches_actuality(
    &mut self,
    hart: &HartState,
    _: usize,
    actuality: bool,
  ) -> bool {
    let state_machine = self
      .state
      .entry(hart.pc())
      .or_insert(State2Bit::StronglyNotTaken);
    let correct = state_machine.prediction() == actuality;
    state_machine.update(actuality);
    correct
  }

  fn id(&self) -> &'static str {
    "dyn-2-bit"
  }
}
