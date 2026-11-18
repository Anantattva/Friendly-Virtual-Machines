# ⚡ D Register-based VM V1

A lightweight, high-performance register-based virtual machine, custom assembler, and interpreter written from scratch in **D**, designed and executed entirely inside **Termux**. 

Inspired by ARM64 architectural patterns, this VM features an extensible Instruction Set Architecture (ISA), a robust two-pass symbol-resolving assembler, and a comprehensive benchmarking test suite comparing interpreter overhead directly against native compiled D code.

---

## 🚀 Performance Benchmarks

Executed on mobile hardware (Redmi 15 5G) via Termux, VM achieves remarkably tight performance gaps (as low as **5.66x** overhead) compared to native D execution:

```text
======================================
 Running D Virtual Machine Test Suite
======================================

++ Addition Test ++
== Max: 1000 | Times: 100
Sum: 500500
Average D time:   4102ns.
Average VM time:  23225ns.
Gap (VM / D):     5.66x

++ Cube Addition Test ++
== Max: 500 | Times: 100
Sum: 15687562500
Average D time:   2135ns.
Average VM time:  27743ns.
Gap (VM / D):     12.99x

++ Collatz Test ++
== Num: 27 | Times: 10 ==
Steps: 111
D VM time:           7080 ns.
Native time:         400 ns.
Gap (VM / Native):   17.70x.

++ Fibonacci Test ++
== Index: 20 | Times: 100 ==
Fib: 6765.
Native D average time:   111 ns.
D VM average time:       788 ns.
Gap (VM / Native):       7.10x.

All benchmark suites completed successfully!
➜  DlangVMV1
```

## 🛠️ Architectural Design
 * Register File (architecture.d): 32 general-purpose 64-bit registers (x0 through x31), where register x31 is hardwired to 0 for RISC-style optimization. Includes split stack and heap memory allocations.
 * Polymorphic ISA (isa.d): Clean object-oriented instruction pattern implementing an ISACommand interface with modular execution handlers for data movement, arithmetic (add, sub, mul, div, rem), stack operations, and conditional branching.
 * Two-Pass Assembler (assembler.d): Strips comments, tokenizes assembly instructions, builds a symbol table for label resolution, and compiles human-readable assembly text directly into executable VM bytecode.
 * Execution Core (cpu.d): Employs an efficient fetch-execute instruction pointer loop with safe stack management and little-endian memory conversion routines.

```
📂 Project Structure
├── architecture.d  # Core memory structures, registers, and execution state
├── isa.d           # Instruction Set Architecture definitions and command classes
├── assembler.d     # Two-pass text assembler and symbol table label resolver
├── cpu.d           # CPU execution loop, program counter, and stack operations
├── tests.d         # Comprehensive benchmark suite (Addition, Cube, Collatz, Fibonacci)
└── main.d          # Entry point for running the test suite
```

## 💻 Getting Started & Running
Make sure you have a D compiler (like dmd or ldc2) installed (available via pkg in Termux or your native environment). Compile and run the project using Dub or direct compilation, such as:
>```dub run -b release```

## 📜 Example Assembly Code
Here is a sample of how assembly code looks in DlangVM's custom syntax (demonstrated in the Collatz benchmark):

```
mov x0, #27  // Initialize starting number
mov x1, #0   // Initialize step counter

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
```
