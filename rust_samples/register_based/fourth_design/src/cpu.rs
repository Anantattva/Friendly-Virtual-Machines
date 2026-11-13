// ।। ॐ नमः शिवाय ।। 
// @date_start 5th November, 2026

use crate::cpu_design;
use crate::ir;

pub struct CPU {
  pub registers: cpu_design::Registers,
  pub memory: Vec<u8>,
  pub sp: usize,         // stack pointer/stack register
  pub stack_line: Vec<usize>,
  pub pc: usize,       // program counter
  pub state: cpu_design::State,
}

impl CPU {
  pub fn new(construction_size: usize) -> Self {
    Self {
      registers: cpu_design::Registers::new(),
      memory: vec![0; construction_size],
      sp: 0,
      stack_line: vec![],
      pc: 0,
      state: cpu_design::State::new(),
    }
  }
}

impl CPU {
  pub fn clear_memory(&mut self) {
    self.registers.x.fill(0);
    self.memory.fill(0);
    self.stack_line.clear();
  }
  
  pub fn reset(&mut self) {
    self.clear_memory();
    self.flags.reset();
    self.sp = 0;
    self.pc = 0;
    self.state.reset();
  }
}

impl CPU {
  pub fn execute_program(&mut self, program: &ir::Program) {
    self.state.is_running = true;
    let mut old_pc = 0;
    while self.state.is_running && self.pc < program.len() {
      unsafe {
        old_pc = self.pc;
        program.get_unchecked(self.pc).execute(self);
  
        // << increment only if instruction didnt change pc >> \\
        if old_pc == self.pc {
          self.pc += 1;
        }
      }
    }
  }
}
