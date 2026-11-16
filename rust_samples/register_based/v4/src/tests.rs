// ।। ॐ नमः शिवाय ।। \\
// @date_start 5th November, 2026

#![allow(warnings)]

use std::time::Instant;
use std::hint::black_box;

use crate::cpu::CPU;
use crate::ir::{ IRCommands, Operation, Condition };
use crate::ir::Program;

pub fn run_addition_test(max: u64, times: u128) {
  // --- Rust Baseline --- \\
  let mut sum_rust: u64 = 0;
  let start_rust = Instant::now();
  for _ in 0..times {
    sum_rust = black_box(0);
    for k in 1..=max {
      sum_rust += black_box(k);
    }
  }
  let average_time_rust = start_rust.elapsed().as_nanos() / times;

  // --- My IR VM Benchmark --- \\
  let mut my_cpu = CPU::new(1024);
  let program: Program = vec![
    IRCommands::MoveImm { rd: 0, imm: 0 },
    IRCommands::MoveImm { rd: 1, imm: 0 },

    IRCommands::MoveRegImmOp { rd: 1, rn: 1, imm: 1, op: Operation::Add },
    IRCommands::MoveRegRegOp { rd: 0, rn: 0, rm: 1, op: Operation::Add },
    IRCommands::BranchRegImm { rn: 1, imm: max, cd: Condition::NotEqual, ta: 2 },
    IRCommands::Exit,
  ];

  let start_vm = Instant::now();
  for _ in 0..times {
    my_cpu.reset();
    my_cpu.execute_program(&program);
  }
  let average_time_vm = start_vm.elapsed().as_nanos() / times;

  // -- Assertion & Result -- \\
  assert_eq!(
    sum_rust,
    my_cpu.registers.read(0),
    "Rust sum {} != VM sum {}.",
    sum_rust,
    my_cpu.registers.read(0),    
  );
  let gap = average_time_vm / average_time_rust;
  println!("== Sum upto {} across {} runs ==", max, times);
  println!("Rust time: {}ns | VM time: {}ns", average_time_rust, average_time_vm);
  println!("Gap VM / Rust: {}x.\n", gap);
}

pub fn run_pell_test(max: u64, times: u128) {
  // --- Rust Baseline --- \\
  fn pell(num: u64) -> u64 {
    if num == 0 {
      return 0;
    } else if num == 1 {
      return 1;
    } else {
      let mut counter = 2;
      let mut base0 = 0;
      let mut base1 = 1;
      let mut output = 2;
      while counter < num {
        counter += 1;
        base0 = output;
        output = 2 * output + base1;
        base1 = base0;
      }
      return output;
    }
  }
  
  let mut pell_rust: u64 = 0;
  let start_rust = Instant::now();
  for _ in 0..times {
    pell_rust = black_box(0);
    for k in 1..=max {
      pell_rust = black_box(pell(max));
    }
  }
  let average_time_rust = start_rust.elapsed().as_nanos() / times;

  // --- My IR VM Benchmark --- \\
  let mut my_cpu = CPU::new(1024);
  let program: Program = vec![
    IRCommands::MoveImm { rd: 0, imm: 0 },
    IRCommands::MoveImm { rd: 1, imm: max },
    IRCommands::MoveImm { rd: 2, imm: 0 },    // base0
    IRCommands::MoveImm { rd: 3, imm: 1 },    // base1
    IRCommands::MoveImm { rd: 4, imm: 2 },    // output
    IRCommands::MoveImm { rd: 5, imm: 2 },    // counter

    IRCommands::BranchRegImm { rn: 1, imm: 0, cd: Condition::Equal, ta: 16 },
    IRCommands::BranchRegImm { rn: 1, imm: 1, cd: Condition::Equal, ta: 16 },

    IRCommands::MoveReg { rd: 2, rn: 4 },
    IRCommands::MoveRegImmOp { rd: 4, rn: 4, imm: 2, op: Operation::Mul },
    IRCommands::MoveRegRegOp { rd: 4, rn: 4, rm: 3, op: Operation::Add },
    IRCommands::MoveReg { rd: 3, rn: 2 },
    IRCommands::MoveRegImmOp { rd: 5, rn: 5, imm: 1, op: Operation::Add },
    IRCommands::BranchRegImm { rn: 5, imm: max, cd: Condition::NotEqual, ta: 8 },
    IRCommands::MoveReg { rd: 0, rn: 4 },
    IRCommands::Exit,
    
    IRCommands::MoveReg { rd: 0, rn: 1 },
    IRCommands::Exit,
  ];
  let start_vm = Instant::now();
  for _ in 0..times {
    my_cpu.reset();
    my_cpu.execute_program(&program);
  }
  let average_time_vm = start_vm.elapsed().as_nanos() / times;

  // -- Assertion & Result -- \\
  assert_eq!(
    pell_rust,
    my_cpu.registers.read(0),
    "Rust pell {} != VM pell {}.",
    pell_rust,
    my_cpu.registers.read(0),    
  );
  let gap = average_time_vm / average_time_rust;
  println!("== Pell benchmark for {} across {} runs ==", max, times);
  println!("Rust time: {}ns | VM time: {}ns", average_time_rust, average_time_vm);
  println!("Gap VM / Rust: {}x.\n", gap);
}