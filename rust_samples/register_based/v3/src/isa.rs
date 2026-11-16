// ।। ॐ नमः शिवाय ।। 
// @date_start 26th October, 2026

use crate::cpu::CPU;

#[derive(Debug, Clone)]
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
  Remi  { rd: usize, rn: usize, imm: u8 },
  Remr  { rd: usize, rn: usize, rm: usize },
}


pub type Program = Vec<ISACommands>;

// !! CPU execution function !! \\
impl ISACommands {
  #[inline(always)]
  pub fn execute(&self, cpu: &mut CPU) {
    match self {
      ISACommands::Exit => {
        cpu.state.is_running = false;
      }
      ISACommands::Movi { rd, imm } => {
        cpu.registers.write(*rd, *imm as u64);
      }
      ISACommands::Movr { rd, rn } => {
        let val = cpu.registers.read(*rn);
        cpu.registers.write(*rd, val);
      }
      ISACommands::Addi { rd, rn, imm } => {
        let val = cpu.registers.read(*rn).wrapping_add(*imm as u64);
        cpu.registers.write(*rd, val);
      }
      ISACommands::Addr { rd, rn, rm } => {
        let val1 = cpu.registers.read(*rn);
        let val2 = cpu.registers.read(*rm);
        cpu.registers.write(*rd, val1.wrapping_add(val2));
      }
      ISACommands::Addis { rd, rn, imm } => {
        let val1 = cpu.registers.read(*rn);
        let imm_val = *imm as u64;
        let (sum, did_carry) = val1.overflowing_add(imm_val);
        cpu.flags.zero_flag = sum == 0;
        cpu.flags.negative_flag = (sum as i64) < 0;
        cpu.flags.carry_flag = did_carry;
        let val1_i = val1 as i64;
        let imm_i = imm_val as i64;
        let sum_i = sum as i64;
        cpu.flags.overflow_flag = (val1_i ^ imm_i) < 0 && (val1_i ^ sum_i) >= 0;
        cpu.registers.write(*rd, sum);
      }
      ISACommands::Addrs { rd, rn, rm } => {
        let val1 = cpu.registers.read(*rn);
        let val2 = cpu.registers.read(*rm);
        let (sum, did_carry) = val1.overflowing_add(val2);
        cpu.flags.zero_flag = sum == 0;
        cpu.flags.negative_flag = (sum as i64) < 0;
        cpu.flags.carry_flag = did_carry;
        let val1_i = val1 as i64;
        let val2_i = val2 as i64;
        let sum_i = sum as i64;
        cpu.flags.overflow_flag = (val1_i ^ val2_i) < 0 && (val1_i ^ sum_i) >= 0;
        cpu.registers.write(*rd, sum);
      }
      ISACommands::Subi { rd, rn, imm } => {
        let val = cpu.registers.read(*rn).wrapping_sub(*imm as u64);
        cpu.registers.write(*rd, val);
      }
      ISACommands::Subr { rd, rn, rm } => {
        let val1 = cpu.registers.read(*rn);
        let val2 = cpu.registers.read(*rm);
        cpu.registers.write(*rd, val1.wrapping_sub(val2));
      }
      ISACommands::Subis { rd, rn, imm } => {
        let val1 = cpu.registers.read(*rn);
        let imm_val = *imm as u64;
        let (diff, did_borrow) = val1.overflowing_sub(imm_val);
        cpu.flags.zero_flag = diff == 0;
        cpu.flags.negative_flag = (diff as i64) < 0;
        cpu.flags.carry_flag = !did_borrow;
        let val1_i = val1 as i64;
        let imm_i = imm_val as i64;
        let diff_i = diff as i64;
        cpu.flags.overflow_flag = (val1_i ^ imm_i) < 0 && (val1_i ^ diff_i) < 0;
        cpu.registers.write(*rd, diff);
      }
      ISACommands::Subrs { rd, rn, rm } => {
        let val1 = cpu.registers.read(*rn);
        let val2 = cpu.registers.read(*rm);
        let (diff, did_borrow) = val1.overflowing_sub(val2);
        cpu.flags.zero_flag = diff == 0;
        cpu.flags.negative_flag = (diff as i64) < 0;
        cpu.flags.carry_flag = !did_borrow;
        let val1_i = val1 as i64;
        let val2_i = val2 as i64;
        let diff_i = diff as i64;
        cpu.flags.overflow_flag = (val1_i ^ val2_i) < 0 && (val1_i ^ diff_i) < 0;
        cpu.registers.write(*rd, diff);
      }
      ISACommands::Muli { rd, rn, imm } => {
        let prod = cpu.registers.read(*rn).wrapping_mul(*imm as u64);
        cpu.registers.write(*rd, prod);
      }
      ISACommands::Mulr { rd, rn, rm } => {
        let val1 = cpu.registers.read(*rn);
        let val2 = cpu.registers.read(*rm);
        cpu.registers.write(*rd, val1.wrapping_mul(val2));
      }
      ISACommands::Divi { rd, rn, imm } => {
        if *imm == 0 {
          panic!("ZeroDivisionError: caught at divi.");
        }
        let quo = cpu.registers.read(*rn).wrapping_div(*imm as u64);
        cpu.registers.write(*rd, quo);
      }
      ISACommands::Divr { rd, rn, rm } => {
        let divisor = cpu.registers.read(*rm);
        if divisor == 0 {
          panic!("ZeroDivisionError: caught at divr.");
        }
        let quo = cpu.registers.read(*rn).wrapping_div(divisor);
        cpu.registers.write(*rd, quo);
      }
      ISACommands::Cmpi { rn, imm } => {
        let val1 = cpu.registers.read(*rn);
        let imm_val = *imm as u64;
        let (diff, did_borrow) = val1.overflowing_sub(imm_val);
        cpu.flags.zero_flag = diff == 0;
        cpu.flags.negative_flag = (diff as i64) < 0;
        cpu.flags.carry_flag = !did_borrow;
        let val1_i = val1 as i64;
        let imm_i = imm_val as i64;
        let diff_i = diff as i64;
        cpu.flags.overflow_flag = (val1_i ^ imm_i) < 0 && (val1_i ^ diff_i) < 0;
      }
      ISACommands::Cmpr { rn, rm } => {
        let val1 = cpu.registers.read(*rn);
        let val2 = cpu.registers.read(*rm);
        let (diff, did_borrow) = val1.overflowing_sub(val2);
        cpu.flags.zero_flag = diff == 0;
        cpu.flags.negative_flag = (diff as i64) < 0;
        cpu.flags.carry_flag = !did_borrow;
        let val1_i = val1 as i64;
        let val2_i = val2 as i64;
        let diff_i = diff as i64;
        cpu.flags.overflow_flag = (val1_i ^ val2_i) < 0 && (val1_i ^ diff_i) < 0;
      }
      ISACommands::B { target_address } => {
        cpu.pc = *target_address;
      }
      ISACommands::Bl { target_address } => {
        cpu.stack_line.push(cpu.pc + 1);
        cpu.pc = *target_address;
      }
      ISACommands::Beq { target_address } => {
        if cpu.flags.zero_flag {
          cpu.pc = *target_address;
        }
      }
      ISACommands::Bne { target_address } => {
        if !cpu.flags.zero_flag {
          cpu.pc = *target_address;
        }
      }
      ISACommands::Bgt { target_address } => {
        if !cpu.flags.zero_flag && cpu.flags.negative_flag == cpu.flags.overflow_flag {
          cpu.pc = *target_address;
        }
      }
      ISACommands::Blt { target_address } => {
        if cpu.flags.negative_flag != cpu.flags.overflow_flag {
          cpu.pc = *target_address;
        }
      }
      ISACommands::Bge { target_address } => {
        if cpu.flags.negative_flag == cpu.flags.overflow_flag {
          cpu.pc = *target_address;
        }
      }
      ISACommands::Ble { target_address } => {
        if cpu.flags.zero_flag || cpu.flags.negative_flag != cpu.flags.overflow_flag {
          cpu.pc = *target_address;
        }
      }
      ISACommands::Ldr { rd, rn, imm } => {
        let base_address = cpu.registers.read(*rn);
        let target_address = (base_address + *imm as u64) as usize;
        let bytes: [u8; 8] = cpu.memory[target_address..target_address + 8]
          .try_into()
          .expect("Out of bounds error caught at `ldr`.");
        let value = u64::from_le_bytes(bytes);
        cpu.registers.write(*rd, value);
      }
      ISACommands::Str { rt, rn, imm } => {
        let base = cpu.registers.read(*rn);
        let imm_val = *imm as u64;
        let addr = base.checked_add(imm_val).expect("STR: address overflow") as usize;
        let end = addr.checked_add(8).expect("STR: address overflow");
        cpu.memory[addr..end].copy_from_slice(&u64::to_le_bytes(cpu.registers.read(*rt)));
      }
      ISACommands::Ldrb { rd, rn, imm } => {
        let target_address = (cpu.registers.read(*rn) + *imm as u64) as usize;
        let value = cpu.memory[target_address] as u64;
        cpu.registers.write(*rd, value);
      }
      ISACommands::Strb { rt, rn, imm } => {
        let value = cpu.registers.read(*rt) as u8;
        let target_address = (cpu.registers.read(*rn) + *imm as u64) as usize;
        cpu.memory[target_address] = value;
      }
      ISACommands::Ret => {
        cpu.pc = cpu.stack_line.pop().expect("Stack line crashed! Likely due to improper stack frame management.\nEnsure that for your every `bl`, you pass a `ret` & vice-versa.");
      }
      ISACommands::Remi { rd, rn, imm } => {
        cpu.registers.write(*rd, cpu.registers.read(*rn) % *imm as u64)
      }
      ISACommands::Remr { rd, rn, rm } => {
        cpu.registers.write(*rd, cpu.registers.read(*rn) % cpu.registers.read(*rm));
      }
    }
  }
}
