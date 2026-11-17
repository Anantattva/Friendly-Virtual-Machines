// ॐ श्री गणेशाय नम \\
// ।। ॐ नमः शिवाय ।। \\
// @date_start 17th November, 2026

use crate::vm::VM;

#[derive(Debug, Copy, Clone)]
pub enum Bytecode {
  Exit,
  Push(i64),
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Jump(usize),
  Eq,  // equal
  Ne,  // not equal
  Gt,  // greater than
  Lt,  // less than
  Ge,  // greater than or equal
  Le,  // less than or equal
  JumpIfTrue(usize),
  JumpIfFalse(usize),
  StoreLocal(usize), // Pops stack, saves into local[index]
  GetLocal(usize),   // Loads local[index], pushes onto stack
}

impl Bytecode {
  #[inline(always)]
  pub fn execute(&self, vm: &mut VM) {
    match self {
      Bytecode::Exit => {
        vm.state.is_running = false;
      },
      Bytecode::Push(value) => {
        vm.stack.push(*value);
      },
      Bytecode::Add => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(lhs + rhs);
      },
      Bytecode::Sub => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(lhs - rhs);
      },
      Bytecode::Mul => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(lhs * rhs);
      },
      Bytecode::Div => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(lhs / rhs);
      },
      Bytecode::Rem => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(lhs % rhs);
      },
      Bytecode::Jump(target_address) => {
        vm.pc = *target_address;
      },
      Bytecode::Eq => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(if lhs == rhs { 1 } else { 0 });
      },
      Bytecode::Ne => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(if lhs != rhs { 1 } else { 0 });
      },
      Bytecode::Gt => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(if lhs > rhs { 1 } else { 0 });
      },
      Bytecode::Lt => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(if lhs < rhs { 1 } else { 0 });
      },
      Bytecode::Ge => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(if lhs >= rhs { 1 } else { 0 });
      },
      Bytecode::Le => {
        let rhs = vm.stack.pop().unwrap();
        let lhs = vm.stack.pop().unwrap();
        vm.stack.push(if lhs <= rhs { 1 } else { 0 });
      },
      Bytecode::JumpIfTrue(target_address) => {
        let condition = vm.stack.pop().unwrap();
        if condition == 1 {
          vm.pc = *target_address;
        }
      },
      Bytecode::JumpIfFalse(target_address) => {
        let condition = vm.stack.pop().unwrap();
        if condition == 0 {
          vm.pc = *target_address;
        }
      },
      Bytecode::StoreLocal(index) => {
        let value = vm.stack.pop().unwrap();
        if *index >= vm.local.len() {
          vm.local.resize(index + 1, 0);
        }
        vm.local[*index] = value;
      },
      Bytecode::GetLocal(index) => {
        let value = vm.local[*index];
        vm.stack.push(value);
      },
    }
  }
}

pub type Program = Vec<Bytecode>;
