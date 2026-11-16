// ।। ॐ नमः शिवाय ।। \\
// @date_start 7th November, 2026

#![allow(warnings)]

use crate::architecture::{Registers, State, Memory};
use crate::isa::Program;

pub struct CPU {
  pub memory: Memory,
  pub registers: Registers,
  pub state: State,
  pub pc: usize,
}

impl CPU {
  pub fn new(stack_size: usize, heap_size: usize) -> Self {
    Self {
      memory: Memory::new(stack_size, heap_size),
      registers: Registers::new(),
      state: State::new(),
      pc: 0,
    }
  }
}

impl CPU {
  pub fn execute_program(&mut self, program: &Program) {
    self.state.is_running = true;
    let mut old_pc: usize = 0;
    while self.pc < program.len() && self.state.is_running {
      old_pc = self.pc;
      program[self.pc].execute(self);

      if old_pc == self.pc {
        self.pc += 1;
      }
    }
  }
}

impl CPU {
  pub fn clear_memory(&mut self) {
    self.registers.x.0.fill(0);
    self.registers.v.0.fill(0.0);
    self.memory.stack.fill(0);
    self.memory.heap.fill(0);
    self.registers.sp = 0;
    self.registers.hp = 0;
  }

  pub fn reset(&mut self) {
    self.clear_memory();
    self.state.is_running = false;
    self.pc = 0;
  }
}