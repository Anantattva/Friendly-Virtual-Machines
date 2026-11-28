## <div align="center">⚡ FVM Rust Registers V1 ⚡</div>
```
    ██████╗ ██╗███████╗ ██████╗    ██╗   ██╗███╗   ███╗
    ██╔══██╗██║██╔════╝██╔════╝    ██║   ██║████╗ ████║
    ██████╔╝██║███████╗██║         ██║   ██║██╔████╔██║
    ██╔══██╗██║╚════██║██║         ╚██╗ ██╔╝██║╚██╔╝██║
    ██║  ██║██║███████║╚██████╗     ╚████╔╝ ██║ ╚═╝ ██║
    ╚═╝  ╚═╝╚═╝╚══════╝ ╚═════╝      ╚═══╝  ╚═╝     ╚═╝
```

A hand-crafted 64-bit RISC virtual machine (inspired by ARM64), written in pure Rust.
44 instructions. 31 general-purpose registers. Zero dependencies. One very fast interpreter.

---

> ।। ॐ नमः शिवाय ।।

---

### ✦ Overview

This is a minimal, from-scratch, ARM64-style bytecode interpreter implemented in Rust. It features a clean `fetch-decode-execute` pipeline, a `fixed-width 32-bit instruction encoding`, a full `ARM-style condition-flag model (N, Z, C, V)`, and a function pointer dispatch table for maximum interpreter throughput.

> *No crates. No runtime. No magic. Just u32 words, u64 registers, and a Vec<u8> that thinks it's RAM.*

**Designed for:**
- 🧠 Learning how CPUs, ISAs, and interpreters actually work
- ⚡ Benchmarking interpretation overhead against native Rust
- 🛠️ Hacking — write your own assembler, add instructions, build a compiler target
- 🕉️ Contemplation — every file begins with a mantra

---

### ✦ Architecture at a Glance

**Component Specification:**
- Word size: 64-bit
- Instruction width: 32-bit (fixed)
- Registers: x0–x30 (31 GPRs) + x31 = XZR (hardwired zero)
- Program counter: pc: u64
- Flags: Z (zero), N (negative), C (carry / !borrow), V (overflow)
- Link register: x30 (used by BL / RET)
- Memory: Flat Vec<u8>, little-endian
- Opcode: space 8 bits (256 possible, 44 implemented)
-Dispatch: Static function-pointer table 

---

### ✦ Instruction Set

All instructions are 32 bits wide, encoded little-endian. The opcode lives in the low 8 bits, with operand fields packed above it.

**Control**

Mnemonic Opcode Encoding Description
- EXIT 0x00 [0x00 \| 24 unused] Halt the VM
- B 0x05 [op \| target:24] Unconditional branch
- B.EQ 0x06 [op \| target:24] Branch if Z == 1
- B.NE 0x0A [op \| target:24] Branch if Z == 0
- B.LT 0x0B [op \| target:24] Branch if N != V
- B.LE 0x0C [op \| target:24] Branch if Z \|\| N != V
- B.GT 0x0D [op \| target:24] Branch if !Z && N == V
- B.GE 0x0E [op \| target:24] Branch if N == V
- BL 0x22 [op \| target:24] Branch with link (x30 = pc + 4)
- RET 0x23 [0x23 \| 24 unused] Return to x30

**Data Movement**

Mnemonic Opcode Encoding Description
- MOV 0x01 [op \| rd \| imm:16] Load 16-bit immediate into register
- MOVR 0x26 [op \| rd \| rn \| unused] Register-to-register move

**Arithmetic**

Mnemonic Opcode Encoding Description
- ADD 0x02 [op \| rd \| rn \| rm] rd = rn + rm
- SUB 0x03 [op \| rd \| rn \| rm] rd = rn - rm
- MUL 0x04 [op \| rd \| rn \| rm] rd = rn * rm
- DIV 0x12 [op \| rd \| rn \| rm] rd = rn / rm (panics on ÷0)
- REM 0x14 [op \| rd \| rn \| rm] rd = rn % rm (panics on ÷0)
- ADDI 0x0F [op \| rd \| rn \| imm:8] rd = rn + imm
- SUBI 0x10 [op \| rd \| rn \| imm:8] rd = rn - imm
- MULI 0x11 [op \| rd \| rn \| imm:8] rd = rn * imm
- DIVI 0x13 [op \| rd \| rn \| imm:8] rd = rn / imm
- REMI 0x15 [op \| rd \| rn \| imm:8] rd = rn % imm

**Logic & Bitwise**

Mnemonic Opcode Encoding Description
AND 0x16 [op \| rd \| rn \| rm] Bitwise AND
ORR 0x18 [op \| rd \| rn \| rm] Bitwise OR
EOR 0x1A [op \| rd \| rn \| rm] Bitwise XOR
ANDI 0x17 [op \| rd \| rn \| imm:8] Bitwise AND with immediate
ORRI 0x19 [op \| rd \| rn \| imm:8] Bitwise OR with immediate
EORI 0x1B [op \| rd \| rn \| imm:8] Bitwise XOR with immediate

**Shifts**

Mnemonic Opcode Encoding Description
- LSL 0x1C [op \| rd \| rn \| rm] Logical shift left by rm & 0x3F
- LSLI 0x1D [op \| rd \| rn \| imm:6] Logical shift left immediate
- LSR 0x1E [op \| rd \| rn \| rm] Logical shift right
- LSRI 0x1F [op \| rd \| rn \| imm:6] Logical shift right immediate
- ASR 0x20 [op \| rd \| rn \| rm] Arithmetic shift right (sign-preserving)
- ASRI 0x21 [op \| rd \| rn \| imm:6] Arithmetic shift right immediate

**Memory**

Mnemonic Opcode Encoding Description
- LDR 0x07 [op \| rd \| rn \| imm:8] Load 8 bytes from [rn + imm]
- STR 0x08 [op \| rt \| rn \| imm:8] Store 8 bytes to [rn + imm]
- LDRB 0x24 [op \| rd \| rn \| imm:8] Load single byte
- STRB 0x25 [op \| rt \| rn \| imm:8] Store single byte

**Comparison & Flag-Setting**

Mnemonic Opcode Encoding Description
- CMP 0x09 [op \| rn \| rm \| unused] Set flags from rn - rm
- CMPI 0x27 [op \| rn \| imm:16] Set flags from rn - imm
- ADDS 0x28 [op \| rd \| rn \| rm] ADD + set flags
- SUBS 0x29 [op \| rd \| rn \| rm] SUB + set flags
- ADDIS 0x2A [op \| rd \| rn \| imm:8] ADDI + set flags
- SUBIS 0x2B [op \| rd \| rn \| imm:8] SUBI + set flags

---

### ✦ The Flag Model (ARM-style)

All condition codes follow the classic ARM semantics:

**Flag Meaning Source**
- Z Result is zero result == 0
- N Result's MSB is 1 (result as i64) < 0
- C ADD: carry-out · SUB: no borrow overflowing_add().1 / !overflowing_sub().1
- V Signed overflow Operand/result sign mismatch

> This gives you the full set of comparison branches — EQ, NE, LT, LE, GT, GE — all derived cleanly from (N, Z, C, V).

---

### ✦ Quick Start

```rust
use crate::cpu::CPU;

let mut cpu = CPU::new(1024);  // 1 KB of RAM

// MOV x1, #42   →  0x01, 0x01, 0x2A, 0x00
// MOV x2, #10   →  0x01, 0x02, 0x0A, 0x00
// ADD x3, x1,x2 →  0x02, 0x03, 0x01, 0x02
let program = vec![
    0x01, 0x01, 0x2A, 0x00,
    0x01, 0x02, 0x0A, 0x00,
    0x02, 0x03, 0x01, 0x02,
];

cpu.execute_program(&program);

assert_eq!(cpu.registers.read(3), 52);  // 42 + 10
```

**A Loop That Counts**

```asm
        MOV  x0, #0        ; comparison stop = 0
        MOV  x1, #21       ; step value
        MOV  x2, #7        ; counter
        MOV  x3, #0        ; accumulator
        MOV  x4, #1        ; decrementer
loop:   ADD  x3, x3, x1    ; x3 += 21
        SUB  x2, x2, x4    ; x2 -= 1
        CMP  x2, x0        ; set flags
        B.EQ done          ; exit when zero
        B    loop
done:
```

After execution, x3 == 147 (21 × 7). ✅

---

### ✦ Performance Snapshot

Benchmarks comparing the VM against native Rust (same algorithms, same hardware):

**Workload VM/Rust Overhead**
- Sum 1..100 313 ns 2136 ns 6.8×
- Sum 1..200 469 ns 3750 ns 8.0×
- Sum of squares 1..100 261 ns 2604 ns 10.0×
+ Fibonacci(71) 208 ns 2292 ns 11.0×
- Pell(21) 52 ns 781 ns 15.0×

7–11× overhead is excellent for a pure-interpreter design with no JIT. This is the kind of gap you'd expect from a well-tuned bytecode VM — every instruction goes through a function-pointer indirection and a full decode.

> For context: execute_instruction does fetch → decode → dispatch → execute in ~20 ns on average, including branch prediction.

---

### ✦ Project Layout

```
src/
├── main.rs              ← entry point (currently runs tests)
├── cpu.rs               ← CPU struct, fetch / decode / execute / run loop
├── cpu_instructions.rs  ← all 44 instruction handlers + dispatch table
├── state.rs             ← Registers, Flags, State
└── tests.rs             ← regression tests + benchmarks
```

---

### ✦ Design Notes

**Compile-time dispatch / Function Pointer Dispatch Table**

The instruction table is a const array of function pointers:

```rust
pub const INSTRUCTION_HANDLERS: [InstructionFn; 44] = [
    exit, mov, add, sub, mul, b, beq, ldr, str, cmp, /* ... */
];
```

Decode is a single array index — no match, no vtable, no branch chain. Opcodes are sequential, so opcode maps directly to INSTRUCTION_HANDLERS[opcode].

**XZR is free**

Register index 31 is hardwired zero — reads return 0, writes are discarded. No special-casing at the instruction level; the Registers::read / write methods handle it.

**Safe-unsafe hygiene**

The register file and instruction fetch use get_unchecked for speed, but the bounds are guaranteed by the u8-wide register index and the execution loop's pc < program.len() guard. Every unsafe access is preceded by an invariant the compiler cannot see but the ISA guarantees.

**Branch detection**

Rather than have every branch instruction increment pc manually, the run loop does this:

```rust
old_pc = self.pc;
self.execute_instruction();
if old_pc == self.pc {
    self.pc += 4;
}
```

A branch leaves pc changed (to the target); a non-branch leaves it alone, and the loop advances pc. This eliminates a whole class of "forgot to advance the PC" bugs.

---

### ✦ Testing

```bash
cargo run --release
```

**Current tests exercise:**
- ✅ Arithmetic (ADD, SUB, MUL)
- ✅ Loops with conditional branches (B.EQ, B.NE)
- ✅ Memory round-trips (STR → LDR, byte variants)
- ✅ Recursive-style algorithms: Fibonacci and Pell numbers
- ✅ Cross-validation against native Rust implementations (asserted)

---

### ✦ License

MIT — do what you like, attribution appreciated.

---

<div align="center">

।। सर्वे भवन्तु सुखिनः ।।

May all beings be happy.

Happy Programming. 🦀⚙️🕉

</div>
