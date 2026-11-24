// ।। ॐ नमः शिवाय ।। \\
// @date_start 8th November, 2026

#![allow(warnings)]

use std::time::Instant;
use std::hint::black_box;

use crate::cpu::CPU;
use crate::isa::ISACommand;
use crate::isa::Program;
use crate::assembler::assemble_program;

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

pub fn run_is_prime_test(num: u64) {
  fn is_prime(n: u64) -> u64 {
    for i in 2..=(n / 2) {
      if n % i == 0 {
        return 0;
      }
    }
    1
  }

  let is_prime_rust = is_prime(num);

  let mut my_cpu = CPU::new(1024, 4096);
  let program = assemble_program(&
    format!(
      "
      mov x0, #{}
      mov x1, x0      // store num      mov x0, #1      // store default result: true/1
      div x2, x1, #2  // divisor limit = num / 2
      mov x3, #2      // start divisor = 2

      check_loop:
        bgt prime_true, x3, x2
        rem x4, x1, x3           // remainder = num % divisor
        beq prime_false, x4, #0  // if rem == 0: not prime
        add x3, x3, #1           // divisor++
        b check_loop            
        exit

      prime_true:
        mov x0, #1
        exit
        
      prime_false:
        mov x0, #0
        exit
      ",
      num
    )
  );

  my_cpu.execute_program(&program.unwrap());

  assert_eq!(is_prime_rust, my_cpu.registers.x.read(0));
  println!("++ Is Prime Test?? ++");
  println!("++ 1 = True | 0 = False ++");
  println!("Number: {}", num);
  println!("Rust Result: {} | VM Result: {}\n", is_prime_rust,  my_cpu.registers.x.read(0));
}

pub fn run_fib_test1(num: u64, times: u128) {
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
    main:
      mov x0, #{}  // store num
      bl fib
      exit

    fib:
      ble base_case, x0, #1  // num <= 1
      b other_case
      
      base_case:
        ret      

      other_case:
        // prologue
        subsp #16
        str x30, [sp]
        str x0, [sp + #8]

        // recursive callback: fib(n - 1)
        sub x0, x0, #1
        bl fib
        subsp #8
        str x0, [sp]
        
        // recursive callback: fib(n - 2)
        ldr x0, [sp + #16]
        sub x0, x0, #2
        bl fib

        // final result
        ldr x1, [sp]
        addsp #8
        add x0, x1, x0

        // epilogue
        ldr x30, [sp]
        addsp #16
        ret
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
  println!("++ Fibonacci Benchmark 1 ++");
  println!("== Index: {} | Runs: {} ==", num, times);
  println!("-----------------------------------");
  println!("Assembler compilation time: {}ns", assemble_time);
  println!("Rust average time:          {}ns", average_rust_time);
  println!("VM average time:            {}ns", average_vm_time);
  println!("Computed Fibonacci Result:  {}.", rust_fib);
  println!("Performance Gap (VM / Rust): {:.2}x\n", gap);
}

pub fn run_fib_test2(num: u64, times: u128) {
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
    main:
      mov x0, #{}  // store num
      bl fib
      exit

    fib:
      bgt other_case, x0, #1  // num > 1
      b base_case
      
      base_case:
        ret      

      other_case:
        // prologue
        subsp #16
        stp x30, x0, [sp]

        // recursive callback: fib(n - 1)
        sub x0, x0, #1
        bl fib
        subsp #8
        str x0, [sp]
        
        // recursive callback: fib(n - 2)
        ldr x0, [sp + #16]
        sub x0, x0, #2
        bl fib

        // final result
        ldr x1, [sp]
        addsp #8
        add x0, x1, x0

        // epilogue
        ldr x30, [sp]
        addsp #16
        ret
  ", num);
  
  let mut my_cpu = CPU::new(1024, (num * 8 + 1024) as usize);

  let assemble_start = Instant::now();
  let program = assemble_program(&program_text).expect("Found errors in program during assembler stage. Please fix first!.\n");
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
  println!("++ Fibonacci Benchmark 2 ++");
  println!("== Index: {} | Runs: {} ==", num, times);
  println!("-----------------------------------");
  println!("Assembler compilation time: {}ns", assemble_time);
  println!("Rust average time:          {}ns", average_rust_time);
  println!("VM average time:            {}ns", average_vm_time);
  println!("Computed Fibonacci Result:  {}.", rust_fib);
  println!("Performance Gap (VM / Rust): {:.2}x\n", gap);
}

pub fn run_tests() {
  run_addition_test(1000, 1000);
  run_addition_test(10000, 1000);
  
  run_is_prime_test(1);
  run_is_prime_test(2);
  run_is_prime_test(3);
  run_is_prime_test(5);
  run_is_prime_test(12);
  run_is_prime_test(71);
  run_is_prime_test(120);

  run_fib_test1(5, 1);
  run_fib_test1(12, 1);
  run_fib_test1(5, 100);
  run_fib_test1(12, 100);
  
  run_fib_test2(5, 100);
  run_fib_test2(12, 100);
}
