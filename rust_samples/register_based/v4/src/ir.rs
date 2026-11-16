// ।। ॐ नमः शिवाय ।। \\
// @date_start 5th November, 2026

#![allow(warnings)]

use crate::cpu::CPU;
use crate::syscalls::SyscallsID;

#[derive(Debug)]
pub enum Condition {
  Equal,               // beq
  NotEqual,            // bne
  GreaterThan,         // bgt
  LessThan,            // blt
  GreaterThanOrEqual,  // bge
  LessThanOrEqual      // ble
}

#[derive(Debug)]
pub enum Operation {
  Add,
  Sub,
  Mul,
  Div,
  Rem
}

#[derive(Debug)]
pub enum IRCommands {
  MoveImm      { rd: usize, imm: u64 },
  MoveReg      { rd: usize, rn: usize },
  MoveRegImmOp { rd: usize, rn: usize, imm: u64, op: Operation },
  MoveRegRegOp { rd: usize, rn: usize, rm: usize, op: Operation },
  Branch       { ta: usize },
  BranchRegImm { rn: usize, imm: u64, cd: Condition, ta: usize },
  BranchRegReg { rn: usize, rm: usize, cd: Condition, ta: usize },
  Call         { sb: usize },
  Return,
  Syscall      { scid: SyscallsID },
  Exit
}

impl IRCommands {
  #[inline(always)]
  pub fn execute(&self, cpu: &mut CPU) {
    match self {
      IRCommands::MoveImm { rd, imm } => {
        cpu.registers.write(*rd, *imm);
      },
      IRCommands::MoveReg { rd, rn } => {
        cpu.registers.write(*rd, cpu.registers.read(*rn));
      },
      IRCommands::MoveRegImmOp { rd, rn, imm, op } => {
        match op {
          Operation::Add => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) + *imm);
          },
          Operation::Sub => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) - *imm);
          },
          Operation::Mul => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) * *imm);
          },
          Operation::Div => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) / *imm);
          },
          Operation::Rem => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) % *imm);
          },
        }
      },
      IRCommands::MoveRegRegOp { rd, rn, rm, op } => {
        match op {
          Operation::Add => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) + cpu.registers.read(*rm));
          },
          Operation::Sub => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) - cpu.registers.read(*rm));
          },
          Operation::Mul => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) * cpu.registers.read(*rm));
          },
          Operation::Div => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) / cpu.registers.read(*rm));
          },
          Operation::Rem => {
            cpu.registers.write(*rd, cpu.registers.read(*rn) % cpu.registers.read(*rm));
          },
        }
      },
      IRCommands::Branch { ta } => {
        cpu.pc = *ta;
      }
      IRCommands::BranchRegImm { rn, imm, cd, ta } => {
        match cd {
          Condition::Equal => {
            if cpu.registers.read(*rn) == *imm {
              cpu.pc = *ta;
            }
          },
          Condition::NotEqual => {
            if cpu.registers.read(*rn) != *imm {
              cpu.pc = *ta;
            }
          },
          Condition::GreaterThan => {
            if cpu.registers.read(*rn) > *imm {
              cpu.pc = *ta;
            }
          },
          Condition::LessThan => {
            if cpu.registers.read(*rn) < *imm {
              cpu.pc = *ta;
            }
          },
          Condition::GreaterThanOrEqual => {
            if cpu.registers.read(*rn) >= *imm {
              cpu.pc = *ta;
            }
          },
          Condition::LessThanOrEqual => {
            if cpu.registers.read(*rn) <= *imm {
              cpu.pc = *ta;
            }
          },
        }
      },
      IRCommands::BranchRegReg { rn, rm, cd, ta } => {
        match cd {
          Condition::Equal => {
            if cpu.registers.read(*rn) == cpu.registers.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::NotEqual => {
            if cpu.registers.read(*rn) != cpu.registers.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::GreaterThan => {
            if cpu.registers.read(*rn) > cpu.registers.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::LessThan => {
            if cpu.registers.read(*rn) < cpu.registers.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::GreaterThanOrEqual => {
            if cpu.registers.read(*rn) >= cpu.registers.read(*rm) {
              cpu.pc = *ta;
            }
          },
          Condition::LessThanOrEqual => {
            if cpu.registers.read(*rn) <= cpu.registers.read(*rm) {
              cpu.pc = *ta;
            }
          },
        }
      },
      IRCommands::Call { sb } => {
        cpu.stack_line.push(cpu.pc + 1);
        cpu.pc = *sb;
      },
      IRCommands::Return => {
        cpu.pc = cpu.stack_line.pop().expect("StackUnderflowError: Caught at IR execution stage.\nNo corresponding stack address for `Return` instruction.\nLikely caused due to uneven number of Call-Return.\nConsider having a `Return` for each `Call` and vice-versa.");
      },
      IRCommands::Syscall { scid } => {
        cpu.call_system(scid);
      },
      IRCommands::Exit  => {
        cpu.state.is_running = false;
      }
    }
  }
}

pub type Program = Vec<IRCommands>;