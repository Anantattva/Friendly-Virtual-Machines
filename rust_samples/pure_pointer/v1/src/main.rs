// ।। ॐ नमः शिवाय ।। \\
// @date_start 12th November, 2026

// @learning @building @experimenting
// This is an experiment to craft a Virtual Machine in Rust
// Chronologically, its my 6th proper VM

// @design
// This VM aims to keep memory & processing same
// This VM has no registers - data is manipulated directly on memory via pointer access

#![allow(warnings)]

mod cpu;
mod ir;
mod tests;

fn main() {
  tests::run_addition_test(100, 100);
  tests::run_addition_test(100, 1000);
  tests::run_addition_test(1000, 1000);
  
  tests::run_fibonacci_test(3, 10);
  tests::run_fibonacci_test(10, 100);
  tests::run_fibonacci_test(25, 1000);
}