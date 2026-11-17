// ॐ श्री गणेशाय नम \\
// ।। ॐ नमः शिवाय ।। \\
// @date_start 17th November, 2026

use crate::bytecode::Program;

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

pub struct VM {
  pub stack: Vec<i64>,
  pub local: Vec<i64>,
  pub sp: usize,
  pub pc: usize,
  pub state: State,
}

impl VM {
  pub fn new() -> Self {
    Self {
      stack: vec![],
      local: vec![0; 16], // preallocate a few empty local variable
      sp: 0,
      pc: 0,
      state: State::new(),
    }
  }
}

impl VM {
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
    self.stack.clear();
    self.local.fill(0);
    self.sp = 0;
    self.pc = 0;
    self.state.is_running = false;
  }
}