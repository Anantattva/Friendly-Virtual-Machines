// ।। ॐ नमः शिवाय ।। \\
// @date_start 10th November, 2026

module architecture;

import std.stdio;
import std.exception;

struct Memory {
  ubyte[] stack;
  ubyte[] heap;

  this(size_t stackSize, size_t heapSize) {
    stack = new ubyte[stackSize];
    heap = new ubyte[heapSize];
    stack[] = 0;
    heap[] = 0;
  }
}

struct Registers {
  ulong[32] x = 0;
  size_t sp = 0;
  size_t hp = 0;

  @safe ulong read(size_t index) {
    if (index == 31) return 0;
    enforce(index < 32, "InterpreterError: RegistersX out of bounds read.");
    return x[index];
  }

  @safe void write(size_t index, ulong newValue) {
    if (index == 31) return;
    enforce(index < 32, "InterpreterError: RegistersX out of bounds write.");
    x[index] = newValue;
  }
}

struct State {
  bool isRunning = false;
}