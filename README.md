<div align="center">

<!-- Replace with your logo path once uploaded -->
<!-- <img src="assets/logo.png" width="220" alt="Friendly Virtual Machines Logo" /> -->

# ⚡ Friendly Virtual Machines ⚡
### *A Curated Collection of Diverse, Ultra-Fast, Miniature Virtual Machines*

[![License: MIT](https://img.shields.io/badge/License-MIT-purple.svg?style=for-the-badge)](LICENSE)
[![Core Language: Rust](https://img.shields.io/badge/Primary-Rust-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Footprint: < 7,000 LOC](https://img.shields.io/badge/Footprint-%3C_7%2C000_LOC-brightgreen.svg?style=for-the-badge)]()
[![Zero Dependencies](https://img.shields.io/badge/Dependencies-Zero-blue.svg?style=for-the-badge)]()

*No 500,000-line production monoliths. No trivial 50-line toy calculators.*  
*Just clean, readable, bare-metal-inspired execution engines you can hold in your head at once.*

[Explore the VMs](#-the-virtual-machines) • [Design Philosophy](#-design-philosophy) • [Quick Start](#-quick-start) • [Roadmap](#-roadmap) • [Contributing](#-contributing)

</div>

---

## 🎯 Why This Repository Exists

When learning how virtual machines, instruction sets, and bytecode interpreters actually work under the hood, developers usually hit a wall between two extremes:

1. **The Toy Script:** A 50-line blog tutorial that evaluates `2 + 2` on a stack, missing registers, flags, memory addressing, and real control flow.
2. **The Industrial Monolith:** Massive runtimes like V8, QEMU, or the JVM, where core execution loops are buried beneath millions of lines of JIT compilers, garbage collectors, and legacy infrastructure.

**Friendly Virtual Machines** bridges that gap. Every VM in this repository is a self-contained, high-velocity execution engine engineered to stay **strictly under 7,000 lines of code**—complete with custom instruction sets, assemblers, and head-to-head native benchmarks.

---

## 🧠 Design Philosophy

* **📖 Readability First:** Every module is cleanly separated (CPU state, register file, ISA semantics, assembler, tests suite) with clear architectural comments so you can trace every cycle from text to execution.
* **⚡ Engineered for Velocity:** Built with zero-cost abstractions, `#[inline(always)]` instruction dispatch loops, and deliberate low-level optimizations to minimize interpreter overhead.
* **📦 Zero External Bloat:** Pure standard library implementations. Clone, compile, and run offline in seconds with zero dependency trees.
* **📈 Evolutionary Progression:** Each sample builds upon the architectural lessons of the previous one—moving from foundational bytecode loops to full hardware-inspired register machines.

---

## 🏛️ The Virtual Machines

> *Each VM lives in its own isolated crate/directory with dedicated benchmarks and documentation.*

Currently, the repo only contains Rust VMs:
- [Rust samples](rust_samples)

*(Check individual sample folders for detailed ISA tables and deep-dive architecture notes.)*

---

## 🔬 Inside the Architecture (Example: `V3` ARM64-Inspired Engine)

To give you a taste of what sits inside these folders, **V3** models an **AArch64** processor in pure Rust:

```text
  Assembly Source (.asm)
         │
         ▼
 ┌───────────────────┐
 │  Pass 1: Lexer &     │  ──► Builds Symbol Table (maps `loop:`, `exit:` to PC indices)
 │   Label Scanner      │
 └─────────┬─────────┘
           ▼
 ┌───────────────────┐
 │  Pass 2: Command     │  ──► Emits typed `Vec<ISACommands>` with resolved branch targets
 │    Constructor       │
 └─────────┬─────────┘
           ▼
 ┌───────────────────┐
 │    CPU Engine        │  ──► x0–x30 + xzr (x31) | NZCV Flags   | Byte-Addressed RAM | Call Stack
 └───────────────────┘
