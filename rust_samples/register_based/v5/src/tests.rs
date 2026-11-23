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

pub fn run_stack_test1(a: u64, b: u64) {
  let mut my_cpu = CPU::new(1024, 4096);
  let program = assemble_program(
    &format!(
      "
      main:
        mov x0, #{}
        mov x1, #{}
        bl add_two
        exit

      add_two:
        add x2, x0, x1
        mov x0, x2
        ret
      ",
      a, b
    )
  );

  my_cpu.reset();
  my_cpu.execute_program(&program.unwrap());

  println!("++ Stack Test 1 using Subroutines ++");
  println!("== {} + {} = ?? | Expected: {} ==", a, b, a + b);
  println!("VM output: {}\n", my_cpu.registers.x.read(0));
}

pub fn run_stack_test2(a: u64, b: u64) {
  let mut my_cpu = CPU::new(1024, 4096);
  let program = assemble_program(
    &format!(
      "
      main:
        mov x0, #{}
        mov x1, #{}

        bl add_ten
        mov x2, x0
        
        mov x0, x1
        bl add_ten
        
        mov x1, x0
        mov x0, x2        
        
        add x0, x0, x1
        exit

      add_ten:
        add x0, x0, #10
        ret
      ",
      a, b
    )
  );

  my_cpu.reset();
  my_cpu.execute_program(&program.unwrap());

  println!("++ Stack Test 2 using Subroutines & Frames ++");
  println!("== ({} + 10) + ({} + 10) = ?? | Expected: {} ==", a, b, a + b + 20);
  println!("VM output: {}\n", my_cpu.registers.x.read(0));
}

pub fn run_stack_test3(a: u64, b: u64) {
  let mut my_cpu = CPU::new(1024, 4096);
  let program = assemble_program(
    &format!(
      "
      main:
        mov x0, #{}   // a
        mov x1, #{}   // b
        bl outer
        exit
        
      // Suppose we reserve a 16-byte frame for `outer`:
      //   [sp + 0] : saved x30 (return address of outer)
      //   [sp + 8] : saved b (so add_ten can't clobber it)
      
      outer:
        // 1. Prologue: allocate 16 bytes & save x30 and x1 (b)
        subsp #16
        str x30, [sp + #0]
        str x1,  [sp + #8]
        
        // 2. Call add_ten(a): pass `a` in x0
        bl add_ten        // x0 becomes a + 10; x30 is overwritten with address after `bl`
        mov x2, x0        // keep (a + 10) in x2 for a moment
        
        // 3. Call add_ten(b): restore `b` from frame into x0
        ldr x0, [sp + #8]
        bl add_ten        // x0 becomes b + 20
        mov x1, x0
        
        // 4. Combine results: x0 = (a + 10) + (b + 20)
        add x0, x2, x1
        
        // 5. Epilogue: restore outer's return address & release frame
        ldr x30, [sp + #0]
        addsp #16
        ret
        
      add_ten:
        add x0, x0, #10   // x0 = x0 + 10
        ret               // returns to whoever `bl`'d add_ten
      ",
      a, b
    )
  );

  my_cpu.reset();
  my_cpu.execute_program(&program.unwrap());

  println!("++ Stack Test 3 using Subroutines & Frames ++");
  println!("== ({} + 10) + ({} + 10) = ?? | Expected: {} ==", a, b, a + b + 20);
  println!("VM output: {}\n", my_cpu.registers.x.read(0));
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
  run_array_squaring_test(100, 100);
  run_addition_test(1000, 1000);
  run_addition_test(10000, 1000);
  
  run_is_prime_test(1);
  run_is_prime_test(2);
  run_is_prime_test(3);
  run_is_prime_test(5);
  run_is_prime_test(12);
  run_is_prime_test(71);
  run_is_prime_test(120);

  run_stack_test1(10, 25);
  run_stack_test1(671, 822);
  
  run_stack_test2(7, 17);
  run_stack_test2(81, 733);
  
  run_stack_test3(5, 25);
  run_stack_test3(57, 25);

  run_fib_test1(5, 1);
  run_fib_test1(12, 1);
  run_fib_test1(5, 100);
  run_fib_test1(12, 100);
  
  run_fib_test2(5, 100);
  run_fib_test2(12, 100);
}
