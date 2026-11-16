// ।। ॐ नमः शिवाय ।। \\
// @date_start 7th November, 2026

#![allow(warnings)]

use crate::cpu::CPU;

pub enum Operation {
  Add,
  Sub,
  Mul,
  Div,
  Rem,
}

pub enum Condition {
  Eq,  // equal
  Ne,  // not equal
  Gt,  // greater than
  Lt,  // less than
  Ge,  // greater than or equal
  Le,  // less than or equal
}

pub enum ISACommand {
  // -- MOV Instructions -- \\
  Xmovi { rd: usize, imm: u64 },
  Vmovi { rd: usize, imm: f64 },
  Xmovr { rd: usize, rn: usize },
  Vmovr { rd: usize, rn: usize },
  Xmovv { rd: usize, rn: usize },
  Vmovx { rd: usize, rn: usize },

  // -- Arithmetic Instructions -- \\
  XCalci { rd: usize, rn: usize, imm: u64, op: Operation },
  VCalci { rd: usize, rn: usize, imm: f64, op: Operation },
  XCalcr { rd: usize, rn: usize, rm: usize, op: Operation },
  VCalcr { rd: usize, rn: usize, rm: usize, op: Operation },

  // -- Branching Instructions -- \\
  B { ta: usize },
  Bxi { ta: usize, rn: usize, imm: u64, cd: Condition },
  Bvi { ta: usize, rn: usize, imm: f64, cd: Condition },
  Bxr { ta: usize, rn: usize, rm: usize, cd: Condition },
  Bvr { ta: usize, rn: usize, rm: usize, cd: Condition },
  Bl { ta: usize },
  Ret,

  // -- Stack Pointer Instructions -- \\
  Addsp { imm: usize },
  Subsp { imm: usize },
  
  // -- Load Instructions -- \\
  Ldrx { rd: usize, imm: usize },
  Ldrv { rd: usize, imm: usize },
  Ldpx { rd1: usize, rd2: usize, imm: usize },
  Ldpv { rd1: usize, rd2: usize, imm: usize },

  // -- Store Instructions -- \\
  Strx { rt: usize, imm: usize },
  Strv { rt: usize, imm: usize },
  Stpx { rt1: usize, rt2: usize, imm: usize },
  Stpv { rt1: usize, rt2: usize, imm: usize },

  // -- Exit -- \\
  Exit,
}

impl ISACommand {
  #[inline(always)]
  pub fn execute(&self, cpu: &mut CPU) {
    match self {
      // -- MOVs -- \\
      ISACommand::Xmovi { rd, imm } => {
        cpu.registers.x.write(*rd, *imm);
      },
      ISACommand::Vmovi { rd, imm } => {
        cpu.registers.v.write(*rd, *imm);
      },
      ISACommand::Xmovr { rd, rn } => {
        cpu.registers.x.write(*rd, cpu.registers.x.read(*rn));
      },
      ISACommand::Vmovr { rd, rn } => {
        cpu.registers.v.write(*rd, cpu.registers.v.read(*rn));
      },
      ISACommand::Xmovv { rd, rn } => {
        cpu.registers.x.write(*rd, cpu.registers.v.read(*rn) as u64);
      },
      ISACommand::Vmovx { rd, rn } => {
        cpu.registers.v.write(*rd, cpu.registers.x.read(*rn) as f64);
      },

      // -- CALCs -- \\
      ISACommand::XCalci { rd, rn, imm, op } => {
        match op {
          Operation::Add => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_add(*imm));
          },
          Operation::Sub => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_sub(*imm));
          },
          Operation::Mul => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_mul(*imm));
          },
          Operation::Div => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_div(*imm));
          },
          Operation::Rem => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_rem(*imm));
          },
        }
      },
      ISACommand::VCalci { rd, rn, imm, op } => {
        match op {
          Operation::Add => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) + *imm);
          },
          Operation::Sub => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) - *imm);
          },
          Operation::Mul => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) * *imm);
          },
          Operation::Div => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) / *imm);
          },
          Operation::Rem => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) % *imm);
          },
        }
      },
      ISACommand::XCalcr { rd, rn, rm, op } => {
        match op {
          Operation::Add => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_add(cpu.registers.x.read(*rm)));
          },
          Operation::Sub => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_sub(cpu.registers.x.read(*rm)));
          },
          Operation::Mul => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_mul(cpu.registers.x.read(*rm)));
          },
          Operation::Div => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_div(cpu.registers.x.read(*rm)));
          },
          Operation::Rem => {
            cpu.registers.x.write(*rd, cpu.registers.x.read(*rn).wrapping_rem(cpu.registers.x.read(*rm)));
          },
        }
      },
      ISACommand::VCalcr { rd, rn, rm, op } => {
        match op {
          Operation::Add => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) + cpu.registers.v.read(*rm));
          },
          Operation::Sub => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) - cpu.registers.v.read(*rm));
          },
          Operation::Mul => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) * cpu.registers.v.read(*rm));
          },
          Operation::Div => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) / cpu.registers.v.read(*rm));
          },
          Operation::Rem => {
            cpu.registers.v.write(*rd, cpu.registers.v.read(*rn) % cpu.registers.v.read(*rm));
          },
        }
      },

      // -- BRANCHs -- \\
      ISACommand::B { ta } => {
        cpu.pc = *ta;
      },
      ISACommand::Bxi { ta, rn, imm, cd } => {
        match cd {
          Condition::Eq => {
            if cpu.registers.x.read(*rn) == *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Ne => {
            if cpu.registers.x.read(*rn) != *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Gt => {
            if cpu.registers.x.read(*rn) > *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Lt => {
            if cpu.registers.x.read(*rn) < *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Ge => {
            if cpu.registers.x.read(*rn) >= *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Le => {
            if cpu.registers.x.read(*rn) <= *imm {
              cpu.pc = *ta;
            }
          },
        }
      },
      ISACommand::Bvi { ta, rn, imm, cd } => {
        match cd {
          Condition::Eq => {
            if cpu.registers.v.read(*rn) == *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Ne => {
            if cpu.registers.v.read(*rn) != *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Gt => {
            if cpu.registers.v.read(*rn) > *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Lt => {
            if cpu.registers.v.read(*rn) < *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Ge => {
            if cpu.registers.v.read(*rn) >= *imm {
              cpu.pc = *ta;
            }
          },
          Condition::Le => {
            if cpu.registers.v.read(*rn) <= *imm {
              cpu.pc = *ta;
            }
          },
        }
      },
      ISACommand::Bxr { ta, rn, rm, cd } => {
        match cd {
          Condition::Eq => {
            if cpu.registers.x.read(*rn) == cpu.registers.x.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Ne => {
            if cpu.registers.x.read(*rn) != cpu.registers.x.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Gt => {
            if cpu.registers.x.read(*rn) > cpu.registers.x.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Lt => {
            if cpu.registers.x.read(*rn) < cpu.registers.x.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Ge => {
            if cpu.registers.x.read(*rn) >= cpu.registers.x.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Le => {
            if cpu.registers.x.read(*rn) <= cpu.registers.x.read(*rm) {
              cpu.pc = *ta;
            }
          },
        }
      },
      ISACommand::Bvr { ta, rn, rm, cd } => {
        match cd {
          Condition::Eq => {
            if cpu.registers.v.read(*rn) == cpu.registers.v.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Ne => {
            if cpu.registers.v.read(*rn) != cpu.registers.v.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Gt => {
            if cpu.registers.v.read(*rn) > cpu.registers.v.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Lt => {
            if cpu.registers.v.read(*rn) < cpu.registers.v.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Ge => {
            if cpu.registers.v.read(*rn) >= cpu.registers.v.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::Le => {
            if cpu.registers.v.read(*rn) <= cpu.registers.v.read(*rm) {
              cpu.pc = *ta;
            }
          },
        }
      },
      ISACommand::Bl { ta } => {
        cpu.registers.x.write(30, (cpu.pc + 1) as u64);
        cpu.pc = *ta;
      },
      ISACommand::Ret => {
        cpu.pc = cpu.registers.x.read(30) as usize;
      },

      // -- Stack Pointer -- \\
      ISACommand::Addsp { imm } => {
        cpu.registers.sp += *imm;
      },
      ISACommand::Subsp { imm } => {
        cpu.registers.sp -= *imm;
      },

      // -- LOADs -- \\
      ISACommand::Ldrx { rd, imm } => {
        let base = cpu.registers.sp + *imm;
        if base > cpu.memory.stack.len() {
          panic!("InterpreterError::StackError: Stack overflow caught at `ldrx`! Memory size: {}, called stack :{}.", cpu.memory.stack.len(), base);
        } else {
          let val = u64::from_le_bytes(cpu.memory.stack[base..base + 8].try_into().expect("InterpreterError::SlicingError: Expected array slice of length 8!"));
          cpu.registers.x.write(*rd, val);
        }
      },
      ISACommand::Ldrv { rd, imm } => {
        let base = cpu.registers.sp + *imm;
        if base > cpu.memory.stack.len() {
          panic!("InterpreterError::StackError: Stack overflow caught at `ldrv`! Memory size: {}, called stack :{}.", cpu.memory.stack.len(), base);
        } else {
          let val = f64::from_le_bytes(cpu.memory.stack[base..base + 8].try_into().expect("InterpreterError::SlicingError: Expected array slice of length 8!"));
          cpu.registers.v.write(*rd, val);
        }
      },
      ISACommand::Ldpx { rd1, rd2, imm } => {
        let base = cpu.registers.sp + *imm;
        if base + 16 > cpu.memory.stack.len() {
          panic!("InterpreterError::StackError: Stack overflow caught at `ldpx`! Memory size: {}, called stack :{}.", cpu.memory.stack.len(), base);
        } else {
          let val1 = u64::from_le_bytes(cpu.memory.stack[base..base + 8].try_into().expect("InterpreterError::SlicingError: Expected array slice of length 8!"));
          let val2 = u64::from_le_bytes(cpu.memory.stack[(base + 8)..base + 16].try_into().expect("InterpreterError::SlicingError: Expected array slice of length 8!"));
          cpu.registers.x.write(*rd1, val1);
          cpu.registers.x.write(*rd2, val2);
        }
      },
      ISACommand::Ldpv { rd1, rd2, imm } => {
        let base = cpu.registers.sp + *imm;
        if base + 16 > cpu.memory.stack.len() {
          panic!("InterpreterError::StackError: Stack overflow caught at `ldpv`! Memory size: {}, called stack :{}.", cpu.memory.stack.len(), base);
        } else {
          let val1 = f64::from_le_bytes(cpu.memory.stack[base..base + 8].try_into().expect("InterpreterError::SlicingError: Expected array slice of length 8!"));
          let val2 = f64::from_le_bytes(cpu.memory.stack[(base + 8)..base + 16].try_into().expect("InterpreterError::SlicingError: Expected array slice of length 8!"));
          cpu.registers.v.write(*rd1, val1);
          cpu.registers.v.write(*rd2, val2);
        }
      },

      // -- STOREs -- \\
      ISACommand::Strx { rt, imm } => {
        let base = cpu.registers.sp + *imm;
        if base > cpu.memory.stack.len() {
          panic!("InterpreterError::StackError: Stack overflow caught at `strx`!");
        } else {
          let bytes: [u8; 8] = u64::to_le_bytes(cpu.registers.x.read(*rt));
          for i in 0..8 {
            cpu.memory.stack[base + i] = bytes[i]; 
          }
        }
      },
      ISACommand::Strv { rt, imm } => {
        let base = cpu.registers.sp + *imm;
        if base > cpu.memory.stack.len() {
          panic!("InterpreterError::StackError: Stack overflow caught at `strv`!");
        } else {
          let bytes: [u8; 8] = f64::to_le_bytes(cpu.registers.v.read(*rt));
          for i in 0..8 {
            cpu.memory.stack[base + i] = bytes[i]; 
          }
        }
      },
      ISACommand::Stpx { rt1, rt2, imm } => {
        let base = cpu.registers.sp + *imm;
        if base + 16 > cpu.memory.stack.len() {
          panic!("InterpreterError::StackError: Stack overflow caught at `strx`!");
        } else {
          let bytes1: [u8; 8] = u64::to_le_bytes(cpu.registers.x.read(*rt1));
          let bytes2: [u8; 8] = u64::to_le_bytes(cpu.registers.x.read(*rt2));
          for i in 0..8 {
            cpu.memory.stack[base + i] = bytes1[i]; 
            cpu.memory.stack[base + 8 + i] = bytes2[i]; 
          }
        }
      },
      ISACommand::Stpv { rt1, rt2, imm } => {
        let base = cpu.registers.sp + *imm;
        if base + 16 > cpu.memory.stack.len() {
          panic!("InterpreterError::StackError: Stack overflow caught at `strx`!");
        } else {
          let bytes1: [u8; 8] = f64::to_le_bytes(cpu.registers.v.read(*rt1));
          let bytes2: [u8; 8] = f64::to_le_bytes(cpu.registers.v.read(*rt2));
          for i in 0..8 {
            cpu.memory.stack[base + i] = bytes1[i]; 
            cpu.memory.stack[base + 8 + i] = bytes2[i]; 
          }
        }
      },

      // -- Exit -- \\
      ISACommand::Exit => {
        cpu.state.is_running = false;
      },
    }
  }
}

pub type Program = Vec<ISACommand>;