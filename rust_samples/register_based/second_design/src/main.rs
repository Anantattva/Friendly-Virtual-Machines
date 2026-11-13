// ।। ॐ नमः शिवाय ।। 
// @date_start 26th October, 2026
// @date_end   26th October, 2026

mod cpu;
mod cpu_design;
mod isa;
mod tests;

fn main() {
  tests::run_heavy_addition_test(100, 1000);
  tests::run_cube_addition_test(5, 1000);
  tests::run_cube_addition_test(10, 1000);
  tests::run_cube_addition_test(20, 1000);
  tests::run_cube_addition_test(100, 1000);
}