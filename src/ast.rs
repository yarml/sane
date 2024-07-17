pub type RegisterNumber = usize;
pub type Immediate = i64;
pub type Label = String;

#[derive(Debug)]
pub enum Instruction {
  Add(RegisterNumber, RegisterNumber, RegisterNumber),
  Sub(RegisterNumber, RegisterNumber, RegisterNumber),
  AddImm(RegisterNumber, RegisterNumber, Immediate),
  Jump(Label),
  BranchGE(RegisterNumber, RegisterNumber, Label),
  BranchNE(RegisterNumber, RegisterNumber, Label),
  Finish,
  Label(Label),
}

#[derive(Debug)]
enum Argument {
  Register(RegisterNumber),
  Immediate(Immediate),
  Label(Label),
}

impl From<&str> for Argument {
  fn from(arg: &str) -> Self {
    if arg.starts_with('x') {
      Argument::Register(arg[1..].parse().unwrap())
    } else {
      match arg.parse() {
        Ok(imm) => Argument::Immediate(imm),
        Err(_) => Argument::Label(arg.to_string()),
      }
    }
  }
}

impl From<(&str, &[Argument])> for Instruction {
  fn from((opcode, args): (&str, &[Argument])) -> Self {
    match (opcode, &args) {
      (
        "add",
        [Argument::Register(reg1), Argument::Register(reg2), Argument::Register(reg3)],
      ) => Instruction::Add(*reg1, *reg2, *reg3),
      (
        "sub",
        [Argument::Register(reg1), Argument::Register(reg2), Argument::Register(reg3)],
      ) => Instruction::Sub(*reg1, *reg2, *reg3),
      (
        "addi",
        [Argument::Register(reg1), Argument::Register(reg2), Argument::Immediate(imm)],
      ) => Instruction::AddImm(*reg1, *reg2, *imm),
      (
        "bge",
        [Argument::Register(reg1), Argument::Register(reg2), Argument::Label(label)],
      ) => Instruction::BranchGE(*reg1, *reg2, label.clone()),
      (
        "bne",
        [Argument::Register(reg1), Argument::Register(reg2), Argument::Label(label)],
      ) => Instruction::BranchNE(*reg1, *reg2, label.clone()),
      ("j", &[Argument::Label(label)]) => Instruction::Jump(label.clone()),
      ("finish", []) => Instruction::Finish,
      _ => panic!("Invalid combination of opcode and arguments"),
    }
  }
}

pub fn parse(content: &str) -> Vec<Instruction> {
  content
    .split('\n')
    .filter_map(|line| {
      let line = line.trim();
      if line.is_empty() {
        None
      } else if line.ends_with(':') {
        Some(Instruction::Label(line[..line.len() - 1].to_string()))
      } else {
        let opcode = line.split(' ').nth(0).unwrap();
        let args = match line.split_once(' ') {
          None => vec![],
          Some((_, args)) => args
            .replace(' ', "")
            .split(',')
            .map(|arg| Argument::from(arg))
            .collect(),
        };
        Some(Instruction::from((opcode, args.as_slice())))
      }
    })
    .collect()
}
