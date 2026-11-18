// ।। ॐ नमः शिवाय ।। \\
// @date_start 10th November, 2026

module tests;

import std.stdio;
import std.format;
import std.datetime.stopwatch;
import core.volatile : volatileLoad;

import cpu;
import isa;
import assembler;

void runAdditionTest(ulong max, int times) {
  // -- Native D Baseline -- \\
  ulong totalNativeTime = 0;
  ulong totalNativeSum = 0;
  
  auto sw = StopWatch(AutoStart.no);
  for (int i = 0; i < times; i++) {
    sw.start();
    totalNativeSum = 0;
    for (ulong j = 1; j <= max; j++) {
      totalNativeSum += volatileLoad(&j);
    }
    sw.stop();
    totalNativeTime += sw.peek.total!("nsecs");
    sw.reset();
  }
  ulong avgNativeTime = totalNativeTime / times;

  // -- D VM Benchmark -- \\
  auto cpu = new CPU(1024, 1024);
  ISACommand[] program = cast(ISACommand[])[
    new Movi(0, 0),
    new Movi(1, 0),
    new Movi(2, max),
    new Calci(1, 1, 1, Operation.Add),
    new Calcr(0, 0, 1, Operation.Add),
    new Bi(3, 1, max, Condition.Ne), // loops back to index 3
    new Exit()
  ];

  ulong totalVmTime = 0;
  for (int i = 0; i < times; i++) {
    cpu.reset();
    sw.start();
    cpu.executeProgram(program);
    sw.stop();
    totalVmTime += sw.peek.total!("nsecs");
    sw.reset();
  }
  ulong avgVmTime = totalVmTime / times;
  double gap = cast(double)avgVmTime / cast(double)avgNativeTime;

  writeln("++ Addition Test ++");
  writefln("== Max: %d | Times: %d", max, times);
  writefln("Sum: %d", totalNativeSum);
  writefln("Average D time:   %dns.", avgNativeTime);
  writefln("Average VM time:  %dns.", avgVmTime);
  writefln("Gap (VM / D):     %.2fx\n", gap);
}

void runCubeAdditionTest(ulong max, int times) {
  // -- Native D Baseline -- \\
  ulong totalNativeTime = 0;
  ulong nativeSum = 0;
  auto sw = StopWatch(AutoStart.no);

  for (int i = 0; i < times; i++) {
    sw.start();
    nativeSum = 0;
    for (ulong j = 1; j <= max; j++) {
      ulong val = volatileLoad(&j);
      nativeSum += val * val * val;
    }
    sw.stop();
    totalNativeTime += sw.peek.total!("nsecs");
    sw.reset();
  }
  ulong avgNativeTime = totalNativeTime / times;

  // -- D VM Benchmark -- \\
  ISACommand[] program = cast(ISACommand[])[
    new Movi(0, 0),
    new Movi(1, 0),
    new Movi(2, max),
    new Calci(1, 1, 1, Operation.Add),
    new Movr(3, 1),
    new Calcr(3, 3, 1, Operation.Mul),
    new Calcr(3, 3, 1, Operation.Mul),
    new Calcr(0, 0, 3, Operation.Add),
    new Bi(3, 1, max, Condition.Ne),
    new Exit()
  ];

  auto cpu = new CPU(1024, 1024);
  ulong totalVmTime = 0;
  for (int i = 0; i < times; i++) {
    cpu.reset();
    sw.start();
    cpu.executeProgram(program);
    sw.stop();
    totalVmTime += sw.peek.total!("nsecs");
    sw.reset();
  }
  ulong avgVmTime = totalVmTime / times;
  double gap = cast(double)avgVmTime / cast(double)avgNativeTime;

  writeln("++ Cube Addition Test ++");
  writefln("== Max: %d | Times: %d", max, times);
  writefln("Sum: %d", nativeSum);
  writefln("Average D time:   %dns.", avgNativeTime);
  writefln("Average VM time:  %dns.", avgVmTime);
  writefln("Gap (VM / D):     %.2fx\n", gap);
}

void runCollatzTest(ulong num, ulong times) {
  // -- Native D Baseline -- \\
  auto sw = StopWatch(AutoStart.no);
  sw.start();
  ulong steps = 0;
  for (ulong t = 0; t < times; t++) {
    ulong base = num;
    steps = 0;
    while (base != 1) {
      if (base % 2 == 0) {
        base /= 2;
      } else {
        base = 3 * volatileLoad(&base) + 1;
      }
      steps++;
    }
  }
  sw.stop();
  ulong avgNativeTime = sw.peek.total!("nsecs") / times;

  // -- D VM Benchmark -- \\
  auto cpu = new CPU(1024, 1024);
  auto program = assembleProgram(
    format(
      "
      mov x0, #%d  // num
      mov x1, #0   // step counter

      base:
        beq program_exit x0, #1

      rem x2, x0, #2
      beq odd_handler x2, #1

      even_handler:
        div x0, x0, #2
        add x1, x1, #1
        b base

      odd_handler:
        mul x0, x0, #3
        add x0, x0, #1
        add x1, x1, #1
        b base

      program_exit:
        exit
      "
    , num)
  );

  sw.reset();
  sw.start();
  for (ulong i = 0; i < times; i++) {
    cpu.reset();
    cpu.executeProgram(program);
  }
  sw.stop();
  ulong avgVmTime = sw.peek.total!("nsecs") / times;

  assert(cpu.registers.read(1) == steps, format("VM Steps %d != Native Steps %d", cpu.registers.read(1), steps));
  double gap = cast(double)avgVmTime / cast(double)avgNativeTime;

  writeln("++ Collatz Test ++");
  writefln("== Num: %d | Times: %d ==", num, times);
  writefln("Steps: %d", steps);
  writeln("D VM time:           ", avgVmTime, " ns.");
  writeln("Native time:         ", avgNativeTime, " ns.");
  writeln("Gap (VM / Native):   ", format("%.2f", gap), "x.\n");
}

void runFibonacciTest(ulong max, ulong times) {
  // -- Native D Baseline -- \\
  ulong fib(ulong n) {
    switch (n) {
      case 0: return 0;
      case 1: return 1;
      default: {
        ulong a = 0;
        ulong b = 1;
        ulong c = 1;

        for (ulong i = 2; i < n; i++) {
          a = volatileLoad(&c);
          c += volatileLoad(&b);
          b = volatileLoad(&a);
        }
        return c;
      }
    }
  }

  auto sw = StopWatch(AutoStart.yes);
  ulong fibNative = 0;
  for (ulong i = 0; i < times; i++) {
    fibNative = fib(max);
  }
  sw.stop();
  ulong avgNativeTime = sw.peek.total!("nsecs") / times;

  // -- D VM Benchmark -- \\
  auto cpu = new CPU(1024, 1024);
  ISACommand[] program = cast(ISACommand[])[
    new Movi(0, 0),
    new Movi(1, max - 1),
    new Movi(2, 0),
    new Movi(3, 1),
    new Movr(4, 3),
    new Movi(5, 0),
    new Bi(19, 1, 1, Condition.Le),
    // loop (index 8)
    new Calcr(3, 3, 2, Operation.Add),
    new Movr(2, 4),
    new Movr(4, 3),
    new Calci(5, 5, 1, Operation.Add),
    new Bi(8, 5, max - 1, Condition.Ne),
    new Movr(0, 3),
    new Exit(),
    // base case (index 19)
    new Movr(0, 1),
    new Exit()
  ];

  sw.reset();
  sw.start();
  for (ulong i = 0; i < times; i++) {
    cpu.reset();
    cpu.executeProgram(program);
  }
  sw.stop();
  ulong avgVmTime = sw.peek.total!("nsecs") / times;

  double gap = cast(double)avgVmTime / cast(double)avgNativeTime;
  writeln("++ Fibonacci Test ++");
  writeln("== Index: ", max, " | Times: ", times, " ==");
  writefln("Fib: %d.", fibNative);
  writeln("Native D average time:   ", avgNativeTime, " ns.");
  writeln("D VM average time:       ", avgVmTime, " ns.");
  writeln("Gap (VM / Native):       ", format("%.2f", gap), "x.\n");
}
