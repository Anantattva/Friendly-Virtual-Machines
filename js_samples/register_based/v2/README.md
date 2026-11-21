<div align="center">
<h2>🕉️ FVM JS Register V2</h2>


*A hand‑crafted, register‑based virtual machine written in pure JavaScript.*
</div>

---

## 📖 Table of Contents
- [Overview](#-overview)
- [Architecture](#-architecture)
- [Instruction Set (ISA)](#-instruction-set-isa)
- [Assembly Design](#-assembly-design)
- [File Structure](#-file-structure)
- [Key Concepts](#-key-concepts)
- [Benchmarks](#-benchmarks)
- [Getting Started](#-getting-started)

---

## 🌌 Overview

This is a small, self‑contained register‑based virtual machine that interprets a flat array of numbers as bytecode. It ships with:
- A 31‑register file (with x31 hard‑wired to zero, MIPS‑style).
- A function pointer dispatch table for opcode → handler resolution.
- A hand‑written test suite benchmarking three workloads: arithmetic series, Collatz sequence, and prime‑sum generation — each compared against an equivalent native JS baseline.

The project is a study in how far you can push performance through careful dispatch, tight instruction packing, and avoiding branches inside the interpreter loop.

---

## 🏛 Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                          VM Instance                         │
│                                                              │
│  ┌─────────────┐   ┌──────────────┐   ┌──────────────────┐   │
│  │  Registers  │   │    Memory    │   │      State       │   │
│  │  x0 … x30   │   │  stack: u8[] │   │  isRunning: bool │   │
│  │  x31 = 0    │   │  heap:  u8[] │   │                  │   │
│  └─────────────┘   └──────────────┘   └──────────────────┘   │
│                                                              │
│  pc: instruction pointer    sp: stack pointer                │
│  dispatchTable: [opcode → handler(fn)]                       │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
              ┌────────────────────────────────┐
              │  executeProgram(program[])     │
              │  while (running && pc < len)   │
              │     dispatchTable[prog[pc]](…) │
              └────────────────────────────────┘
```

The Interpreter Loop

```js
executeProgram(program) {
  this.state.isRunning = true;
  while (this.state.isRunning && this.pc < program.length) {
    dispatchTable[program[this.pc]](this, program);
  }
}
```

Each handler receives the full VM and program, reads its operands directly from program[pc + 1 …], writes results, and advances pc itself. There is no fetch‑decode‑execute separation — the opcode doubles as an index into the dispatch table.

**Registers**
- x0 … x30 — general purpose, stored as a plain JS Array(31).
- x31 — hard‑wired zero. Reads always return 0, writes are silently ignored. This is a deliberate ARM‑inspired choice that simplifies branch‑free programming (e.g., bcrr against zero).
- Bounds checking emits a SegmentationFault diagnostic when an out‑of‑range index is used.

**Memory**
A minimal Memory class holds two Uint8Array buffers:
- stack — e.g. 1024 bytes.
- heap — e.g. 4096 bytes.

Note: The current instruction set does not yet expose load/store instructions — memory is reserved for future extensions (ld, st, stack frames, etc.).

---

## 🧩 Instruction Set (ISA)

All instructions are fixed‑width, flat‑array encoded. The first cell is the opcode; operands follow.

**Opcode Mnemonic Encoding Semantics**
- 0x00 exit [op] Halt execution.
- 0x01 movi rd, imm [op, rd, imm] rd ← imm
- 0x02 movr rd, rm [op, rd, rm] rd ← rm
- 0x03 cali rd, rm, imm, op [op, rd, rm, imm, opcode] rd ← rm ⟨op⟩ imm
- 0x04 calr rd, rm, rn, op [op, rd, rm, rn, opcode] rd ← rm ⟨op⟩ rn
- 0x05 b ta [op, target] Unconditional jump to ta.
- 0x06 bcri ta, rm, imm, cd [op, ta, rm, imm, cd] Branch if rm ⟨cd⟩ imm.
- 0x07 bcrr ta, rm, rn, cd [op, ta, rm, rn, cd] Branch if rm ⟨cd⟩ rn.

**Operation Codes (op)**
Code Operation
- 1 Add
- 2 Sub
- 3 Mul
- 4 Div
- 5 Rem

**Condition Codes (cd)**
Code Condition
- 1 eq (equal)
- 2 ne (not equal)
- 3 gt (greater than)
- 4 lt (less than)
- 5 ge (greater or equal)
- 6 le (less or equal)

The op and cd enums are exported so that test programs can express themselves declaratively — no magic numbers.

---

## 📜 Assembly Design

Programs are plain arrays. This keeps the bytecode trivial to construct, inspect, and serialize.

Example: Sum 1 … N

```js
const program = [
  inst.movi, 0x00, 0x00,           // r0 = 0                (sum)
  inst.movi, 0x01, max,            // r1 = max              (counter)
  inst.movi, 0x02, 0x00,           // r2 = 0                (i)

  inst.cali, 0x02, 0x02, 0x01, op.add,   // i = i + 1
  inst.calr, 0x00, 0x00, 0x02, op.add,   // sum = sum + i
  inst.bcrr, 0x09, 0x01, 0x02, cd.ne     // if max != i → loop
];
```

Reading top‑to‑bottom, each triple (or pair, or quintuple) is one instruction. The branch target 0x09 is an absolute program counter — line‑count the encoded words to find the loop head.

Encoding Rules at a Glance
- 3‑word instructions: movi, movr, b
- 5‑word instructions: cali, calr, bcri, bcrr
- 1‑word instruction: exit

Handlers advance pc by their encoded length. Branch instructions overwrite pc entirely — no implicit +1.

Idiomatic Conventions Used in Tests

Register Typical Role
- x0 accumulator / sum
- x1 loop bound / counter
- x2 loop index / divisor
- x3 … x6 temporaries

For the prime‑sum test, a richer ABI is used:

```
x0 = sum        x1 = target count     x2 = found count
x3 = candidate  x4 = limit (cand/2)   x5 = divisor
x6 = remainder
```

---

## 🗂 File Structure

```
isa.js          ← ISA definitions, opcodes, condition codes, handlers, dispatch table
vm.js           ← VM class: Registers, Memory, State, executeProgram loop
tests.js        ← Three benchmark suites + JS baselines + assertions
source.js       ← Entry point (calls runTests())
```

### isa.js
Contains every instruction handler as a named export. Each handler shares the signature (vm, program) and mutates vm.pc and vm.registers. Also exports:
- inst — opcode constants
- op — arithmetic opcode constants
- cd — condition‑code constants
- dispatchTable — indexed array of handlers

### vm.js
Defines Registers, Memory, State, and the VM class. The VM exposes:
- executeProgram(program) — interpreter loop
- reset() — clears registers + memory and resets pc / sp / state

### tests.js
Three workloads, each with:
- A pure JS baseline timed with process.hrtime.bigint().
- An equivalent VM program timed over the same number of repetitions.
- A console.assert verifying that the VM result matches the JS result.
- A printed report including the Gap (VM / JS) ratio.

### source.js
The entry point — just calls runTests().

---

### 💡 Key Concepts
- **Opcode‑as‑Dispatch‑Index:** The instruction pointer pc always points at an opcode. That opcode is directly used as an index into dispatchTable:

```js
dispatchTable[program[this.pc]](this, program);
```

No switch, no map lookup — a single array index. This is the fastest dispatch strategy available in JS.

- **Self‑Advancing PC:** Every handler is responsible for advancing pc by its own encoded width. Branch handlers assign pc directly. This means the interpreter loop has zero knowledge of instruction sizes — it simply trusts the handler.

- **Hard‑Wired Zero Register (x31):** Reads of x31 always yield 0; writes are discarded. This enables patterns like:

```js
inst.bcrr, target, 0x00, 0x1F, cd.eq   // compare against zero without a movi
```

It also mirrors the ARM64 zero register, a classic ISA‑design technique that eliminates many branch‑around‑move cases.

- **Fixed‑Width Encoding, Flat Bytecode:** Every instruction lives inside a number[]. There are no variable‑length variants, no headers, no sections. This makes programs trivially constructible in JS and equally trivial to serialize (JSON, Uint32Array, etc.).

- **Benchmark‑Driven Validation:** Every test both verifies correctness (via console.assert) and measures overhead against a native JS implementation. The reported Gap (VM / JS) ratio is a direct signal of interpreter cost per bytecode instruction — useful when experimenting with dispatch strategies.

- **Separation of Concerns:**
  - isa.js: Encoding + per‑instruction behavior
  - vm.js: State containers + interpreter loop
  - tests.js: Workload generation + benchmarking
  - source.js: Wiring

An ISA change touches exactly one file. A VM state change touches exactly one file. A new benchmark touches exactly one file.

- **Extensibility Path:** The current ISA is intentionally lean. Natural extensions:
  - Memory instructions — ld, st against memory.stack / memory.heap.
  - Stack ops — push, pop, call, ret using sp.
  - Bitwise ops — and, or, xor, shl, shr.
  - Compile‑to‑bytecode — an assembler that turns mnemonics into the numeric array.
  - Labels — two‑pass assembly to resolve symbolic branch targets.

Each new opcode is a single handler plus one entry in dispatchTable — no plumbing required.

---

## 📊 Benchmarks
`runTests()` executes three workloads at increasing scale, each repeated multiple times to average out JIT noise:

Workload Parameter Repetitions
- **Addition N** = 100, 200, 2000, 3500 1, 100, 100, 250, 250
- **Collatz n** = 5, 25, 70, 101 1, 100, 100, 250, 250
- **Prime Sum first N primes** = 10, 25, 50, 100 1, 100, 100, 250, 250

Sample Output

```

```

The gap is dominated by:

· One JS function call per VM instruction (dispatch overhead).
· Array bounds lookups for register reads/writes.
· No JIT‑friendly type feedback inside handlers (they're polymorphic across opcodes).

These are exactly the costs any tree‑walking interpreter pays — and exactly why bytecode JITs and threaded interpreters exist.

---

## 🚀 Getting Started

```bash
# Clone the repo, then:
node source.js
```

No dependencies. No build step. Pure Node.js.

Requirements:
- Node.js ≥ 16 (uses process.hrtime.bigint()).
- Or equivalently, Deno or Bun.
- A terminal to enjoy the benchmark output.

Writing Your First Program

```js
import { inst, op, cd } from "./isa.js";
import { VM } from "./vm.js";

// r0 = 10, r1 = 20, r0 = r0 + r1, exit
const program = [
  inst.movi, 0x00, 10,
  inst.movi, 0x01, 20,
  inst.calr, 0x00, 0x00, 0x01, op.add,
  inst.exit
];

const vm = new VM(1024, 4096);
vm.executeProgram(program);
console.log(vm.registers.read(0)); // → 30
```

---

## 🪔 Closing

> कर्मण्येवाधिकारस्ते मा फलेषु कदाचन।
"You have a right to your actions, but never to the fruits thereof."

This VM is not here to beat JavaScript — it exists to teach. Every instruction, every branch, every register index is a chance to see how machines think. Read the handlers, modify the opcodes, add a call/ret pair, and watch the gap shrink as you get closer to the metal.

> ॐ शान्तिः शान्तिः शान्तिः।
