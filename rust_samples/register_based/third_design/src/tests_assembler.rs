// ।। ॐ नमः शिवाय ।। \\
// @date_start 3rd November, 2026

use std::time::Instant;
use std::hint::black_box;
use crate::cpu::CPU;
use crate::assembler;

pub fn test_token_streamer() {
  let program1 = "
    mov x0, #1
    mov x2, #0

    loop:
      add x0, x0, #1
      mul x1, x0, x0
      add x2, x2, #1
      cmp x2, #10

    bne loop
    // target_address = 2
    exit
  ";

  println!("Program: {:#?}", program1);
  println!("TokenStream: {:#?}", assembler::stream_out_tokens_from_raw_text(&program1));
}

pub fn run_complete_test(max: u16, times: u32) {
  // -- 1. Assembler Process Timing & Compilation -- \\
  let start_assemble = Instant::now();
  
  // Dynamic assembly source text for Fibonacci calculation
  let program_text = format!("\
    mov x0, #0
    mov x1, #{}
    mov x2, #0
    mov x3, #1
    mov x4, #1
    mov x5, #0
    cmp x1, #1
    ble end_label
    
    loop_start:
      add x3, x3, x2
      mov x2, x4
      mov x4, x3
      add x5, x5, #1
      cmp x5, x1
    bne loop_start
    
    mov x0, x3
    exit
    
    end_label:
    mov x0, x1
    exit
  ", max - 1);

  // Compile raw text into executable ISA program
  let program = assembler::assemble_program(&program_text);
  let assemble_time = start_assemble.elapsed().as_nanos();

  // -- 2. Rust Native Baseline (Iterative O(n)) -- \\
  fn native_fib(num: u64) -> u64 {
    if num <= 1 {
      return num;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=num {
      let c = a + b;
      a = b;
      b = c;
    }
    b
  }

  let mut total_rust_time: u128 = 0;
  let mut rust_result: u64 = 0;
  for _ in 0..times {
    let start_rust = Instant::now();
    rust_result = black_box(native_fib(max as u64));
    total_rust_time += start_rust.elapsed().as_nanos();
  }
  let average_rust_time = total_rust_time / (times as u128);

  // -- 3. VM Benchmark Execution -- \\
  let mut my_cpu = CPU::new(1024);
  let mut total_vm_time: u128 = 0;
  for _ in 0..times {
    my_cpu.reset();
    let start_vm = Instant::now();
    my_cpu.execute_program(&program);
    total_vm_time += start_vm.elapsed().as_nanos();
  }
  let average_vm_time = total_vm_time / (times as u128);

  // -- 4. Verification & Detailed Metrics -- \\
  let vm_result = my_cpu.registers.read(0);
  assert_eq!(
    rust_result, 
    vm_result, 
    "Result mismatch: Rust native result ({}) != VM result ({})", 
    rust_result, 
    vm_result
  );

  let gap = average_vm_time as f32 / average_rust_time as f32;

  println!("=== Complete Assembler & VM Benchmark ===");
  println!("Fibonacci Index: {} | Sample Runs: {}", max, times);
  println!("-----------------------------------------");
  println!("Assembler compilation time: {}ns", assemble_time);
  println!("Rust native average time:   {}ns", average_rust_time);
  println!("VM execution average time:  {}ns", average_vm_time);
  println!("Computed Result:            {}.", vm_result);
  println!("Performance Gap (VM / Rust): {:.2}x\n", gap);
}

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
  let mut my_cpu = CPU::new(max * 8 + 1024);
  let program_text = format!("\
    mov x0, #0        // sum accumulator
    mov x1, #{}       // max limit
    mov x2, #0        // index i = 0

    // Populate array in memory: memory[i] = i + 1
    init_loop:
      cmp x2, x1
      bge init_done

      // x3 = i * 8 (byte offset)
      mul x3, x2, #8

      // x4 = i + 1
      add x4, x2, #1
      str x4, x3, #0    // Store value at memory[x3 + 0]

      add x2, x2, #1    // i++
      b init_loop

    init_done:
      mov x2, #0        // Reset index i = 0 for math loop
      mov x3, #0        // Sum accumulator = 0

    // Now do squaring and summing
    loop_start:
      cmp x2, x1
      bge loop_end

      // x4 = i * 8 (byte offset)
      mul x4, x2, #8

      // Load element from memory into x5
      ldr x5, x4, #0

      // Square the element: x5 = x5 * x5
      mul x5, x5, x5

      // Accumulate sum: x3 = x3 + x5
      add x3, x3, x5

      add x2, x2, #1    // i++
      b loop_start

    loop_end:
      mov x0, x3        // Move final sum to x0 for verification
      exit     
  ", max);
  
  let assemble_start = Instant::now();
  let program = assembler::assemble_program(&program_text);
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
    my_cpu.registers.read(0),
    "Rust sum {} != VM sum {}.",
    rust_array_sum,
    my_cpu.registers.read(0)
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

