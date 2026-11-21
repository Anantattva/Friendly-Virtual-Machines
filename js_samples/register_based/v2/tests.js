// ॐ श्री गणेशाय नम \\
// ।। ॐ नमः शिवाय ।। \\
// @date_start 20th November, 2026

import { op, cd, inst } from "./isa.js";
import { VM } from "./vm.js";

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
    inst.movi, 0x00, 0x00,
    inst.movi, 0x01, max,
    inst.movi, 0x02, 0x00,

    inst.cali, 0x02, 0x02, 0x01, op.add,
    inst.calr, 0x00, 0x00, 0x02, op.add,
    inst.bcrr, 0x09, 0x01, 0x02, cd.ne
  ];

  const myVM = new VM(1024, 4096);
  let startVM = process.hrtime.bigint();
  let totalVMTime = 0n;
  for (let i = 0; i < times; i++) {
    myVM.reset();
    startVM = process.hrtime.bigint();
    myVM.executeProgram(program);
    totalVMTime += process.hrtime.bigint() - startVM;
  }
  const averageVMTimeNs = Math.round(Number((totalVMTime / BigInt(times))));
  const gap = Math.round(averageVMTimeNs / averageJSTimeNs);

  console.assert(sumJS === myVM.registers.read(0), `JS Sum ${sumJS} != VM Sum ${myVM.registers.read(0)}`);
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
  const myVM = new VM(1024, 4096);
  const program = [
    inst.movi, 0x00, 0x00,  // steps
    inst.movi, 0x01, num,   // n

    // -- check n == 1 -- \\
    inst.bcri, 0x32, 0x01, 0x01, cd.eq,
    
    // -- check n % 2 == 0 -- \\
    inst.cali, 0x02, 0x01, 0x02, op.rem,
    inst.bcri, 0x21, 0x02, 0x01, cd.eq,

    // -- even handler -- \\
    inst.cali, 0x01, 0x01, 0x02, op.div,
    inst.cali, 0x00, 0x00, 0x01, op.add,
    inst.b,    0x06,

    // -- odd handler -- \\
    inst.cali, 0x01, 0x01, 0x03, op.mul,
    inst.cali, 0x01, 0x01, 0x01, op.add,
    inst.cali, 0x00, 0x00, 0x01, op.add,
    inst.b,    0x06,

    inst.exit
  ];

  let startVM = process.hrtime.bigint();
  let totalVMTime = 0n;
  for (let i = 0; i < times; i++) {
    myVM.reset();
    startVM = process.hrtime.bigint();
    myVM.executeProgram(program);
    totalVMTime += process.hrtime.bigint() - startVM;
  }
  const averageVMTimeNs = Number(totalVMTime) / times;
  const gap = Math.round(averageVMTimeNs / averageJSTimeNs);

  console.assert(stepsJS === myVM.registers.read(0), `JS Steps ${stepsJS} !== VM Steps ${myVM.registers.read(0)}`);
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
  const myVM = new VM(1024, 4096);
  const program = [
    inst.movi, 0x00, 0x00,   // [0x00] sum (r0 = 0)
    inst.movi, 0x01, index,  // [0x03] target count (r1 = index)
    inst.movi, 0x02, 0x00,   // [0x06] found count (r2 = 0)
    inst.movi, 0x03, 0x02,   // [0x09] candidate prime (r3 = 2)

    // -- primality check -- \\
    inst.cali, 0x04, 0x03, 0x02, op.div,   // [0x0C] limit (r4 = r3 / 2)
    inst.movi, 0x05, 0x02,                 // [0x11] divisor (r5 = 2)
    inst.bcrr, 0x31, 0x05, 0x04, cd.gt,    // [0x14] divisor > limit -> prime found
    inst.calr, 0x06, 0x03, 0x05, op.rem,   // [0x19] rem = r3 % r5
    inst.bcri, 0x2A, 0x06, 0x00, cd.eq,    // [0x1E] rem == 0 -> not prime
    inst.cali, 0x05, 0x05, 0x01, op.add,   // [0x23] divisor++
    inst.b,    0x14,                       // [0x28] next divisor
    inst.cali, 0x03, 0x03, 0x01, op.add,   // [0x2A] candidate++
    inst.b,    0x0C,                       // [0x2F] re-check primality

    // -- prime sum & advance -- \\
    inst.calr, 0x00, 0x00, 0x03, op.add,   // [0x31] sum += candidate
    inst.cali, 0x03, 0x03, 0x01, op.add,   // [0x36] candidate++
    inst.cali, 0x02, 0x02, 0x01, op.add,   // [0x3B] count++

    // -- outer sum loop -- \\
    inst.bcrr, 0x0C, 0x01, 0x02, cd.gt,    // [0x40] if target > count -> next prime
    inst.exit                              // [0x45] halt
  ];
  
  let startVM = process.hrtime.bigint();
  let totalVMTime = 0n;
  for (let i = 0; i < times; i++) {
    myVM.reset();
    startVM = process.hrtime.bigint();
    myVM.executeProgram(program);
    totalVMTime += process.hrtime.bigint() - startVM;
  }
  const averageVMTimeNs = Number(totalVMTime) / times;
  const gap = Math.round(averageVMTimeNs / averageJSTimeNs);

  console.assert(sumJS === myVM.registers.read(0), `JS Sum ${sumJS} !== VM Sum ${myVM.registers.read(0)}`);
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
  runAdditionTest(3500, 250);

  
  runCollatzTest(5, 1);
  runCollatzTest(25, 100);
  runCollatzTest(70, 100);
  runCollatzTest(101, 250);
  runCollatzTest(101, 250);

  runPrimeSumTest(10, 1);
  runPrimeSumTest(25, 100);
  runPrimeSumTest(50, 100);
  runPrimeSumTest(100, 250);
  runPrimeSumTest(100, 250); 
}
