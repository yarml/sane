use std::collections::HashMap;

use crate::{ast::Instruction, predictor::Predictor};

#[derive(Debug)]
pub struct HartState {
  pc: usize,
  rf: RegisterFile,
}

impl HartState {
  pub fn new() -> HartState {
    HartState {
      pc: 0,
      rf: RegisterFile::new(),
    }
  }
  pub fn pc(&self) -> usize {
    self.pc
  }
  pub fn next(&mut self) {
    self.pc += 1;
  }
  pub fn goto(&mut self, pc: usize) {
    self.pc = pc;
  }
  pub fn reg_read(&self, index: usize) -> u64 {
    self.rf.read(index)
  }
  pub fn reg_write(&mut self, index: usize, value: u64) {
    self.rf.write(index, value);
  }
}

#[derive(Debug)]
pub struct RegisterFile {
  file: Vec<u64>,
}

impl RegisterFile {
  pub fn new() -> RegisterFile {
    RegisterFile { file: vec![0; 32] }
  }

  pub fn read(&self, index: usize) -> u64 {
    if index == 0 {
      0
    } else {
      self.file[index]
    }
  }

  pub fn write(&mut self, index: usize, value: u64) {
    if index == 0 {
      return;
    }
    self.file[index] = value;
  }
}

#[derive(Debug)]
pub struct PredictorResults {
  pub correct: usize,
  pub total: usize,
}

pub fn measure(
  instructions: &[Instruction],
  predictors: &mut [Box<dyn Predictor>],
) -> (
  HartState,
  HashMap<&'static str, HashMap<usize, PredictorResults>>,
) {
  let mut state = HartState::new();
  let mut predictors_accuracy = HashMap::new();

  for predictor in predictors.iter() {
    predictors_accuracy.insert(predictor.id(), HashMap::new());
  }

  loop {
    println!("{}", state.pc());
    let ins = &instructions[state.pc()];

    // Execute instruction
    match ins {
      Instruction::Add(rd, rs1, rs2) => {
        let result = state
          .reg_read(*rs1 as usize)
          .wrapping_add(state.reg_read(*rs2 as usize));
        state.reg_write(*rd as usize, result);
      }
      Instruction::Sub(rd, rs1, rs2) => {
        let result = state
          .reg_read(*rs1 as usize)
          .wrapping_sub(state.reg_read(*rs2 as usize));
        state.reg_write(*rd as usize, result);
      }
      Instruction::AddImm(rd, rs1, imm) => {
        let result = state.reg_read(*rs1 as usize).wrapping_add_signed(*imm);
        state.reg_write(*rd as usize, result);
      }
      Instruction::BranchGE(_, _, _) => {}
      Instruction::BranchNE(_, _, _) => {}
      Instruction::Jump(label) => {
        let new_loc = search_label(instructions, label);
        if let Some(loc) = new_loc {
          state.goto(loc);
        } else {
          panic!("Label not found");
        }
      }
      Instruction::Label(_) => {}
      Instruction::Finish => {
        return (state, predictors_accuracy);
      }
    }

    // Update Predictors, and do jumps
    let branch = match ins {
      Instruction::BranchGE(rs1, rs2, label) => {
        let new_loc =
          search_label(instructions, label).expect("Label not found");
        Some((
          state.reg_read(*rs1 as usize) >= state.reg_read(*rs2 as usize),
          new_loc,
        ))
      }
      Instruction::BranchNE(rs1, rs2, label) => {
        let new_loc =
          search_label(instructions, label).expect("Label not found");
        Some((
          state.reg_read(*rs1 as usize) != state.reg_read(*rs2 as usize),
          new_loc,
        ))
      }
      _ => None,
    };

    if let Some((taken, new_loc)) = branch {
      let current_loc = state.pc();
      for predictor in predictors.iter_mut() {
        let prediction_correct =
          predictor.prediction_matches_actuality(&state, new_loc, taken);
        let predictor_results =
          predictors_accuracy.get_mut(predictor.id()).unwrap();
        let results =
          predictor_results
            .entry(current_loc)
            .or_insert(PredictorResults {
              correct: 0,
              total: 0,
            });
        results.total += 1;
        if prediction_correct {
          results.correct += 1;
        }
      }
      if taken {
        state.goto(new_loc);
      } else {
        state.next();
      }
    } else {
      state.next();
    }
  }
}

fn search_label(instructions: &[Instruction], label: &str) -> Option<usize> {
  for (loc, instruction) in instructions.iter().enumerate() {
    if let Instruction::Label(l) = instruction {
      if l == label {
        return Some(loc);
      }
    }
  }
  None
}
