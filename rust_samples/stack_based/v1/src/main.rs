// ॐ श्री गणेशाय नम \\
// ।। ॐ नमः शिवाय ।। \\
// @date_start 17th November, 2026

mod vm;
mod bytecode;
mod tests;

fn main() {
  tests::run_addition_test(100, 100);
  tests::run_addition_test(1000, 1000);
  tests::run_fibonacci_test(10, 100);
  tests::run_fibonacci_test(25, 100);
}