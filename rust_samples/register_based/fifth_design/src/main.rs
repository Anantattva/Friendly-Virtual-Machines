// ।। ॐ नमः शिवाय ।। 
// @date_start 7th November, 2026

#![allow(warnings)]

mod cpu;
mod architecture;
mod isa;
mod assembler;
mod tests;

fn main() {
  tests::run_array_squaring_test(100, 100);
  tests::run_addition_test(1000, 1000);
  tests::run_addition_test(10000, 1000);
}