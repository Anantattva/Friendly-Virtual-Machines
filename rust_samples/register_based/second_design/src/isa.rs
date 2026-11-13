// ।। ॐ नमः शिवाय ।। 
// @date_start 26th October, 2026

use crate::cpu::CPU;

// ++ Instruction Set Architecture
// ++ Bytecode design

// ++ This file contains the bytecode instructions for our virtual machine.
// ++ The design is inspired by ARM64 Assembly.
// ++ This file aims to act as the ++`Single Source of Truth`++.

// ++==!! 🍁🍁 the core defining nature 🍁🍁 !!==++ \\
pub trait InstructionNature {
  fn hex(&self) -> u8;
  fn name_string(&self) -> &str;
  // fn execute_inst(&self, cpu: &mut CPU, instruction: u32);
  fn execute(&self, cpu: &mut CPU);
}



///// +++++++++++++++++++++++++++ \\\\\
// +++++++ DESIGNING THE ISA +++++++ \\
///// +++++++++++++++++++++++++++ \\\\\

/*
 * ++ THE MODEL ++
 * 
 * // ++ baseline ++
 * 0x00 -> exit
 * 0x01 -> movi
 * 0x02 -> movr
 * 
 * // ++ additions ++
 * 0x03 -> addi
 * 0x04 -> addr
 * 0x05 -> addis
 * 0x06 -> addrs
 * 
 * // ++ subtractions ++
 * 0x07 -> subi
 * 0x08 -> subr
 * 0x09 -> subis
 * 0x0A -> subrs
 * 
 * // ++ multiplications ++
 * 0x0B -> muli
 * 0x0C -> mulr
 * 
 * // ++ divisions ++
 * 0x0D -> divi
 * 0x0E -> divr
 * 
 * // ++ compare & set flags ++
 * 0x0F -> cmpi
 * 0x10 -> cmpr
 * 
 * // ++ branching ++
 * 0x11 -> b
 * 0x12 -> bl
 * 0x13 -> beq
 * 0x14 -> bne
 * 0x15 -> bgt
 * 0x16 -> blt
 * 0x17 -> bge
 * 0x18 -> ble
 * 
 * // ++ load/store ++
 * 0x19 -> ldr
 * 0x1A -> str
 * 0x1B -> ldrb
 * 0x1C -> strb
 * 
 * // ++ return ++
 * 0x1D -> ret
 */

// 0x00 -> exit
pub struct Exit;

impl InstructionNature for Exit {
  fn hex(&self) -> u8 {
    0x00
  }

  fn name_string(&self) -> &str {
    "exit"
  }

  fn execute(&self, cpu: &mut CPU) {
    cpu.state.is_running = false;
  }
}

// 0x01 -> movi
pub struct Movi {
  pub rd: usize,
  pub imm: u16,
}

impl InstructionNature for Movi {
  fn hex(&self) -> u8 {
    0x01
  }

  fn name_string(&self) -> &str {
    "movi"
  }

  fn execute(&self, cpu: &mut CPU) {
    cpu.registers.write(self.rd, self.imm as u64);
  }
}

// 0x02 -> movr
pub struct Movr {
  pub rd: usize,
  pub rn: usize,
}

impl InstructionNature for Movr {
  fn hex(&self) -> u8 {
    0x02
  }

  fn name_string(&self) -> &str {
    "movr"
  }

  fn execute(&self, cpu: &mut CPU) {
    cpu.registers.write(self.rd, cpu.registers.read(self.rn));
  }
}

// 0x03 -> addi
pub struct Addi {
  pub rd: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Addi {
  fn hex(&self) -> u8 {
    0x03
  }

  fn name_string(&self) -> &str {
    "addi"
  }

  fn execute(&self, cpu: &mut CPU) {
    let val = cpu.registers.read(self.rn).wrapping_add(self.imm as u64);
    cpu.registers.write(self.rd, val);
  }
}

// 0x04 -> addr
pub struct Addr {
  pub rd: usize,
  pub rn: usize,
  pub rm: usize,
}

impl InstructionNature for Addr {
  fn hex(&self) -> u8 {
    0x04
  }

  fn name_string(&self) -> &str {
    "addr"
  }

  fn execute(&self, cpu: &mut CPU) {
    let sum = cpu.registers.read(self.rn).wrapping_add(cpu.registers.read(self.rm));
    cpu.registers.write(self.rd, sum);
  }
}

// 0x05 -> addis
pub struct Addis {
  pub rd: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Addis {
  fn hex(&self) -> u8 {
    0x05
  }

  fn name_string(&self) -> &str {
    "addis"
  }

  fn execute(&self, cpu: &mut CPU) {
    let val1 = cpu.registers.read(self.rn);
    let imm_val = self.imm as u64;
    
    // << perform addition with flag tracking matching V1 addis/adds semantics >> \\
    let (sum, did_carry) = val1.overflowing_add(imm_val);

    cpu.flags.zero_flag = sum == 0;
    cpu.flags.negative_flag = (sum as i64) < 0;
    cpu.flags.carry_flag = did_carry;

    let val1_i = val1 as i64;
    let imm_i = imm_val as i64;
    let sum_i = sum as i64;
    cpu.flags.overflow_flag = (val1_i ^ imm_i) < 0 && (val1_i ^ sum_i) >= 0;

    cpu.registers.write(self.rd, sum);
  }
}

// 0x06 -> addrs (Register-Register Add with Flag Updates)
pub struct Addrs {
  pub rd: usize,
  pub rn: usize,
  pub rm: usize,
}

impl InstructionNature for Addrs {
  fn hex(&self) -> u8 {
    0x06
  }

  fn name_string(&self) -> &str {
    "addrs"
  }

  fn execute(&self, cpu: &mut CPU) {
    let val1 = cpu.registers.read(self.rn);
    let val2 = cpu.registers.read(self.rm);

    let (sum, did_carry) = val1.overflowing_add(val2);

    cpu.flags.zero_flag = sum == 0;
    cpu.flags.negative_flag = (sum as i64) < 0;
    cpu.flags.carry_flag = did_carry;

    let val1_i = val1 as i64;
    let val2_i = val2 as i64;
    let sum_i = sum as i64;
    cpu.flags.overflow_flag = (val1_i ^ val2_i) < 0 && (val1_i ^ sum_i) >= 0;

    cpu.registers.write(self.rd, sum);
  }
}

// 0x07 -> subi (Subtract Immediate)
pub struct Subi {
  pub rd: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Subi {
  fn hex(&self) -> u8 {
    0x07
  }

  fn name_string(&self) -> &str {
    "subi"
  }

  fn execute(&self, cpu: &mut CPU) {
    let val = cpu.registers.read(self.rn).wrapping_sub(self.imm as u64);
    cpu.registers.write(self.rd, val);
  }
}

// 0x08 -> subr (Subtract Register)
pub struct Subr {
  pub rd: usize,
  pub rn: usize,
  pub rm: usize,
}

impl InstructionNature for Subr {
  fn hex(&self) -> u8 {
    0x08
  }

  fn name_string(&self) -> &str {
    "subr"
  }

  fn execute(&self, cpu: &mut CPU) {
    let diff = cpu.registers.read(self.rn).wrapping_sub(cpu.registers.read(self.rm));
    cpu.registers.write(self.rd, diff);
  }
}

// 0x09 -> subis (Subtract Immediate & Set Flags)
pub struct Subis {
  pub rd: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Subis {
  fn hex(&self) -> u8 {
    0x09
  }

  fn name_string(&self) -> &str {
    "subis"
  }

  fn execute(&self, cpu: &mut CPU) {
    let val1 = cpu.registers.read(self.rn);
    let imm_val = self.imm as u64;

    // << perform subtraction with borrow tracking >> \\
    let (diff, did_borrow) = val1.overflowing_sub(imm_val);

    cpu.flags.zero_flag = diff == 0;
    cpu.flags.negative_flag = (diff as i64) < 0;
    cpu.flags.carry_flag = !did_borrow; // In ARM, C = 1 if NO borrow

    let val1_i = val1 as i64;
    let imm_i = imm_val as i64;
    let diff_i = diff as i64;
    cpu.flags.overflow_flag = (val1_i ^ imm_i) >= 0 && (val1_i ^ diff_i) < 0;

    cpu.registers.write(self.rd, diff);
  }
}

// 0x0A -> subrs (Subtract Register & Set Flags)
pub struct Subrs {
  pub rd: usize,
  pub rn: usize,
  pub rm: usize,
}

impl InstructionNature for Subrs {
  fn hex(&self) -> u8 {
    0x0A
  }

  fn name_string(&self) -> &str {
    "subrs"
  }

  fn execute(&self, cpu: &mut CPU) {
    let val1 = cpu.registers.read(self.rn);
    let val2 = cpu.registers.read(self.rm);

    // << perform subtraction with borrow tracking >> \\
    let (diff, did_borrow) = val1.overflowing_sub(val2);

    cpu.flags.zero_flag = diff == 0;
    cpu.flags.negative_flag = (diff as i64) < 0;
    cpu.flags.carry_flag = !did_borrow;

    let val1_i = val1 as i64;
    let val2_i = val2 as i64;
    let diff_i = diff as i64;
    cpu.flags.overflow_flag = (val1_i ^ val2_i) >= 0 && (val1_i ^ diff_i) < 0;

    cpu.registers.write(self.rd, diff);
  }
}

// 0x0B -> muli (Multiply Immediate)
pub struct Muli {
  pub rd: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Muli {
  fn hex(&self) -> u8 {
    0x0B
  }

  fn name_string(&self) -> &str {
    "muli"
  }

  fn execute(&self, cpu: &mut CPU) {
    let prod = cpu.registers.read(self.rn).wrapping_mul(self.imm as u64);
    cpu.registers.write(self.rd, prod);
  }
}

// 0x0C -> mulr (Multiply Register)
pub struct Mulr {
  pub rd: usize,
  pub rn: usize,
  pub rm: usize,
}

impl InstructionNature for Mulr {
  fn hex(&self) -> u8 {
    0x0C
  }

  fn name_string(&self) -> &str {
    "mulr"
  }

  fn execute(&self, cpu: &mut CPU) {
    let prod = cpu.registers.read(self.rn).wrapping_mul(cpu.registers.read(self.rm));
    cpu.registers.write(self.rd, prod);
  }
}

// 0x0D -> divi (Divide Immediate)
pub struct Divi {
  pub rd: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Divi {
  fn hex(&self) -> u8 {
    0x0D
  }

  fn name_string(&self) -> &str {
    "divi"
  }

  fn execute(&self, cpu: &mut CPU) {
    if self.imm == 0 {
      panic!("ZeroDivisionError: caught at divi.");
    }
    let quo = cpu.registers.read(self.rn).wrapping_div(self.imm as u64);
    cpu.registers.write(self.rd, quo);
  }
}

// 0x0E -> divr (Divide Register)
pub struct Divr {
  pub rd: usize,
  pub rn: usize,
  pub rm: usize,
}

impl InstructionNature for Divr {
  fn hex(&self) -> u8 {
    0x0E
  }

  fn name_string(&self) -> &str {
    "divr"
  }

  fn execute(&self, cpu: &mut CPU) {
    let divisor = cpu.registers.read(self.rm);
    if divisor == 0 {
      panic!("ZeroDivisionError: caught at divr.");
    }
    let quo = cpu.registers.read(self.rn).wrapping_div(divisor);
    cpu.registers.write(self.rd, quo);
  }
}

// 0x0F -> cmpi (Compare Immediate & Set Flags)
pub struct Cmpi {
  pub rn: usize,
  pub imm: u16,
}

impl InstructionNature for Cmpi {
  fn hex(&self) -> u8 {
    0x0F
  }

  fn name_string(&self) -> &str {
    "cmpi"
  }

  fn execute(&self, cpu: &mut CPU) {
    let val1 = cpu.registers.read(self.rn);
    let imm_val = self.imm as u64;

    // << perform subtraction with borrow tracking (val1 - imm) >> \\
    let (diff, did_borrow) = val1.overflowing_sub(imm_val);

    cpu.flags.zero_flag = diff == 0;
    cpu.flags.negative_flag = (diff as i64) < 0;
    cpu.flags.carry_flag = !did_borrow; // In ARM, C = 1 if NO borrow

    let val1_i = val1 as i64;
    let imm_i = imm_val as i64;
    let diff_i = diff as i64;
    cpu.flags.overflow_flag = (val1_i ^ imm_i) >= 0 && (val1_i ^ diff_i) < 0;
  }
}

// 0x10 -> cmpr (Compare Register & Set Flags)
pub struct Cmpr {
  pub rn: usize,
  pub rm: usize,
}

impl InstructionNature for Cmpr {
  fn hex(&self) -> u8 {
    0x10
  }

  fn name_string(&self) -> &str {
    "cmpr"
  }

  fn execute(&self, cpu: &mut CPU) {
    let val1 = cpu.registers.read(self.rn);
    let val2 = cpu.registers.read(self.rm);

    // << perform subtraction with borrow tracking (val1 - val2) >> \\
    let (diff, did_borrow) = val1.overflowing_sub(val2);

    cpu.flags.zero_flag = diff == 0;
    cpu.flags.negative_flag = (diff as i64) < 0;
    cpu.flags.carry_flag = !did_borrow;

    let val1_i = val1 as i64;
    let val2_i = val2 as i64;
    let diff_i = diff as i64;
    cpu.flags.overflow_flag = (val1_i ^ val2_i) >= 0 && (val1_i ^ diff_i) < 0;
  }
}

// 0x11 -> b (Unconditional Branch)
pub struct B {
  pub target_address: usize,
}

impl InstructionNature for B {
  fn hex(&self) -> u8 {
    0x11
  }

  fn name_string(&self) -> &str {
    "b"
  }

  fn execute(&self, cpu: &mut CPU) {
    cpu.pc = self.target_address;
  }
}

// 0x12 -> bl (Branch with Link)
pub struct Bl {
  pub target_address: usize,
}

impl InstructionNature for Bl {
  fn hex(&self) -> u8 {
    0x12
  }

  fn name_string(&self) -> &str {
    "bl"
  }

  fn execute(&self, cpu: &mut CPU) {
    // << save return address (current PC + 1) into register x30 (standard ARM link register) >> \\
    cpu.registers.write(30, (cpu.pc + 1) as u64);
    cpu.pc = self.target_address;
  }
}

// 0x13 -> beq (Branch if Equal / Zero Flag is true)
pub struct Beq {
  pub target_address: usize,
}

impl InstructionNature for Beq {
  fn hex(&self) -> u8 {
    0x13
  }

  fn name_string(&self) -> &str {
    "beq"
  }

  fn execute(&self, cpu: &mut CPU) {
    if cpu.flags.zero_flag {
      cpu.pc = self.target_address;
    }
  }
}

// 0x14 -> bne (Branch if Not Equal / Zero Flag is false)
pub struct Bne {
  pub target_address: usize,
}

impl InstructionNature for Bne {
  fn hex(&self) -> u8 {
    0x14
  }

  fn name_string(&self) -> &str {
    "bne"
  }

  fn execute(&self, cpu: &mut CPU) {
    if !cpu.flags.zero_flag {
      cpu.pc = self.target_address;
    }
  }
}

// 0x15 -> bgt (Branch if Greater Than - signed)
pub struct Bgt {
  pub target_address: usize,
}

impl InstructionNature for Bgt {
  fn hex(&self) -> u8 {
    0x15
  }

  fn name_string(&self) -> &str {
    "bgt"
  }

  fn execute(&self, cpu: &mut CPU) {
    if !cpu.flags.zero_flag && cpu.flags.negative_flag == cpu.flags.overflow_flag {
      cpu.pc = self.target_address;
    }
  }
}

// 0x16 -> blt (Branch if Less Than - signed)
pub struct Blt {
  pub target_address: usize,
}

impl InstructionNature for Blt {
  fn hex(&self) -> u8 {
    0x16
  }

  fn name_string(&self) -> &str {
    "blt"
  }

  fn execute(&self, cpu: &mut CPU) {
    if cpu.flags.negative_flag != cpu.flags.overflow_flag {
      cpu.pc = self.target_address;
    }
  }
}

// 0x17 -> bge (Branch if Greater than or Equal - signed)
pub struct Bge {
  pub target_address: usize,
}

impl InstructionNature for Bge {
  fn hex(&self) -> u8 {
    0x17
  }

  fn name_string(&self) -> &str {
    "bge"
  }

  fn execute(&self, cpu: &mut CPU) {
    if cpu.flags.negative_flag == cpu.flags.overflow_flag {
      cpu.pc = self.target_address;
    }
  }
}

// 0x18 -> ble (Branch if Less than or Equal - signed)
pub struct Ble {
  pub target_address: usize,
}

impl InstructionNature for Ble {
  fn hex(&self) -> u8 {
    0x18
  }

  fn name_string(&self) -> &str {
    "ble"
  }

  fn execute(&self, cpu: &mut CPU) {
    if cpu.flags.zero_flag || cpu.flags.negative_flag != cpu.flags.overflow_flag {
      cpu.pc = self.target_address;
    }
  }
}

// 0x19 -> ldr (Load Register 64-bit)
pub struct Ldr {
  pub rd: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Ldr {
  fn hex(&self) -> u8 {
    0x19
  }

  fn name_string(&self) -> &str {
    "ldr"
  }

  fn execute(&self, cpu: &mut CPU) {
    let base_address = cpu.registers.read(self.rn);
    let target_address = (base_address + self.imm as u64) as usize;

    // << read 8 bytes safely from memory >> \\
    let bytes: [u8; 8] = cpu.memory[target_address..target_address + 8]
      .try_into()
      .expect("Out of bounds error caught at `ldr`.");
    let value = u64::from_le_bytes(bytes);

    cpu.registers.write(self.rd, value);
  }
}

// 0x1A -> str (Store Register 64-bit)
pub struct Str {
  pub rt: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Str {
  fn hex(&self) -> u8 {
    0x1A
  }

  fn name_string(&self) -> &str {
    "str"
  }

  fn execute(&self, cpu: &mut CPU) {
    let base = cpu.registers.read(self.rn);
    let imm_val = self.imm as u64;
    let addr = base.checked_add(imm_val).expect("STR: address overflow") as usize;
    let end = addr.checked_add(8).expect("STR: address overflow");

    // << copy little-endian bytes into memory >> \\
    cpu.memory[addr..end].copy_from_slice(&u64::to_le_bytes(cpu.registers.read(self.rt)));
  }
}

// 0x1B -> ldrb (Load Register Byte)
pub struct Ldrb {
  pub rd: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Ldrb {
  fn hex(&self) -> u8 {
    0x1B
  }

  fn name_string(&self) -> &str {
    "ldrb"
  }

  fn execute(&self, cpu: &mut CPU) {
    let target_address = (cpu.registers.read(self.rn) + self.imm as u64) as usize;
    let value = cpu.memory[target_address] as u64;
    cpu.registers.write(self.rd, value);
  }
}

// 0x1C -> strb (Store Register Byte)
pub struct Strb {
  pub rt: usize,
  pub rn: usize,
  pub imm: u8,
}

impl InstructionNature for Strb {
  fn hex(&self) -> u8 {
    0x1C
  }

  fn name_string(&self) -> &str {
    "strb"
  }

  fn execute(&self, cpu: &mut CPU) {
    let value = cpu.registers.read(self.rt) as u8;
    let target_address = (cpu.registers.read(self.rn) + self.imm as u64) as usize;
    cpu.memory[target_address] = value;
  }
}

// 0x1D -> ret (return to caller)
pub struct Ret;

impl InstructionNature for Ret {
  fn hex(&self) -> u8 {
    0x1D
  }

  fn name_string(&self) -> &str {
    "ret"
  }

  fn execute(&self, cpu: &mut CPU) {
    cpu.pc = cpu.registers.read(30) as usize;
  }
}

pub enum ISACommands {
  Exit,
  Movi  { rd: usize, imm: u16 },
  Movr  { rd: usize, rn: usize },
  Addi  { rd: usize, rn: usize, imm: u8 },
  Addr  { rd: usize, rn: usize, rm: usize },
  Addis { rd: usize, rn: usize, imm: u8 },
  Addrs { rd: usize, rn: usize, rm: usize },
  Subi  { rd: usize, rn: usize, imm: u8 },
  Subr  { rd: usize, rn: usize, rm: usize },
  Subis { rd: usize, rn: usize, imm: u8 },
  Subrs { rd: usize, rn: usize, rm: usize },
  Muli  { rd: usize, rn: usize, imm: u8 },
  Mulr  { rd: usize, rn: usize, rm: usize },
  Divi  { rd: usize, rn: usize, imm: u8 },
  Divr  { rd: usize, rn: usize, rm: usize },
  Cmpi  { rn: usize, imm: u16 },
  Cmpr  { rn: usize, rm: usize },
  B     { target_address: usize },
  Bl    { target_address: usize },
  Beq   { target_address: usize },
  Bne   { target_address: usize },
  Bgt   { target_address: usize },
  Blt   { target_address: usize },
  Bge   { target_address: usize },
  Ble   { target_address: usize },
  Ldr   { rd: usize, rn: usize, imm: u8 },
  Str   { rt: usize, rn: usize, imm: u8 },
  Ldrb  { rd: usize, rn: usize, imm: u8 },
  Strb  { rt: usize, rn: usize, imm: u8 },
  Ret,
}

pub type Program = Vec<Box<dyn InstructionNature>>;