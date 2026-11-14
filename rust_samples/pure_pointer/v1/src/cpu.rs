// ।। ॐ नमः शिवाय ।। \\
// @date_start 12th November, 2026

#![allow(warnings)]

use crate::ir::Program;

pub struct Memory {
  pub u64: Vec<u64>,
  pub f64: Vec<f64>,
}

impl Memory {
  pub fn new(size: usize) -> Self {
    Self {
      u64: vec![0; size],
      f64: vec![0.0; size],
    }
  }
}

pub struct State {
  pub is_running: bool,
}

impl State {
  pub fn new() -> Self {
    Self {
      is_running: false,
    }
  }
}

pub struct CPU {
  pub memory: Memory,
  pub state: State,
  pub pc: usize,
}

impl CPU {
  pub fn new(size: usize) -> Self {
    Self {
      memory: Memory::new(size),
      state: State::new(),
      pc: 0,
    }
  }

  pub fn execute_program(&mut self, program: &Program) {
    self.state.is_running = true;
    let mut old_pc: usize = 0;

    while self.state.is_running && self.pc < program.len() {
      old_pc = self.pc;
      program[self.pc].execute(self);
      if old_pc == self.pc {
        self.pc += 1;
      }
    }
  }

  pub fn reset(&mut self) {
    self.pc = 0;
    self.state.is_running = false;
    self.memory.f64.fill(0.0);
    self.memory.u64.fill(0);
  }
}