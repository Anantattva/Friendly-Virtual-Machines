// ॐ श्री गणेशाय नम \\
// ।। ॐ नमः शिवाय ।। \\
// @date_start 12th November, 2026

#![allow(warnings)]

use std::time::Instant;
use std::hint::black_box;

use crate::cpu::CPU;
use crate::ir::{ TypeTag, Value };
use crate::ir::{ Operation, Condition };
use crate::ir::{ IR, Program };

pub fn run_addition_test(max: u64, times: u128) {
  // -- Rust Baseline -- \\
  let mut rust_sum: u64 = 0;
  let rust_start = Instant::now();
  for _ in 0..times {
    rust_sum = 0;
    for i in 1..=max {
      rust_sum += black_box(i);
    }
  }
  let average_rust_time = rust_start.elapsed().as_nanos() / times;

  // -- Pure Pointer VM Benchmark -- \\
  let mut my_cpu = CPU::new(1024);
  let program: Program = vec![
    IR::Movv { ptrd: 0, val: Value::U64(0) },
    IR::Movv { ptrd: 1, val: Value::U64(0) },

    IR::Calcv { ptrd: 1, ptrm: 1, val: Value::U64(1), op: Operation::Add },
    IR::Calcp { ptrd: 0, ptrm: 0, ptrn: 1, dtype: TypeTag::U64, op: Operation::Add },

    IR::Bcv { ta: 2, ptrm: 1, val: Value::U64(max), cd: Condition::Ne },
    IR::Exit,
  ];

  let vm_start = Instant::now();
  for _ in 0..times {
    my_cpu.reset();
    my_cpu.execute_program(&program);
  }
  let average_vm_time = vm_start.elapsed().as_nanos() / times;
  let vm_sum = my_cpu.memory.u64[0];

  // -- Assertion & Result -- \\
  assert_eq!(
    rust_sum,
    vm_sum,
    "Rust sum {} != VM sum {}",
    rust_sum,
    vm_sum,
  );
  let gap = average_vm_time as f64 / average_rust_time as f64;

  println!("++ Addition Benchmark ++");
  println!("== Max: {} | Runs: {} ==", max, times);
  println!("Total sum:          {}", vm_sum);
  println!("Native Rust time:   {}ns", average_rust_time);
  println!("VM time:            {}ns", average_vm_time);
  println!("Gap (VM / Rust):    {:.2}x\n", gap);
}

pub fn run_fibonacci_test(index: u64, times: u128) {
  // -- Rust Baseline -- \\
  fn fib(n: u64) -> u64 {
    match n {
      0 | 1 => n,
      _ => {
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        for _ in 2..n {
          a = c;
          c = c + b;
          b = a;
        }
        c
      }
    }
  }

  let start_rust = Instant::now();
  let mut fib_rust: u64 = 0;
  for _ in 0..times {
    fib_rust = black_box(fib(index));
  }
  let average_rust_time = start_rust.elapsed().as_nanos() / times;

  // -- My VM Benchmark -- \\
  let mut my_cpu = CPU::new(1024);
  let program: Program = vec![
    IR::Movv { ptrd: 0, val: Value::U64(0) },
    IR::Movv { ptrd: 1, val: Value::U64(index) },

    // << base case: index <= 1 >> \\
    IR::Bcv  { ta: 5 , ptrm: 1, val: Value::U64(1), cd: Condition::Gt },
    IR::Movp { ptrd: 0, ptrm: 1, dtype: TypeTag::U64 },
    IR::Exit,

    IR::Movv { ptrd: 2, val: Value::U64(2) }, // counter
    IR::Movv { ptrd: 3, val: Value::U64(0) }, // a
    IR::Movv { ptrd: 4, val: Value::U64(1) }, // b
    IR::Movv { ptrd: 5, val: Value::U64(1) }, // c

    IR::Movp  { ptrd: 3, ptrm: 5, dtype: TypeTag::U64 },
    IR::Calcp { ptrd: 5, ptrm: 5, ptrn: 4, dtype: TypeTag::U64, op: Operation::Add },
    IR::Movp  { ptrd: 4, ptrm: 3, dtype: TypeTag::U64 },
    IR::Calcv { ptrd: 2, ptrm: 2, val: Value::U64(1), op: Operation::Add },
    IR::Bcv   { ta: 9 , ptrm: 2, val: Value::U64(index), cd: Condition::Ne },

    IR::Movp { ptrd: 0, ptrm: 5, dtype: TypeTag::U64 },
    IR::Exit,
  ];

  let start_vm = Instant::now();
  for _ in 0..times {
    my_cpu.reset();
    my_cpu.execute_program(&program);
  }
  let average_vm_time = start_vm.elapsed().as_nanos() / times;
  let vm_fib = my_cpu.memory.u64[0];

  assert_eq!(
    fib_rust,
    vm_fib,
    "{} != {}",
    fib_rust,
    vm_fib
  );

  let gap = average_vm_time as f32 / average_rust_time as f32;

  println!("== Fibonacci Index: {} | Times: {} ==", index, times);
  println!("Fib({}) = {}", index, fib_rust);
  println!("Rust average time: {}ns.", average_rust_time);
  println!("VM average time: {}ns.", average_vm_time);
  println!("Gap (VM / Rust): {}x.\n", gap);
}