// ।। ॐ नमः शिवाय ।। \\
// @date_start 31st October, 2026

import * as ISA from './isa.js';
import { CPU } from './cpu.js';


export function runAdditionTest(max, times) {
  // << JS Baseline >> \\
  const startJS = process.hrtime.bigint();
  let sumJS = 0;
  for (let k = 0; k < times; k++) {
    sumJS = 0;
    for (let i = 1; i <= max; i++) {
      sumJS += i;
    }
  }
  const averageJSTimeNs = Math.round(Number((process.hrtime.bigint() - startJS) / BigInt(times)));

  // << My VM Benchmark >> \\
  const program = [
    new ISA.Movi(0, 0),    // sum
    new ISA.Movi(1, max),  // max
    new ISA.Movi(2, 0),    // sum index

    new ISA.Addi(2, 2, 1),
    new ISA.Addr(0, 0, 2),
    new ISA.Cmpr(1, 2),
    new ISA.Bne(3),

    new ISA.Exit()
  ];

  const myCPU = new CPU(1024);
  let startVM = process.hrtime.bigint();
  let totalVMTime = 0n;
  for (let i = 0; i < times; i++) {
    myCPU.reset();
    let startVM = process.hrtime.bigint();
    myCPU.executeProgram(program);
    totalVMTime += process.hrtime.bigint() - startVM;
  }
  const averageVMTimeNs = Math.round(Number((totalVMTime / BigInt(times))));
  const gap = Math.round(averageVMTimeNs / averageJSTimeNs);

  console.assert(BigInt(sumJS) === myCPU.registers.read(0), `JS Sum ${sumJS} != VM Sum ${myCPU.registers.read(0)}`);
  console.log("++ || Addition Test || ++");
  console.log(`== Max: ${max} | Times: ${times} ==`);
  console.log(`Sum: ${sumJS}`);
  console.log(`JS time:         ${averageJSTimeNs}ns.`);
  console.log(`VM time:         ${averageVMTimeNs}ns.`);
  console.log(`Gap (VM / JS):   ${gap}x.\n`);
}

export function runCollatzTest(num, times) {
  // -- JS Baseline -- \\
  const getCollatzSteps = (n) => {
    let steps = 0;
    while (n !== 1) {
      if (n % 2 === 0) {
        n /= 2;
      } else {
        n = 3 * n + 1;
      }
      steps++;
    }
    return steps;
  }

  let stepsJS = 0;
  const startJS = process.hrtime.bigint();
  for (let k = 0; k < times; k++) {
    stepsJS = getCollatzSteps(num);
  }
  const averageJSTimeNs = Number(process.hrtime.bigint() - startJS) / times;

  // -- VM Benchmark -- \\
  const myCPU = new CPU(1024);
  const program = [
    new ISA.Movi(0, 0),    // steps
    new ISA.Movi(1, num),  // num

    // -- base case handler: n == 1 -- \\
    new ISA.Cmpi(1, 1),
    new ISA.Beq(14),

    // -- remainder handler: n % 2 == 0 -- \\
    new ISA.Remi(2, 1, 2),
    new ISA.Cmpi(2, 0),
    new ISA.Beq(11),  // jump to even handler

    // -- odd handler: n = 3 * n + 1 -- \\
    new ISA.Muli(1, 1, 3),
    new ISA.Addi(1, 1, 1),
    new ISA.Addi(0, 0, 1),  // increment steps
    new ISA.B(2),

    // -- even handler: n /= 2 -- \\
    new ISA.Divi(1, 1, 2),
    new ISA.Addi(0, 0, 1),
    new ISA.B(2),

    new ISA.Exit()
  ];

  let startVM = process.hrtime.bigint();
  let totalVMTime = 0n;
  for (let i = 0; i < times; i++) {
    myCPU.reset();
    startVM = process.hrtime.bigint();
    myCPU.executeProgram(program);
    totalVMTime += process.hrtime.bigint() - startVM;
  }
  const averageVMTimeNs = Number(totalVMTime) / times;
  const gap = Math.round(averageVMTimeNs / averageJSTimeNs);

  console.assert(BigInt(stepsJS) === myCPU.registers.read(0), `JS Steps ${stepsJS} !== VM Steps ${myCPU.registers.read(0)}`);
  console.log("++ || Collatz Test || ++");
  console.log(`== Number: ${num} | Times: ${times} ==`);
  console.log(`Steps: ${stepsJS}`);
  console.log(`JS time:         ${averageJSTimeNs}ns.`);
  console.log(`VM time:         ${averageVMTimeNs}ns.`);
  console.log(`Gap (VM / JS):   ${gap}x.\n`);
}

export function runPrimeSumTest(index, times) {
  // -- JS Baseline -- \\
  const isPrime = (n) => {
    for (let k = 2; k <= n / 2; k++) {
      if (n % k === 0) {
        return false;
      }
    }
    return true;
  };

  const sumPrimesUpto = (count) => {
    let sum = 0;
    let prime = 2;
    for (let k = 0; k < count; k++) {
      while (!isPrime(prime)) {
        prime++;
      }
      sum += prime;
      prime++;
    }
    return sum;
  };

  let sumJS = 0;
  const startJS = process.hrtime.bigint();
  for (let k = 0; k < times; k++) {
    sumJS = sumPrimesUpto(index);
  }
  const averageJSTimeNs = Number(process.hrtime.bigint() - startJS) / times;

  // -- VM Benchmark -- \\
  // x0: sum | x1: found count | x2: target count (index) | x3: candidate prime
  // x4: divisor | x5: candidate / 2 | x6: remainder
  const myCPU = new CPU(1024);
  const program = [
    new ISA.Movi(0, 0),      // [0]  sum = 0
    new ISA.Movi(1, 0),      // [1]  count = 0
    new ISA.Movi(2, index),  // [2]  target = index
    new ISA.Movi(3, 2),      // [3]  candidate = 2

    // -- outer loop: stop once count == index -- \\
    new ISA.Cmpr(1, 2),      // [4]  cmp count, target
    new ISA.Beq(21),         // [5]  if count == target -> Exit

    // -- primality check loop for candidate -- \\
    new ISA.Movi(4, 2),      // [6]  divisor = 2
    new ISA.Divi(5, 3, 2),   // [7]  limit = candidate / 2
    new ISA.Cmpr(4, 5),      // [8]  cmp divisor, limit
    new ISA.Bgt(15),         // [9]  if divisor > limit -> prime found
    new ISA.Remr(6, 3, 4),   // [10] rem = candidate % divisor
    new ISA.Cmpi(6, 0),      // [11] cmp rem, 0
    new ISA.Beq(19),         // [12] if rem == 0 -> not prime
    new ISA.Addi(4, 4, 1),   // [13] divisor++
    new ISA.B(8),            // [14] next divisor

    // -- prime handler: accumulate sum & advance -- \\
    new ISA.Addr(0, 0, 3),   // [15] sum += candidate
    new ISA.Addi(1, 1, 1),   // [16] count++
    new ISA.Addi(3, 3, 1),   // [17] candidate++
    new ISA.B(4),            // [18] next prime slot

    // -- composite handler: try next candidate -- \\
    new ISA.Addi(3, 3, 1),   // [19] candidate++
    new ISA.B(6),            // [20] re-check primality

    new ISA.Exit()           // [21] halt
  ];

  let startVM = process.hrtime.bigint();
  let totalVMTime = 0n;
  for (let i = 0; i < times; i++) {
    myCPU.reset();
    startVM = process.hrtime.bigint();
    myCPU.executeProgram(program);
    totalVMTime += process.hrtime.bigint() - startVM;
  }
  const averageVMTimeNs = Number(totalVMTime) / times;
  const gap = Math.round(averageVMTimeNs / averageJSTimeNs);

  console.assert(BigInt(sumJS) === myCPU.registers.read(0), `JS Sum ${sumJS} !== VM Sum ${myCPU.registers.read(0)}`);
  console.log("++ || Prime Sum Test || ++");
  console.log(`== Index: ${index} | Times: ${times} ==`);
  console.log(`Sum: ${sumJS}`);
  console.log(`JS time:         ${averageJSTimeNs}ns.`);
  console.log(`VM time:         ${averageVMTimeNs}ns.`);
  console.log(`Gap (VM / JS):   ${gap}x.\n`);
}

export function runTests() {
  runAdditionTest(100, 1);
  runAdditionTest(200, 100);
  runAdditionTest(2000, 100);
  runAdditionTest(3500, 250);

  runCollatzTest(5, 1);
  runCollatzTest(25, 100);
  runCollatzTest(70, 100);
  runCollatzTest(101, 250);

  runPrimeSumTest(10, 1);
  runPrimeSumTest(25, 100);
  runPrimeSumTest(50, 100);
  runPrimeSumTest(100, 250);
}