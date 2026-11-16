<div align="center">

# ⚡ FVM-V3: ARM64-Inspired Register Virtual Machine
### *High-Velocity Bytecode Execution & Two-Pass Assembler in Pure Rust (<1,500 LOC)*

[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Architecture: ARM64 Subset](https://img.shields.io/badge/Architecture-ARM64_Subset-blue.svg?style=for-the-badge)]()
[![Footprint: < 1,500 LOC](https://img.shields.io/badge/Footprint-%3C1%2C500_LOC-brightgreen.svg?style=for-the-badge)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-purple.svg?style=for-the-badge)](LICENSE)

*Part of the **[Friendly-Virtual-Machines](https://github.com/Anantattva/Friendly-Virtual-Machines)** collection — readable, self-contained, bare-metal-inspired execution engines without the monolith bloat.*

</div>

---

## ✨ Overview

**FVM-V3** is a compact, blazingly fast register-based virtual machine modeled after **ARM64 (AArch64)**. Built from the ground up with zero external dependencies, it pairs a **two-pass text assembler** (complete with symbolic label resolution) with an **inline-optimized fetch-decode-execute loop**.

Unlike toy stack calculators or 500,000-line production runtimes, V3 sits in the sweet spot: **every single architectural mechanism—registers, condition flags, memory addressing, call stacks, and assembly parsing—is transparent, hackable, and readable in one sitting.**

---

## 🏛️ Architectural Highlights

* **31 General-Purpose 64-Bit Registers (`x0`–`x30`) + Hardware Zero (`xzr` / `x31`):**
  * Reading from `x31` always yields `0`; writing to `x31` is discarded at zero cost—just like real AArch64 silicon.
* **Full ARM Condition Flags (`Z`, `N`, `C`, `V`):**
  * Arithmetic flag-setting variants (`adds`, `subs`) and comparisons (`cmp`) track **Zero**, **Negative**, **Carry**, and **Signed Overflow** flags using Rust's `overflowing_add` / `overflowing_sub`.
* **Two-Pass Symbolic Assembler:**
  * **Pass 1:** Strips comments (`//`), tokenizes instructions, and maps labels (`loop:`, `end_label:`) into a symbol table.
  * **Pass 2:** Resolves branch targets (`b`, `bl`, `beq`, `bne`, `bgt`, `blt`, `bge`, `ble`) directly into instruction indices.
* **Zero-Cost Performance Hatches:**
  * Uses `#[inline(always)]` on the instruction dispatch engine and `unsafe` `get_unchecked` indexing on register reads/writes and the main execution loop to minimize interpreter overhead.
* **Subroutine Call Stack (`bl` / `ret`):**
  * Dedicated return-address stack frame tracking (`stack_line`) for clean function calls and returns.

---

## 📜 Supported Instruction Set (ISA)

| Category | Mnemonics | Syntax Example | Description |
| :--- | :--- | :--- | :--- |
| **Data Movement** | `mov` | `mov x0, #42` / `mov x0, x1` | Load 16-bit immediate or copy register |
| **Arithmetic** | `add`, `sub`, `mul`, `div`, `rem` | `add x0, x1, #8` / `mul x2, x0, x1` | Integer arithmetic (register & immediate) |
| **Flag Arithmetic** | `adds`, `subs` | `adds x0, x1, x2` | Arithmetic that updates `Z`, `N`, `C`, `V` flags |
| **Comparison** | `cmp` | `cmp x2, #10` / `cmp x7, x8` | Sets condition flags via `rn - op2` without storing result |
| **Branching** | `b`, `beq`, `bne`, `bgt`, `blt`, `bge`, `ble` | `bne loop_start` | Unconditional & signed conditional jumps via labels or indices |
| **Subroutines** | `bl`, `ret` | `bl compute` / `ret` | Branch with link (pushes `pc + 1`) & return from subroutine |
| **Memory Access** | `ldr`, `str`, `ldrb`, `strb` | `ldr x5, x4, #0` / `str x8, x6, #8` | 64-bit word (`u64` little-endian) & byte (`u8`) load/store |
| **System** | `exit` | `exit` | Halts CPU execution cleanly  |

---

## 🗂️ Project Structure

```text
├── src/
│   ├── main.rs             # Entry point & benchmark runner suite 
│   ├── cpu.rs              # CPU struct, memory buffer, program counter & execution loop 
│   ├── cpu_design.rs       # Register file (x0-x30 + xzr), NZCV Flags & CPU State
│   ├── isa.rs              # ISACommands enum & #[inline(always)] instruction semantics 
│   ├── assembler.rs        # Tokenizer, symbol table builder & two-pass label resolver 
│   ├── tests.rs            # Raw ISA benchmarks (Collatz, Bubble Sort, 4x4 Matrix Mul)
│   └── tests_assembler.rs  # End-to-end assembly text compilation & execution benchmarks
└── Cargo.toml
