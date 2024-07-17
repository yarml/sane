mod ast;
mod machine;
mod predictor;

use ast::parse;
use machine::measure;
use predictor::{
  dynamic::{StateMachine1Bit, StateMachine2Bit},
  simple::{AlwaysTaken, BackwardTaken, ForwardTaken, NeverTaken},
  Predictor,
};
use std::fs;

fn main() {
  let mut predictors: Vec<Box<dyn Predictor>> = vec![
    Box::new(AlwaysTaken),
    Box::new(NeverTaken),
    Box::new(ForwardTaken),
    Box::new(BackwardTaken),
    Box::new(StateMachine1Bit::new()),
    Box::new(StateMachine2Bit::new()),
  ];

  let content = fs::read_to_string("input.asm").unwrap();
  let instructions = parse(&content);
  let (final_state, predictors) = measure(&instructions, &mut predictors);
  println!("{}", content);
  println!("State after execution: {:?}", final_state);
  for (name, results) in predictors {
    println!("{}", name);
    for (pc, result) in results {
      println!("  {}: {}/{}", pc, result.correct, result.total);
    }
  }
}
