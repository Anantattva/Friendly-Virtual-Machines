// ।। ॐ नमः शिवाय ।। \\
// @date_start 12th November, 2026

#![allow(warnings)]

use crate::cpu::CPU;

// ++ Intermediate Representation ++ \\

pub enum TypeTag {
  U64,
  F64,
}

pub enum Value {
  U64(u64),
  F64(f64),
}

pub enum Operation {
  Add,
  Sub,
  Mul,
  Div,
  Rem,
}

// #[derive(PartialEq, PartialOrd)]
pub enum Condition {
  Eq,  // equal
  Ne,  // not equal
  Gt,  // greater than
  Lt,  // less than
  Ge,  // greater than or equal
  Le,  // less than or equal
}

pub enum IR {
  Movv { ptrd: usize, val: Value },
  Movp { ptrd: usize, ptrm: usize, dtype: TypeTag },
  Calcv { ptrd: usize, ptrm: usize, val: Value, op: Operation },
  Calcp { ptrd: usize, ptrm: usize, ptrn: usize, dtype: TypeTag, op: Operation },
  B { ta: usize },
  Bcv { ta: usize, ptrm: usize, val: Value, cd: Condition },
  Bcp { ta: usize, ptrm: usize, ptrn: usize, dtype: TypeTag, cd: Condition },
  Exit,
}

impl IR {
  pub fn execute(&self, cpu: &mut CPU) {
    match self {
      IR::Movv { ptrd, val } => {
        match val {
          Value::U64(v) => cpu.memory.u64[*ptrd] = *v,
          Value::F64(v) => cpu.memory.f64[*ptrd] = *v,
        }
      },
      IR::Movp { ptrd, ptrm, dtype } => {
        match dtype {
          TypeTag::U64 => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm],
          TypeTag::F64 => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm],
        }
      },
      IR::Calcv { ptrd, ptrm, val, op } => {
        match val {
          Value::U64(v) => {
            match op {
              Operation::Add => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] + *v,
              Operation::Sub => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] - *v,
              Operation::Mul => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] * *v,
              Operation::Div => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] / *v,
              Operation::Rem => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] % *v,
            }
          },
          Value::F64(v) => {
            match op {
              Operation::Add => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] + *v,
              Operation::Sub => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] - *v,
              Operation::Mul => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] * *v,
              Operation::Div => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] / *v,
              Operation::Rem => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] % *v,
            }
          },
        }
      },
      IR::Calcp { ptrd, ptrm, ptrn, dtype, op } => {
        match dtype {
          TypeTag::U64 => {
            match op {
              Operation::Add => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] + cpu.memory.u64[*ptrn],
              Operation::Sub => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] - cpu.memory.u64[*ptrn],
              Operation::Mul => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] * cpu.memory.u64[*ptrn],
              Operation::Div => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] / cpu.memory.u64[*ptrn],
              Operation::Rem => cpu.memory.u64[*ptrd] = cpu.memory.u64[*ptrm] % cpu.memory.u64[*ptrn],
            }
          },
          TypeTag::F64 => {
            match op {
              Operation::Add => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] + cpu.memory.f64[*ptrn],
              Operation::Sub => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] - cpu.memory.f64[*ptrn],
              Operation::Mul => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] * cpu.memory.f64[*ptrn],
              Operation::Div => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] / cpu.memory.f64[*ptrn],
              Operation::Rem => cpu.memory.f64[*ptrd] = cpu.memory.f64[*ptrm] % cpu.memory.f64[*ptrn],
            }
          },
        }
      },
      IR::B { ta } => {
        cpu.pc = *ta;
      },
      IR::Bcv { ta, ptrm, val, cd } => {
        match val {
          Value::U64(v) => {
            let is_true = match cd {
              Condition::Eq => cpu.memory.u64[*ptrm] == *v,
              Condition::Ne => cpu.memory.u64[*ptrm] != *v,
              Condition::Gt => cpu.memory.u64[*ptrm] > *v,
              Condition::Lt => cpu.memory.u64[*ptrm] < *v,
              Condition::Ge => cpu.memory.u64[*ptrm] >= *v,
              Condition::Le => cpu.memory.u64[*ptrm] <= *v,
            };
            if is_true {
              cpu.pc = *ta;
            }
          },
          Value::F64(v) => {
            let is_true = match cd {
              Condition::Eq => cpu.memory.f64[*ptrm] == *v,
              Condition::Ne => cpu.memory.f64[*ptrm] != *v,
              Condition::Gt => cpu.memory.f64[*ptrm] > *v,
              Condition::Lt => cpu.memory.f64[*ptrm] < *v,
              Condition::Ge => cpu.memory.f64[*ptrm] >= *v,
              Condition::Le => cpu.memory.f64[*ptrm] <= *v,
            };
            if is_true {
              cpu.pc = *ta;
            }
          },
        }
      }
      IR::Bcp { ta, ptrm, ptrn, dtype, cd } => {
        match dtype {
          TypeTag::U64 => {
            let is_true = match cd {
              Condition::Eq => cpu.memory.u64[*ptrm] == cpu.memory.u64[*ptrn],
              Condition::Ne => cpu.memory.u64[*ptrm] != cpu.memory.u64[*ptrn],
              Condition::Gt => cpu.memory.u64[*ptrm] > cpu.memory.u64[*ptrn],
              Condition::Lt => cpu.memory.u64[*ptrm] < cpu.memory.u64[*ptrn],
              Condition::Ge => cpu.memory.u64[*ptrm] >= cpu.memory.u64[*ptrn],
              Condition::Le => cpu.memory.u64[*ptrm] <= cpu.memory.u64[*ptrn],
            };
            if is_true {
              cpu.pc = *ta;
            }
          },
          TypeTag::F64 => {
            let is_true = match cd {
              Condition::Eq => cpu.memory.f64[*ptrm] == cpu.memory.f64[*ptrn],
              Condition::Ne => cpu.memory.f64[*ptrm] != cpu.memory.f64[*ptrn],
              Condition::Gt => cpu.memory.f64[*ptrm] > cpu.memory.f64[*ptrn],
              Condition::Lt => cpu.memory.f64[*ptrm] < cpu.memory.f64[*ptrn],
              Condition::Ge => cpu.memory.f64[*ptrm] >= cpu.memory.f64[*ptrn],
              Condition::Le => cpu.memory.f64[*ptrm] <= cpu.memory.f64[*ptrn],
            };
            if is_true {
              cpu.pc = *ta;
            }
          },
        }
      }
      IR::Exit => {
        cpu.state.is_running = false;
      },
    }
  }
}

pub type Program = Vec<IR>;