// ॐ श्री गणेशाय नम \\
// ।। ॐ नमः शिवाय ।। \\
// @date_atart 19th November, 2026

import { dispatchTable } from "./isa.js";

class Registers {
  constructor() {
    this.x = new Array(31).fill(0);
  }
  read(index) {
    if (index == 31) {
      return 0;
    } else if (index < 31) {
      return this.x[index];
    } else {
      console.error(`VMError::SegmentationFault: Caught at registers read! Invalid Bound: ${index}.`);
    }
  }
  write(index, newValue) {
    if (index == 31) {
      return;
    } else if (index < 31) {
      return this.x[index] = newValue;
    } else {
      console.error(`VMError::SegmentationFault: Caught at registers write! Invalid Bound: ${index}.`);
    }
  }
}

class Memory {
  constructor(stackSize, heapSize) {
    this.stack = new Uint8Array(stackSize).fill(0);
    this.heap = new Uint8Array(heapSize).fill(0);
  }
}

class State {
  constructor() {
    this.isRunning = false;
  }
}

export class VM {
  constructor(stackSize, heapSize) {
    this.registers = new Registers();
    this.memory = new Memory(stackSize, heapSize);
    this.state = new State();
    this.pc = 0;
    this.sp = 0;
  }
  executeProgram(program) {
    this.state.isRunning = true;
    while (this.state.isRunning && this.pc < program.length) {
      dispatchTable[program[this.pc]](this, program);
    }
  }
  clearMemory() {
    this.registers.x.fill(0);
    this.memory.stack.fill(0);
    this.memory.heap.fill(0);
  }
  reset() {
    this.clearMemory();
    this.state.isRunning = false;
    this.pc = 0;
    this.sp = 0;
  }
}