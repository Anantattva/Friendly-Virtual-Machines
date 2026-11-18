// ।। ॐ नमः शिवाय ।। \\
// @date_start 10th November, 2026

module cpu;

import std.stdio;
import std.exception;
import std.bitmanip;
import std.conv;

import architecture;
import isa;

class CPU {
  Memory memory;
  Registers registers;
  State state;
  size_t pc = 0;

  this(size_t stackSize, size_t heapSize) {
    memory = Memory(stackSize, heapSize);
    pc = 0;
  }

  void reset() {
    registers.x[] = 0;
    registers.sp = 0;
    registers.hp = 0;
    memory.stack[] = 0;
    memory.heap[] = 0;
    state.isRunning = false;
    pc = 0;
  }

  void executeProgram(ISACommand[] program) {
    state.isRunning = true;
    size_t oldPC = 0;
    while (pc < program.length && state.isRunning) {
      oldPC = pc;
      program[pc].execute(this);
      if (oldPC == pc) {
        pc++;
      }
    }
  }

  void storeStackU64(size_t offset, ulong val) {
    enforce(offset + 8 <= memory.stack.length, "InterpreterError: Stack overflow on store.");
    memory.stack[offset .. offset + 8] = nativeToLittleEndian(val);
  }

  ulong loadStackU64(size_t offset) {
    enforce(offset + 8 <= memory.stack.length, "InterpreterError: Stack overflow on load.");
    return littleEndianToNative!ulong(memory.stack[offset .. offset + 8].to!(ubyte[8]));
  }}