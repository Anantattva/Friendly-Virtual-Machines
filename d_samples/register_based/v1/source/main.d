// ।। ॐ नमः शिवाय ।। \\
// @date_start 10th November, 2026

module main;

import std.stdio;
import tests;

void main() {
  writeln("======================================");
  writeln(" Running D Virtual Machine Test Suite ");
  writeln("======================================\n");

  runAdditionTest(1000, 100);
  runCubeAdditionTest(500, 100);
  runCollatzTest(27, 10);
  runFibonacciTest(20, 100);
  
  writeln("All benchmark suites completed successfully!");
}
