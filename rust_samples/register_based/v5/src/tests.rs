// ।। ॐ नमः शिवाय ।। \\
// @date_start 8th November, 2026

#![allow(warnings)]

use std::time::Instant;
use std::hint::black_box;

use crate::cpu::CPU;
use crate::isa::ISACommand;
use crate::isa::Program;
use crate::assembler::assemble_program;

pub fn run_array_squaring_test(max: usize, times: u128) {
  // -- Rust Baseline -- \\
  let initial_data: Vec<u64> = (1..=max as u64).collect();
  let mut rust_array = initial_data.clone();
  let mut rust_array_sum: u64 = 0;
  
  let start_rust = Instant::now();
  for _ in 0..times {
    rust_array.copy_from_slice(&initial_data);
    rust_array_sum = 0;
    for i in 0..max {
      rust_array[i] = black_box(rust_array[i] * rust_array[i]);
      rust_array_sum += rust_array[i];
    }
  }
  let average_rust_time = start_rust.elapsed().as_nanos() / times;

  // -- My VM Benchmark -- \\
  let program_text = format!("\
    mov x0, #0        // sum accumulator
    mov x1, #{}       // max limit
    mov x2, #0        // index i = 0

    // Populate array in memory: stack[sp + 0] = i + 1, then advance sp by 8
    init_loop:
      bge init_done, x2, x1

      // x4 = i + 1
      add x4, x2, #1
      str x4, #0        // Store value at stack[sp + 0]

      addsp #8          // Advance stack pointer by 8 bytes
      add x2, x2, #1    // i++
      b init_loop

    init_done:
      // Reset stack pointer back by max * 8 bytes
      subsp #{}

      mov x2, #0        // Reset index i = 0 for math loop
      mov x3, #0        // Sum accumulator = 0

    // Now do squaring and summing
    loop_start:
      bge loop_end, x2, x1

      // Load element from stack[sp + 0] into x5
      ldr x5, #0

      // Square the element: x5 = x5 * x5
      mul x5, x5, x5

      // Accumulate sum: x3 = x3 + x5
      add x3, x3, x5

      addsp #8          // Advance stack pointer by 8 bytes
      add x2, x2, #1    // i++
      b loop_start

    loop_end:
      // Reset stack pointer back by max * 8 bytes
      subsp #{}

      mov x0, x3        // Move final sum to x0 for verification
      exit     
  ", max, max * 8, max * 8);
  
  let mut my_cpu = CPU::new(1024, max * 8 + 1024);

  let assemble_start = Instant::now();
  let program = assemble_program(&program_text).expect("Found errors in program during assembler stage. Please fix first!.");
  let assemble_time = assemble_start.elapsed().as_nanos();

  let start_vm = Instant::now();
  for _ in 0..times {
    my_cpu.reset();
    my_cpu.execute_program(&program);
  }
  let average_vm_time = start_vm.elapsed().as_nanos() / times;

  // -- Assertion & Result -- \\
  assert_eq!(
    rust_array_sum,
    my_cpu.registers.x.read(0),
    "Rust sum {} != VM sum {}.",
    rust_array_sum,
    my_cpu.registers.x.read(0)
  );

  let gap = average_vm_time as f64 / average_rust_time as f64;
  println!("++ Array Squaring & Sum Benchmark ++");
  println!("Array Size: {} | Sample Runs: {}", max, times);
  println!("--------------------------------------");
  println!("Assembler compilation time: {}ns", assemble_time);
  println!("Rust average time:          {}ns", average_rust_time);
  println!("VM average time:            {}ns", average_vm_time);
  println!("Computed Sum Result:        {}.", rust_array_sum);
  println!("Performance Gap (VM / Rust): {:.2}x\n", gap);
}

pub fn run_addition_test(max: u64, times: u128) {
  // -- Rust Baseline -- \\
  let mut rust_sum: u64 = 0;
  
  let start_rust = Instant::now();
  for _ in 0..times {
    rust_sum = 0;
    for i in 1..=max {
      rust_sum += black_box(i);
    }
  }
  let average_rust_time = start_rust.elapsed().as_nanos() / times;

  // -- My VM Benchmark -- \\
  let program_text = format!("
    mov x0, #0
    mov x1, #{}
    mov x2, #0

    add_loop:
      add x2, x2, #1
      add x0, x0, x2
      bne add_loop, x2, x1
  ", max);
  
  let mut my_cpu = CPU::new(1024, (max * 8 + 1024) as usize);

  let assemble_start = Instant::now();
  let program = assemble_program(&program_text).expect("Found errors in program during assembler stage. Please fix first!.");
  let assemble_time = assemble_start.elapsed().as_nanos();

  let start_vm = Instant::now();
  for _ in 0..times {
    my_cpu.reset();
    my_cpu.execute_program(&program);
  }
  let average_vm_time = start_vm.elapsed().as_nanos() / times;

  // -- Assertion & Result -- \\
  assert_eq!(
    rust_sum,
    my_cpu.registers.x.read(0),
    "Rust sum {} != VM sum {}.",
    rust_sum,
    my_cpu.registers.x.read(0)
  );

  let gap = average_vm_time as f64 / average_rust_time as f64;
  println!("++ Addition Benchmark ++");
  println!("== Max: {} | Runs: {} ==", max, times);
  println!("-----------------------------------");
  println!("Assembler compilation time: {}ns", assemble_time);
  println!("Rust average time:          {}ns", average_rust_time);
  println!("VM average time:            {}ns", average_vm_time);
  println!("Computed Sum Result:        {}.", rust_sum);
  println!("Performance Gap (VM / Rust): {:.2}x\n", gap);
}

pub fn run_naive_fib_test(num: u64, times: u128) {
  // -- Rust Baseline -- \\
  fn fib(n: u64) -> u64 {
    match n <= 1 {
      true => n,
      _ => fib(n - 1) + fib(n - 2),
    }
  }
  
  let mut rust_fib: u64 = 0;  
  let start_rust = Instant::now();
  for _ in 0..times {
    rust_fib = black_box(fib(num));
  }
  let average_rust_time = start_rust.elapsed().as_nanos() / times;

  // -- My VM Benchmark -- \\
  let program_text = format!("
    mov x1, #{}

    base_case:
      bgt x1, #1
      mov x0, x1
      exit

    other_case:
      
  ", num);
  
  let mut my_cpu = CPU::new(1024, (num * 8 + 1024) as usize);

  let assemble_start = Instant::now();
  let program = assemble_program(&program_text).expect("Found errors in program during assembler stage. Please fix first!.");
  let assemble_time = assemble_start.elapsed().as_nanos();

  let start_vm = Instant::now();
  for _ in 0..times {
    my_cpu.reset();
    my_cpu.execute_program(&program);
  }
  let average_vm_time = start_vm.elapsed().as_nanos() / times;

  // -- Assertion & Result -- \\
  assert_eq!(
    rust_fib,
    my_cpu.registers.x.read(0),
    "Rust fib {} != VM fib {}.",
    rust_fib,
    my_cpu.registers.x.read(0)
  );

  let gap = average_vm_time as f64 / average_rust_time as f64;
  println!("++ Fibonacci Benchmark ++");
  println!("== Index: {} | Runs: {} ==", num, times);
  println!("-----------------------------------");
  println!("Assembler compilation time: {}ns", assemble_time);
  println!("Rust average time:          {}ns", average_rust_time);
  println!("VM average time:            {}ns", average_vm_time);
  println!("Computed Fibonacci Result:  {}.", rust_fib);
  println!("Performance Gap (VM / Rust): {:.2}x\n", gap);
}
