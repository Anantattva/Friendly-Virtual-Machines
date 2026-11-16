// ।। ॐ नमः शिवाय ।। \\
// @date 5th November, 2026

mod cpu;
mod cpu_design;
mod ir;
mod syscalls;
mod tests;

fn main() {
  tests::run_addition_test(100, 100);
  tests::run_addition_test(1000, 1000);
  
  tests::run_pell_test(5, 100);
  tests::run_pell_test(12, 1000);
}