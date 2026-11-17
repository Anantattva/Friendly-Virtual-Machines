// ॐ श्री गणेशाय नम \\
// ।। ॐ नमः शिवाय ।। \\
// @date_start 17th November, 2026

use std::time::Instant;
use std::hint::black_box;

use crate::vm::VM;
use crate::bytecode::{Bytecode, Program};

pub fn run_addition_test(max: i64, times: u128) {
  // -- Rust Baseline -- \\
  let mut rust_sum: i64 = 0;
  let rust_start = Instant::now();
  for _ in 0..times {
    rust_sum = 0;
    for i in 1..=max {
      rust_sum += black_box(i);
    }
  }
  let average_rust_time = rust_start.elapsed().as_nanos() / times;

  // -- Stack Based VM Benchmark -- \\
  let mut my_vm = VM::new();
  // Local 0 = sum (0)
  // Local 1 = n   (max)

  let mut program = vec![];
  program.push(Bytecode::Push(0));
  program.push(Bytecode::StoreLocal(0)); // sum = 0

  program.push(Bytecode::Push(max));
  program.push(Bytecode::StoreLocal(1)); // n = max

  // --- LOOP START ---
  let loop_start = program.len();

  // If n == 0, exit loop.
  // Let's load n, and if it's 0, jump to exit.
  program.push(Bytecode::GetLocal(1));
  let exit_jump_idx = program.len();
  program.push(Bytecode::JumpIfFalse(0)); // Placeholder for exit

  // sum = sum + n
  program.push(Bytecode::GetLocal(0));
  program.push(Bytecode::GetLocal(1));
  program.push(Bytecode::Add);
  program.push(Bytecode::StoreLocal(0));

  // n = n - 1
  program.push(Bytecode::GetLocal(1));
  program.push(Bytecode::Push(1));
  program.push(Bytecode::Sub);
  program.push(Bytecode::StoreLocal(1));

  // Jump back to start
  program.push(Bytecode::Jump(loop_start));

  // --- EXIT ---
  let exit_pc = program.len();
  program[exit_jump_idx] = Bytecode::JumpIfFalse(exit_pc);

  // Push final sum
  program.push(Bytecode::GetLocal(0));

  let vm_start = Instant::now();
  for _ in 0..times {
    my_vm.reset();
    my_vm.execute_program(&program);
  }
  let average_vm_time = vm_start.elapsed().as_nanos() / times;
  let vm_sum = my_vm.stack[0];

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

pub fn run_fibonacci_test(index: i64, times: u128) {
  // -- Rust Baseline -- \\
  fn fib(n: i64) -> i64 {
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
  let mut fib_rust: i64 = 0;
  for _ in 0..times {
    fib_rust = black_box(fib(index));
  }
  let average_rust_time = start_rust.elapsed().as_nanos() / times;

  // -- My VM Benchmark -- \\
  let mut my_vm = VM::new();
  let mut program = vec![];

  // a = 0, b = 1, c = 1
  program.push(Bytecode::Push(0));
  program.push(Bytecode::StoreLocal(0));
  program.push(Bytecode::Push(1));
  program.push(Bytecode::StoreLocal(1));
  program.push(Bytecode::Push(1));
  program.push(Bytecode::StoreLocal(2));

  // i = 2 (loop counter stored in Local 3)
  program.push(Bytecode::Push(2));
  program.push(Bytecode::StoreLocal(3));

  // --- LOOP START ---
  let loop_start = program.len();

  // If i > index, exit loop. 
  // Let's check: index < i (or i <= index inverted)
  program.push(Bytecode::GetLocal(3));
  program.push(Bytecode::Push(1));
  program.push(Bytecode::Add);
  program.push(Bytecode::Push(index));
  program.push(Bytecode::Gt); // i > index ?
  let exit_jump_idx = program.len();
  program.push(Bytecode::JumpIfTrue(0)); // Placeholder for exit

  // a = c
  program.push(Bytecode::GetLocal(2));
  program.push(Bytecode::StoreLocal(0));

  // c = c + b
  program.push(Bytecode::GetLocal(2));
  program.push(Bytecode::GetLocal(1));
  program.push(Bytecode::Add);
  program.push(Bytecode::StoreLocal(2));

  // b = a (old c)
  program.push(Bytecode::GetLocal(0));
  program.push(Bytecode::StoreLocal(1));

  // i = i + 1
  program.push(Bytecode::GetLocal(3));
  program.push(Bytecode::Push(1));
  program.push(Bytecode::Add);
  program.push(Bytecode::StoreLocal(3));

  // Jump back to loop start
  program.push(Bytecode::Jump(loop_start));

  // --- EXIT ---
  let exit_pc = program.len();
  program[exit_jump_idx] = Bytecode::JumpIfTrue(exit_pc);

  // Push final result `c`
  program.push(Bytecode::GetLocal(2));

  let start_vm = Instant::now();
  for _ in 0..times {
    my_vm.reset();
    my_vm.execute_program(&program);
  }
  let average_vm_time = start_vm.elapsed().as_nanos() / times;
  let vm_fib = my_vm.stack[0];

  assert_eq!(
    fib_rust,
    vm_fib,
    "{} != {}",
    fib_rust,
    vm_fib
  );

  let gap = average_vm_time as f32 / average_rust_time as f32;

  println!("== Fibonacci Index: {} | Times: {} ==", index, times);
  println!("Fib({}) =            {}", index, fib_rust);
  println!("Rust average time:   {}ns.", average_rust_time);
  println!("VM average time:     {}ns.", average_vm_time);
  println!("Gap (VM / Rust):     {:.2}x.\n", gap);
}