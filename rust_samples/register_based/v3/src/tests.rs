// ।। ॐ नमः शिवाय ।। 
// @date_start 28th October, 2026

#![allow(warnings)]

use crate::cpu::CPU;
use crate::isa;

use std::hint::black_box;
use std::time::Instant;

pub fn run_heavy_addition_test(max: u16, times: u32) {
  // -- Rust baseline -- \\
  let mut total_rust_time: u128 = 0;
  let mut start_rust = Instant::now();
  let mut time_rust = 0;
  let mut total_rust_sum = black_box(0);
  for _ in 0..times {
    total_rust_sum = 0;
    start_rust = Instant::now();
    for i in 1..=max {
      total_rust_sum += black_box(i);
    }
    time_rust = start_rust.elapsed().as_nanos();
    total_rust_time += time_rust;
  }
  let average_rust_time = (total_rust_time) / (times as u128);
  println!("Average time for Rust to add upto {} across {} runs: {}ns.", max, times, average_rust_time);

  // -- My VM -- \\
  let mut my_cpu = CPU::new(1024);
  let program: isa::Program = vec![
    isa::ISACommands::Movi  { rd: 0, imm: 0 },
    isa::ISACommands::Movi  { rd: 1, imm: 0 },
    isa::ISACommands::Movi  { rd: 2, imm: max },
    isa::ISACommands::Addi  { rd: 1,  rn: 1, imm: 1 },
    isa::ISACommands::Addr  { rd: 0,  rn: 0,  rm: 1 },
    isa::ISACommands::Cmpr  { rn: 2,  rm: 1 },
    isa::ISACommands::Bne   { target_address: 3 },
    isa::ISACommands::Exit,
  ];
  let mut total_vm_time: u128 = 0;
  let mut start_vm = Instant::now();
  let mut time_vm = 0;
  for _ in 0..times {
    my_cpu.reset();
    start_vm = Instant::now();
    
    my_cpu.execute_program(&program);
    
    time_vm = start_vm.elapsed().as_nanos();
    total_vm_time += time_vm;
  }
  let average_vm_time = (total_vm_time) / (times as u128);
  let gap = (average_vm_time as f64) / (average_rust_time as f64);
  println!("Sum upto {} by my VM: {}.", max, my_cpu.registers.read(0));
  println!("Average time for my VM to add upto {} across {} runs: {}ns.", max, times, average_vm_time);
  println!("Gap between my VM & Rust: {}x.", gap);
}

pub fn run_cube_addition_test(max: u32, times: u128) {
  // -- Rust baseline -- \\
  let mut start_rust = Instant::now();
  let mut time_rust = start_rust.elapsed().as_nanos();
  let mut total_rust_time: u128 = 0;
  let mut rust_sum = 0;
  for _ in 0..times {
    start_rust = Instant::now();
    rust_sum = 0;
    for i in 1..=max {
      rust_sum += black_box(i) * black_box(i) * black_box(i);
    }
    time_rust = start_rust.elapsed().as_nanos();
    total_rust_time += time_rust;
  }
  let average_rust_time = total_rust_time / times;
  println!("Rust cube sum upto {}: {}.", max, rust_sum);
  println!("Average Rust time across {} runs: {}ns.", times, average_rust_time);

  // -- VM Benchmark -- \\
  let program: isa::Program = vec![
    isa::ISACommands::Movi  { rd: 0, imm: 0 },
    isa::ISACommands::Movi  { rd: 1, imm: 0 },
    isa::ISACommands::Movi  { rd: 2, imm: max as u16 },
    isa::ISACommands::Addi  { rd: 1,  rn: 1, imm: 1 },
    isa::ISACommands::Movr  { rd: 3,  rn: 1 },
    isa::ISACommands::Mulr  { rd: 3,  rn: 3, rm: 1 },
    isa::ISACommands::Mulr  { rd: 3,  rn: 3, rm: 1 },
    isa::ISACommands::Addr  { rd: 0,  rn: 0, rm: 3 },
    isa::ISACommands::Cmpr  { rn: 2,  rm: 1 },
    isa::ISACommands::Bne   { target_address: 3 },
    isa::ISACommands::Exit,
  ];
  let mut my_cpu = CPU::new(1024);
  let mut start_vm = Instant::now();
  let mut time_vm = start_vm.elapsed().as_nanos();
  let mut total_vm_time: u128 = 0;
  for _ in 0..times {
    my_cpu.reset();
    start_vm = Instant::now();
    my_cpu.execute_program(&program);
    time_vm = start_vm.elapsed().as_nanos();
    total_vm_time += time_vm;
  }
  let average_vm_time = total_vm_time / times;
  let gap = (average_vm_time as f64) / (average_rust_time as f64);
  println!("VM cube sum upto {}: {}.", max, my_cpu.registers.read(0));
  println!("Average VM time across {} runs: {}ns.", times, average_vm_time);
  println!("Gap between my VM & Rust: {}x", gap);
}

pub fn run_collatz_test(num: u64, times: u128) {
  // -- Rust baseline -- \\
  let mut base = num;
  let mut steps = 0;
  let start_rust = Instant::now();
  for _ in 0..times {
    base = black_box(num);
    steps = black_box(0);
    while base != 1 {
      match base % 2 {
        0 => base /= black_box(2),
        _ => base = black_box(3) * black_box(base) + black_box(1),
      };
      steps += 1;
    }
  }
  let average_rust_time = start_rust.elapsed().as_nanos() / times;

  // -- VM Benchmark -- \\
  let mut my_cpu = CPU::new(1024);
  let program: isa::Program = vec![
    // -- times while loop setup -- \\
    isa::ISACommands::Movi  { rd: 2, imm: 0 },             // times

    // -- collatz reset -- \\
    isa::ISACommands::Movi  { rd: 0, imm: num as u16 },    // base num
    isa::ISACommands::Movi  { rd: 1, imm: 0 },             // steps
    
    // -- collatz chain loop -- \\
    isa::ISACommands::Remi  { rd: 3, rn: 0, imm: 2 },
    isa::ISACommands::Cmpi  { rn: 3, imm: 0 },
    isa::ISACommands::Bne   { target_address: 9 },

    // -- even handler -- \\
    isa::ISACommands::Divi  { rd: 0, rn: 0, imm: 2 },
    isa::ISACommands::Addi  { rd: 1, rn: 1, imm: 1 },
    isa::ISACommands::B     { target_address: 12 },

    // -- odd handler -- \\
    isa::ISACommands::Muli  { rd: 0, rn: 0, imm: 3 },
    isa::ISACommands::Addi  { rd: 0, rn: 0, imm: 1 },
    isa::ISACommands::Addi  { rd: 1, rn: 1, imm: 1 },

    // -- iterate collatz -- \\
    isa::ISACommands::Cmpi  { rn: 0, imm: 1 },
    isa::ISACommands::Bne   { target_address: 3 },

    // -- increment times -- \\
    isa::ISACommands::Addi  { rd: 2, rn: 2, imm: 1 },
    isa::ISACommands::Cmpi  { rn: 2, imm: times as u16 },
    isa::ISACommands::Bne   { target_address: 1 },

    // -- reset --
    isa::ISACommands::Exit, 
  ];

  let start_vm = Instant::now();
  my_cpu.execute_program(&program);
  let average_vm_time = start_vm.elapsed().as_nanos() / times;

  assert_eq!(
    my_cpu.registers.read(1),
    steps,
    "VM Steps {} != Rust Steps {}.",
    my_cpu.registers.read(1),
    steps,
  );

  let gap = (average_vm_time as f64) / (average_rust_time as f64);
  println!("Collatz chain {} across {} runs.", num, times);
  println!("VM time: {}ns.", average_vm_time);
  println!("Rust time: {}ns.", average_rust_time);
  println!("Gap VM / Rust: {}x.", gap);
}

pub fn run_bubble_sort_test() {
  let initial_data: [u64; 5] = [42, 7, 19, 3, 99];

  // -- Rust baseline -- \\
  let mut rust_data = black_box(initial_data);
  let start_rust = Instant::now();
  rust_data.sort();
  let rust_time = start_rust.elapsed().as_nanos();
  println!("Rust sorted array: {:?}, time: {}ns.", rust_data, rust_time);

  // -- VM Benchmark -- \\
  let mut my_cpu = CPU::new(1024);

  // Load initial array into VM memory starting at byte address 0
  for (i, &val) in initial_data.iter().enumerate() {
    let addr = i * 8;
    my_cpu.memory[addr..addr + 8].copy_from_slice(&val.to_le_bytes());
  }

  // Bubble sort ISA program
  // Registers:
  // r0 = base address (0)
  // r1 = N (5)
  // r2 = outer index (i)
  // r3 = inner index (j)
  // r4 = outer limit (N - 1)
  // r5 = inner limit ((N - 1) - i)
  // r6 = memory address of arr[j]
  // r7 = arr[j]
  // r8 = arr[j+1]
  let program: isa::Program = vec![
    /* 0 */ isa::ISACommands::Movi  { rd: 0, imm: 0 },       // r0 = base = 0
    /* 1 */ isa::ISACommands::Movi  { rd: 1, imm: 5 },       // r1 = N = 5
    /* 2 */ isa::ISACommands::Movi  { rd: 2, imm: 0 },       // r2 = i = 0
    /* 3 */ isa::ISACommands::Movr  { rd: 4, rn: 1 },       // r4 = N
    /* 4 */ isa::ISACommands::Subi  { rd: 4, rn: 4, imm: 1 },// r4 = outer_limit = N - 1
    
    // -- Outer Loop Header -- \\
    /* 5 */ isa::ISACommands::Cmpr  { rn: 2, rm: 4 },       // Compare i vs outer_limit
    /* 6 */ isa::ISACommands::Bge   { target_address: 24 },  // If i >= outer_limit, exit
    /* 7 */ isa::ISACommands::Movr  { rd: 5, rn: 4 },       // r5 = outer_limit
    /* 8 */ isa::ISACommands::Subr  { rd: 5, rn: 5, rm: 2 },// r5 = inner_limit = (N - 1) - i
    /* 9 */ isa::ISACommands::Movi  { rd: 3, imm: 0 },       // r3 = j = 0

    // -- Inner Loop Header -- \\
    /* 10 */ isa::ISACommands::Cmpr { rn: 3, rm: 5 },      // Compare j vs inner_limit
    /* 11 */ isa::ISACommands::Bge  { target_address: 22 },  // If j >= inner_limit, jump to outer increment
    /* 12 */ isa::ISACommands::Muli { rd: 6, rn: 3, imm: 8 },// r6 = j * 8 (byte offset)
    /* 13 */ isa::ISACommands::Addr { rd: 6, rn: 6, rm: 0 },// r6 = base + byte offset
    /* 14 */ isa::ISACommands::Ldr  { rd: 7, rn: 6, imm: 0 },// r7 = arr[j]
    /* 15 */ isa::ISACommands::Ldr  { rd: 8, rn: 6, imm: 8 },// r8 = arr[j+1]
    /* 16 */ isa::ISACommands::Cmpr { rn: 7, rm: 8 },      // Compare arr[j] and arr[j+1]
    /* 17 */ isa::ISACommands::Ble  { target_address: 20 },  // If arr[j] <= arr[j+1], skip swap
    
    // -- Swap Elements -- \\
    /* 18 */ isa::ISACommands::Str  { rt: 8, rn: 6, imm: 0 },// arr[j] = r8
    /* 19 */ isa::ISACommands::Str  { rt: 7, rn: 6, imm: 8 },// arr[j+1] = r7

    // -- Inner Loop Increment -- \\
    /* 20 */ isa::ISACommands::Addi { rd: 3, rn: 3, imm: 1 },// j++
    /* 21 */ isa::ISACommands::B    { target_address: 10 },  // Repeat inner loop

    // -- Outer Loop Increment -- \\
    /* 22 */ isa::ISACommands::Addi { rd: 2, rn: 2, imm: 1 },// i++
    /* 23 */ isa::ISACommands::B    { target_address: 5 },   // Repeat outer loop

    // -- Exit -- \\
    /* 24 */ isa::ISACommands::Exit,
  ];

  let start_vm = Instant::now();
  my_cpu.execute_program(&program);
  let vm_time = start_vm.elapsed().as_nanos();

  // Read sorted values back from VM memory
  let mut vm_sorted_data = [0u64; 5];
  for (i, val) in vm_sorted_data.iter_mut().enumerate() {
    let addr = i * 8;
    let bytes: [u8; 8] = my_cpu.memory[addr..addr + 8].try_into().unwrap();
    *val = u64::from_le_bytes(bytes);
  }

  let gap = (vm_time as f64) / (rust_time as f64);
  println!("VM sorted array:    {:?}, time: {}ns.", vm_sorted_data, vm_time);
  println!("Gap VM / Rust: {}x,", gap);
  assert_eq!(vm_sorted_data, rust_data, "VM sorting result mismatch!");
  println!("Bubble sort test passed successfully!\n");
}

pub fn run_matrix_multiplication_test(times: u128) {
  let size = 4;
  let mat_a: [u64; 16] = [
    1, 2, 3, 4,
    5, 6, 7, 8,
    9, 10, 11, 12,
    13, 14, 15, 16,
  ];
  let mat_b: [u64; 16] = [
    1, 0, 0, 0,
    0, 1, 0, 0,
    0, 0, 1, 0,
    0, 0, 0, 1,
  ];

  let mut rust_c = [0u64; 16];
  let start_rust = Instant::now();
  for _ in 0..times {
    rust_c.fill(0);
    for i in 0..size {
      for j in 0..size {
        for k in 0..size {
          rust_c[i * size + j] += black_box(mat_a[i * size + k]) * black_box(mat_b[k * size + j]);
        }
      }
    }
  }
  let average_rust_time = start_rust.elapsed().as_nanos() / times;

  let program: isa::Program = vec![
    /* 0  */ isa::ISACommands::Movi  { rd: 0, imm: 0 },       // Base A = 0
    /* 1  */ isa::ISACommands::Movi  { rd: 1, imm: 128 },     // Base B = 128
    /* 2  */ isa::ISACommands::Movi  { rd: 2, imm: 256 },     // Base C = 256
    /* 3  */ isa::ISACommands::Movi  { rd: 6, imm: 4 },       // size = 4
    /* 4  */ isa::ISACommands::Movi  { rd: 3, imm: 0 },       // i = 0 (outer index)

    // -- Outer Loop Header (i) -- [Index 5]
    /* 5  */ isa::ISACommands::Cmpr  { rn: 3, rm: 6 },
    /* 6  */ isa::ISACommands::Bge   { target_address: 38 },  // Exit outer loop -> Jump to Exit (38)

    /* 7  */ isa::ISACommands::Movi  { rd: 4, imm: 0 },       // j = 0 (middle index)

    // -- Middle Loop Header (j) -- [Index 8]
    /* 8  */ isa::ISACommands::Cmpr  { rn: 4, rm: 6 },
    /* 9  */ isa::ISACommands::Bge   { target_address: 35 },  // Exit middle loop -> Jump to i++ (35)

    /* 10 */ isa::ISACommands::Movi  { rd: 7, imm: 0 },       // accum = 0

    // -- Inner Loop Header (k) -- [Index 11]
    /* 11 */ isa::ISACommands::Movi  { rd: 5, imm: 0 },       // k = 0 (inner index)
    /* 12 */ isa::ISACommands::Cmpr  { rn: 5, rm: 6 },
    /* 13 */ isa::ISACommands::Bge   { target_address: 28 },  // ✔️ Fixed: Jump to store C[i][j] (28)

    // Offset A = (i * 4 + k) * 8
    /* 14 */ isa::ISACommands::Mulr  { rd: 8, rn: 3, rm: 6 }, 
    /* 15 */ isa::ISACommands::Addr  { rd: 8, rn: 8, rm: 5 }, 
    /* 16 */ isa::ISACommands::Muli  { rd: 8, rn: 8, imm: 8 }, 
    /* 17 */ isa::ISACommands::Addr  { rd: 8, rn: 8, rm: 0 }, 
    /* 18 */ isa::ISACommands::Ldr   { rd: 8, rn: 8, imm: 0 }, // r8 = A[i][k]

    // Offset B = (k * 4 + j) * 8
    /* 19 */ isa::ISACommands::Mulr  { rd: 9, rn: 5, rm: 6 }, 
    /* 20 */ isa::ISACommands::Addr  { rd: 9, rn: 9, rm: 4 }, 
    /* 21 */ isa::ISACommands::Muli  { rd: 9, rn: 9, imm: 8 }, 
    /* 22 */ isa::ISACommands::Addr  { rd: 9, rn: 9, rm: 1 }, 
    /* 23 */ isa::ISACommands::Ldr   { rd: 9, rn: 9, imm: 0 }, // r9 = B[k][j]

    // accum += A[i][k] * B[k][j]
    /* 24 */ isa::ISACommands::Mulr  { rd: 8, rn: 8, rm: 9 },
    /* 25 */ isa::ISACommands::Addr  { rd: 7, rn: 7, rm: 8 },

    /* 26 */ isa::ISACommands::Addi  { rd: 5, rn: 5, imm: 1 }, // k++
    /* 27 */ isa::ISACommands::B     { target_address: 12 },   // Loop back to inner check

    // -- Store C[i][j] = accum -- [Index 28]
    /* 28 */ isa::ISACommands::Mulr  { rd: 8, rn: 3, rm: 6 },
    /* 29 */ isa::ISACommands::Addr  { rd: 8, rn: 8, rm: 4 },
    /* 30 */ isa::ISACommands::Muli  { rd: 8, rn: 8, imm: 8 },
    /* 31 */ isa::ISACommands::Addr  { rd: 8, rn: 8, rm: 2 },
    /* 32 */ isa::ISACommands::Str   { rt: 7, rn: 8, imm: 0 },

    /* 33 */ isa::ISACommands::Addi  { rd: 4, rn: 4, imm: 1 }, // j++
    /* 34 */ isa::ISACommands::B     { target_address: 8 },    // Loop back to middle check

    /* 35 */ isa::ISACommands::Addi  { rd: 3, rn: 3, imm: 1 }, // i++
    /* 36 */ isa::ISACommands::B     { target_address: 5 },    // Loop back to outer check

    /* 37 */ isa::ISACommands::Exit,                          // [Index 37]
    /* 38 */ isa::ISACommands::Exit,                          // [Index 38] Safe exit landing
  ];

  let mut my_cpu = CPU::new(2048);
  let mut total_vm_time: u128 = 0;

  for _ in 0..times {
    my_cpu.reset();
    for (idx, &val) in mat_a.iter().enumerate() {
      let addr = idx * 8;
      my_cpu.memory[addr..addr + 8].copy_from_slice(&val.to_le_bytes());
    }
    for (idx, &val) in mat_b.iter().enumerate() {
      let addr = 128 + (idx * 8);
      my_cpu.memory[addr..addr + 8].copy_from_slice(&val.to_le_bytes());
    }

    let start_vm = Instant::now();
    my_cpu.execute_program(&program);
    total_vm_time += start_vm.elapsed().as_nanos();
  }

  let average_vm_time = total_vm_time / times;
  let mut vm_c = [0u64; 16];
  for (idx, val) in vm_c.iter_mut().enumerate() {
    let addr = 256 + (idx * 8);
    let bytes: [u8; 8] = my_cpu.memory[addr..addr + 8].try_into().unwrap();
    *val = u64::from_le_bytes(bytes);
  }

  assert_eq!(vm_c, rust_c, "Matrix multiplication mismatch!");
  let gap = (average_vm_time as f64) / (average_rust_time as f64);

  println!("=== Matrix Multiplication Benchmark ===");
  println!("Average Rust time across {} runs: {}ns", times, average_rust_time);
  println!("Average VM time across {} runs:   {}ns", times, average_vm_time);
  println!("Gap (VM / Rust): {:.2}x\n", gap);
}

pub fn run_fibonacci_test(max: u16, times: u128) {
  // -- Rust Baseline -- \\
  fn fib(num: u64) -> u64 {
    if num <= 1 {
      return num;
    } else {
      return fib(num - 1) + fib(num - 2);
    }
  }

  let start_rust = Instant::now();
  let mut fib_rust: u64 = 0;
  for _ in 0..times {
    fib_rust = black_box(fib(max as u64));
  }
  let average_rust_time = start_rust.elapsed().as_nanos() / times;

  // -- My VM Benchmark -- \\
  let mut my_cpu = CPU::new(1024);
  let program = vec![
    isa::ISACommands::Movi { rd: 0, imm: 0 },
    isa::ISACommands::Movi { rd: 1, imm: max - 1 },

    isa::ISACommands::Movi { rd: 2, imm: 0 },
    isa::ISACommands::Movi { rd: 3, imm: 1 },
    isa::ISACommands::Movr { rd: 4,  rn: 3 },
    isa::ISACommands::Movi { rd: 5, imm: 0 },
    
    isa::ISACommands::Cmpi { rn: 1, imm: 1 },
    isa::ISACommands::Ble  { target_address: 19 },

    isa::ISACommands::Addr { rd: 3, rn: 3, rm: 2 },
    isa::ISACommands::Movr { rd: 2, rn: 4 },
    isa::ISACommands::Movr { rd: 4, rn: 3 },
    isa::ISACommands::Addi { rd: 5, rn: 5, imm: 1 },
    
    isa::ISACommands::Cmpr { rn: 5, rm: 1 },
    isa::ISACommands::Bne  { target_address: 8 },

    isa::ISACommands::Movr { rd: 0, rn: 3 },
    isa::ISACommands::Exit,
    
    isa::ISACommands::Movr { rd: 0, rn: 1 },
    isa::ISACommands::Exit,
  ];

  let start_vm = Instant::now();
  for _ in 0..times {
    my_cpu.reset();
    my_cpu.execute_program(&program);
  }
  let average_vm_time = start_vm.elapsed().as_nanos() / times;

  assert_eq!(
    fib_rust,
    my_cpu.registers.read(0),
    "{} != {}",
    fib_rust,
    my_cpu.registers.read(0)
  );

  let gap = average_vm_time as f32 / average_rust_time as f32;

  println!("== Fibonacci Index: {} | Times: {} ==", max, times);
  println!("Rust average time: {}ns.", average_rust_time);
  println!("VM average time: {}ns.", average_vm_time);
  println!("Gap VM / Rust: {}x.", gap);

  // Caveat:
  // Rust uses O(2^n) complexity
  // VM uses O(n) complexity
  // Which is why the VM can occasionally beat native Rust
}