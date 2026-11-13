// ।। ॐ नमः शिवाय ।। 
// @date_start 28th October, 2026
// @date_end   7th November, 2026

mod cpu;
mod cpu_design;
mod isa;
mod tests;
mod assembler;
mod tests_assembler;

fn main() {
  tests::run_heavy_addition_test(100, 1000);
  // tests::run_cube_addition_test(5, 1000);
  // tests::run_cube_addition_test(10, 1000);
  // tests::run_cube_addition_test(20, 1000);
  tests::run_cube_addition_test(100, 1000);
  tests::run_collatz_test(17, 100);
  tests::run_bubble_sort_test();
  tests::run_matrix_multiplication_test(200);

  tests::run_fibonacci_test(10, 100);
  tests::run_fibonacci_test(15, 100);
  tests::run_fibonacci_test(25, 100);
  tests::run_fibonacci_test(25, 1000);

  tests_assembler::test_token_streamer();
  tests_assembler::run_complete_test(10, 100);
  tests_assembler::run_array_squaring_test(10, 100);
  tests_assembler::run_array_squaring_test(100, 100);
}