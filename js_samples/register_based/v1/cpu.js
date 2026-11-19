// ।। ॐ नमः शिवाय ।। \\
// @date_start 31st October, 2026

import { Registers, Flags, State } from './state.js';

export class CPU {
  constructor(constructionSize) {
    this.memory = new Uint8Array(constructionSize);
    this.registers = new Registers();
    this.flags = new Flags();
    this.pc = 0;          // program counter
    this.sp = BigInt(0);  // stack pointer
    this.state = new State();
  }

  executeProgram(program) {
    this.state.isRunning = true;
    while (this.state.isRunning && this.pc < program.length) {
      this.oldPC = this.pc;
      program[this.pc].execute(this);

      if (this.oldPC == this.pc) {
        this.pc++;
      }
    }
  }

  clearMemory() {
    this.memory.fill(0);
    this.registers.x.fill(0n);
  }

  reset() {
    this.clearMemory();
    this.flags.N = false;
    this.flags.Z = false;
    this.flags.C = false;
    this.flags.V = false;
    this.pc = 0;
    this.sp = 0;
    this.state.isRunning = false;
  }
}