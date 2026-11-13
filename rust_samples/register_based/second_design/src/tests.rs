// ।। ॐ नमः शिवाय ।। 
// @date_start 26th October, 2026

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
    Box::new(isa::Movi  { rd: 0, imm: 0 }),
    Box::new(isa::Movi  { rd: 1, imm: 0 }),
    Box::new(isa::Movi  { rd: 2, imm: max }),
    Box::new(isa::Addi  { rd: 1,  rn: 1, imm: 1 }),
    Box::new(isa::Addr  { rd: 0,  rn: 0,  rm: 1 }),
    Box::new(isa::Cmpr  { rn: 2,  rm: 1 }),
    Box::new(isa::Bne   { target_address: 3 }),
    Box::new(isa::Exit),
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
    Box::new(isa::Movi  { rd: 0, imm: 0 }),
    Box::new(isa::Movi  { rd: 1, imm: 0 }),
    Box::new(isa::Movi  { rd: 2, imm: max as u16 }),
    Box::new(isa::Addi  { rd: 1,  rn: 1, imm: 1 }),
    Box::new(isa::Movr  { rd: 3,  rn: 1 }),
    Box::new(isa::Mulr  { rd: 3,  rn: 3, rm: 1}),
    Box::new(isa::Mulr  { rd: 3,  rn: 3, rm: 1}),
    Box::new(isa::Addr  { rd: 0,  rn: 0, rm: 3 }),
    Box::new(isa::Cmpr  { rn: 2,  rm: 1 }),
    Box::new(isa::Bne   { target_address: 3 }),
    Box::new(isa::Exit),
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