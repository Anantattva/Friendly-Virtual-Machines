// ।। ॐ नमः शिवाय ।। \\
// @date_start 8th November, 2026

#![allow(warnings)]

use std::collections::HashMap;

use crate::isa::{Operation, Condition, ISACommand};
use crate::isa::Program;

///// ================== \\\\\
///// ++ Token Parser ++ \\\\\
///// ================== \\\\\

/// +++ global enum to handle errors ++ \\\
#[derive(Debug)]
pub enum AssemblerError {
  TextParsingError(String),
  TokenParsingError(String),
  LabelResolutionError(String),
}

/// ++ helper parser functions ++ \\\
pub fn parse_reg(token: &str) -> Result<usize, AssemblerError> {
  token
    .replace('x', "")
    .replace('v', "")
    .parse::<usize>()
    .map_err(|_| AssemblerError::TokenParsingError(format!("Invalid register name: '{}'.", token)))
}

pub fn parse_imm(token: &str) -> Result<usize, AssemblerError> {
  token
    .replace('#', "")
    .parse::<usize>()
    .map_err(|_| AssemblerError::TokenParsingError(format!("Invalid immediate value: '{}'.", token)))
}

/// ++ handler functions for ISACommand generator ++ \\\
pub fn handle_one_length_command(inst: &str) -> Result<ISACommand, AssemblerError> {
  match inst {
    "ret" => Ok(ISACommand::Ret),
    "exit" => Ok(ISACommand::Exit),
    _ => Err(AssemblerError::TextParsingError(format!("Unknown or unimplemented instruction: '{}'.", inst))),
  }
}

pub fn handle_two_length_command(inst: &str, op1: &str) -> Result<ISACommand, AssemblerError> {
  match inst {
    "b" => Ok(ISACommand::B { ta: parse_imm(op1)? }),
    "bl" => Ok(ISACommand::Bl { ta: parse_imm(op1)? }),
    "addsp" => Ok(ISACommand::Addsp { imm: parse_imm(op1)? }),
    "subsp" => Ok(ISACommand::Subsp { imm: parse_imm(op1)? }),
    _ => Err(AssemblerError::TextParsingError(format!("Unknown or unimplemented instruction: '{}'.", inst))),
  }
}

pub fn handle_mov_command(op1: &str, op2: &str) -> Result<ISACommand, AssemblerError> {
  if op1.contains('x') && op2.contains('#') {
    return Ok(ISACommand::Xmovi { rd: parse_reg(op1)?, imm: parse_imm(op2)? as u64 });
  } else if op1.contains('v') && op2.contains('#') {
    return Ok(ISACommand::Vmovi { rd: parse_reg(op1)?, imm: parse_imm(op2)? as f64 });
  } else if op1.contains('x') && op2.contains('x') {
    return Ok(ISACommand::Xmovr { rd: parse_reg(op1)?, rn: parse_reg(op2)? });
  } else if op1.contains('v') && op2.contains('v') {
    return Ok(ISACommand::Vmovr { rd: parse_reg(op1)?, rn: parse_reg(op2)? });
  } else if op1.contains('x') && op2.contains('v') {
    return Ok(ISACommand::Xmovv { rd: parse_reg(op1)?, rn: parse_reg(op2)? });
  } else if op1.contains('v') && op2.contains('x') {
    return Ok(ISACommand::Vmovx { rd: parse_reg(op1)?, rn: parse_reg(op2)? });
  }
  Err(AssemblerError::TextParsingError(format!("Unexpected or invalid 'mov' instruction: {}, {}.", op1, op2)))
}

pub fn handle_ldr_command(tokens: &Vec<&str>) -> Result<ISACommand, AssemblerError> {
  let op1 = tokens[0];
  let parsed_reg = parse_reg(op1)?;
  let is_float = op1.contains('v');

  match tokens.len() {
    2 => {
      // Case 1: ldr x0, [sp] (3 tokens)
      if is_float {
        Ok(ISACommand::Ldrv { rd: parsed_reg, imm: 0 })
      } else {
        Ok(ISACommand::Ldrx { rd: parsed_reg, imm: 0 })
      }
    }
    3 => {
      // Case 2: ldr x0, [sp + #8] (5 tokens)
      let imm = parse_imm(tokens[2])?;
      if is_float {
        Ok(ISACommand::Ldrv { rd: parsed_reg, imm })
      } else {
        Ok(ISACommand::Ldrx { rd: parsed_reg, imm })
      }
    }
    _ => Err(AssemblerError::TextParsingError(format!("Invalid token length or format for 'ldr': got length {} and tokens {:?}.", tokens.len(), tokens))),
  }
}

pub fn handle_str_command(tokens: &Vec<&str>) -> Result<ISACommand, AssemblerError> {
  let op1 = tokens[0];
  let parsed_reg = parse_reg(op1)?;
  let is_float = op1.contains('v');

  match tokens.len() {
    2 => {
      // Case 1: str x0, [sp] (2 tokens: x0, sp)
      if is_float {
        Ok(ISACommand::Strv { rt: parsed_reg, imm: 0 })
      } else {
        Ok(ISACommand::Strx { rt: parsed_reg, imm: 0 })
      }
    }
    3 => {
      // Case 2: str x0, [sp + #8] (3 tokens: x0, sp, #8)
      let imm = parse_imm(tokens[2])?;
      if is_float {
        Ok(ISACommand::Strv { rt: parsed_reg, imm })
      } else {
        Ok(ISACommand::Strx { rt: parsed_reg, imm })
      }
    }
    _ => Err(AssemblerError::TextParsingError(format!("Invalid token length or format for 'str': got length {} and tokens {:?}.", tokens.len(), tokens))),
  }
}


pub fn handle_three_length_command(inst: &str, op1: &str, op2: &str) -> Result<ISACommand, AssemblerError> {
  // << handles MOV & LDR/STR for case imm 0 >> \\
  match inst {
    "mov" => handle_mov_command(op1, op2),
    "ldr" => handle_ldr_command(&vec![op1, op2]),
    "str" => handle_str_command(&vec![op1, op1]),
    _ => Err(AssemblerError::TextParsingError(format!("Unknown or unimplemented instruction: '{}'.", inst))),
  }
}

pub fn handle_ldp_command(tokens: &Vec<&str>) -> Result<ISACommand, AssemblerError> {
  let op1 = tokens[0];
  let op2 = tokens[0];
  let parsed_reg1 = parse_reg(op1)?;
  let parsed_reg2 = parse_reg(op2)?;
  let is_float = op1.contains('v');

  match tokens.len() {
    3 => {
      if !is_float {
        Ok(ISACommand::Ldpx { rd1: parsed_reg1, rd2: parsed_reg2, imm: 0 })
      } else {
        Ok(ISACommand::Ldpv { rd1: parsed_reg1, rd2: parsed_reg2, imm: 0,})
      }
    },
    4 => {
      if !is_float {
        Ok(ISACommand::Ldpx { rd1: parsed_reg1, rd2: parsed_reg2, imm: parse_imm(tokens[3])? })
      } else {
        Ok(ISACommand::Ldpv { rd1: parsed_reg1, rd2: parsed_reg2, imm: parse_imm(tokens[3])? })
      }
    },
    _ => Err(AssemblerError::TextParsingError(format!("Invalid tokens for 'ldp': {:?}.", tokens))),
  }
}

pub fn handle_stp_command(tokens: &Vec<&str>) -> Result<ISACommand, AssemblerError> {
  let op1 = tokens[0];
  let op2 = tokens[0];
  let parsed_reg1 = parse_reg(op1)?;
  let parsed_reg2 = parse_reg(op2)?;
  let is_float = op1.contains('v');

  match tokens.len() {
    3 => {
      if !is_float {
        Ok(ISACommand::Stpx { rt1: parsed_reg1, rt2: parsed_reg2, imm: 0 })
      } else {
        Ok(ISACommand::Stpv { rt1: parsed_reg1, rt2: parsed_reg2, imm: 0,})
      }
    },
    4 => {
      if !is_float {
        Ok(ISACommand::Stpx { rt1: parsed_reg1, rt2: parsed_reg2, imm: parse_imm(tokens[3])? })
      } else {
        Ok(ISACommand::Stpv { rt1: parsed_reg1, rt2: parsed_reg2, imm: parse_imm(tokens[3])? })
      }
    },
    _ => Err(AssemblerError::TextParsingError(format!("Invalid tokens for 'stp': {:?}.", tokens))),
  }
}

pub fn handle_arithmetic_command(inst: &str, op1: &str, op2: &str, op3: &str) -> Result<ISACommand, AssemblerError> {
  let parsed_reg1 = parse_reg(op1)?;
  let parsed_reg2 = parse_reg(op2)?;
  let is_int = op1.contains('x');
  let is_imm = op3.contains('#');

  match inst {
    "add" => {
      if is_int && is_imm {
        Ok(ISACommand::XCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as u64, op: Operation::Add })
      } else if is_int && !is_imm {
        Ok(ISACommand::XCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Add })
      } else if !is_int && is_imm {
        Ok(ISACommand::VCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as f64, op: Operation::Add })
      } else if !is_int && !is_imm {
        Ok(ISACommand::VCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Add })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid 'add' tokens: {}, {}, {}.", op1, op2, op3)))
      }
    },
    "sub" => {
      if is_int && is_imm {
        Ok(ISACommand::XCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as u64, op: Operation::Sub })
      } else if is_int && !is_imm {
        Ok(ISACommand::XCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Sub })
      } else if !is_int && is_imm {
        Ok(ISACommand::VCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as f64, op: Operation::Sub })
      } else if !is_int && !is_imm {
        Ok(ISACommand::VCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Sub })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid 'sub' tokens: {}, {}, {}.", op1, op2, op3)))
      }
    },
    "mul" => {
      if is_int && is_imm {
        Ok(ISACommand::XCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as u64, op: Operation::Mul })
      } else if is_int && !is_imm {
        Ok(ISACommand::XCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Mul })
      } else if !is_int && is_imm {
        Ok(ISACommand::VCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as f64, op: Operation::Mul })
      } else if !is_int && !is_imm {
        Ok(ISACommand::VCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Mul })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid 'mul' tokens: {}, {}, {}.", op1, op2, op3)))
      }
    },
    "div" => {
      if is_int && is_imm {
        Ok(ISACommand::XCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as u64, op: Operation::Div })
      } else if is_int && !is_imm {
        Ok(ISACommand::XCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Div })
      } else if !is_int && is_imm {
        Ok(ISACommand::VCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as f64, op: Operation::Div })
      } else if !is_int && !is_imm {
        Ok(ISACommand::VCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Div })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid 'div' tokens: {}, {}, {}.", op1, op2, op3)))
      }
    },
    "rem" => {
      if is_int && is_imm {
        Ok(ISACommand::XCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as u64, op: Operation::Rem })
      } else if is_int && !is_imm {
        Ok(ISACommand::XCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Rem })
      } else if !is_int && is_imm {
        Ok(ISACommand::VCalci { rd: parsed_reg1, rn: parsed_reg2, imm: parse_imm(op3)? as f64, op: Operation::Rem })
      } else if !is_int && !is_imm {
        Ok(ISACommand::VCalcr { rd: parsed_reg1, rn: parsed_reg2, rm: parse_reg(op3)?, op: Operation::Rem })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid 'rem' tokens: {}, {}, {}.", op1, op2, op3)))
      }
    },
    _ => Err(AssemblerError::TextParsingError(format!("Unknown or unimplemented instruction: {}.", inst))),
  }
}

pub fn handle_condition_command(inst: &str, op1: &str, op2: &str, op3: &str) -> Result<ISACommand, AssemblerError> {
  let parsed_ta = parse_imm(op1)?;
  let parsed_reg = parse_reg(op2)?;
  let is_int = op2.contains('x');
  let is_imm = op3.contains('#');

  match inst {
    "beq" => {
      if is_int && is_imm {
        Ok(ISACommand::Bxi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as u64, cd: Condition::Eq })
      } else if !is_int && is_imm {
        Ok(ISACommand::Bvi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as f64, cd: Condition::Eq })
      } else if is_int && !is_imm {
        Ok(ISACommand::Bxr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Eq })
      } else if !is_int && !is_imm {
        Ok(ISACommand::Bvr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Eq })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid tokens for 'beq': {}, {}, {}.", op1, op2, op3)))
      }
    },
    "bne" => {
      if is_int && is_imm {
        Ok(ISACommand::Bxi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as u64, cd: Condition::Ne })
      } else if !is_int && is_imm {
        Ok(ISACommand::Bvi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as f64, cd: Condition::Ne })
      } else if is_int && !is_imm {
        Ok(ISACommand::Bxr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Ne })
      } else if !is_int && !is_imm {
        Ok(ISACommand::Bvr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Ne })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid tokens for 'bne': {}, {}, {}.", op1, op2, op3)))
      }
    },
    "bgt" => {
      if is_int && is_imm {
        Ok(ISACommand::Bxi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as u64, cd: Condition::Gt })
      } else if !is_int && is_imm {
        Ok(ISACommand::Bvi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as f64, cd: Condition::Gt })
      } else if is_int && !is_imm {
        Ok(ISACommand::Bxr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Gt })
      } else if !is_int && !is_imm {
        Ok(ISACommand::Bvr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Gt })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid tokens for 'bgt': {}, {}, {}.", op1, op2, op3)))
      }
    },
    "blt" => {
      if is_int && is_imm {
        Ok(ISACommand::Bxi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as u64, cd: Condition::Lt })
      } else if !is_int && is_imm {
        Ok(ISACommand::Bvi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as f64, cd: Condition::Lt })
      } else if is_int && !is_imm {
        Ok(ISACommand::Bxr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Lt })
      } else if !is_int && !is_imm {
        Ok(ISACommand::Bvr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Lt })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid tokens for 'blt': {}, {}, {}.", op1, op2, op3)))
      }
    },
    "bge" => {
      if is_int && is_imm {
        Ok(ISACommand::Bxi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as u64, cd: Condition::Ge })
      } else if !is_int && is_imm {
        Ok(ISACommand::Bvi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as f64, cd: Condition::Ge })
      } else if is_int && !is_imm {
        Ok(ISACommand::Bxr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Ge })
      } else if !is_int && !is_imm {
        Ok(ISACommand::Bvr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Ge })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid tokens for 'bge': {}, {}, {}.", op1, op2, op3)))
      }
    },
    "ble" => {
      if is_int && is_imm {
        Ok(ISACommand::Bxi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as u64, cd: Condition::Le })
      } else if !is_int && is_imm {
        Ok(ISACommand::Bvi { ta: parsed_ta, rn: parsed_reg, imm: parse_imm(op3)? as f64, cd: Condition::Le })
      } else if is_int && !is_imm {
        Ok(ISACommand::Bxr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Le })
      } else if !is_int && !is_imm {
        Ok(ISACommand::Bvr { ta: parsed_ta, rn: parsed_reg, rm: parse_reg(op3)?, cd: Condition::Le })
      } else {
        Err(AssemblerError::TextParsingError(format!("Unexpected or invalid tokens for 'ble': {}, {}, {}.", op1, op2, op3)))
      }
    },
    _ => Err(AssemblerError::TextParsingError(format!("Unknown or unimplemented instruction: '{}'.", inst)))
  }
}

pub fn handle_four_length_command(inst: &str, op1: &str, op2: &str, op3: &str) -> Result<ISACommand, AssemblerError> {
  // << handles LDR/STR for case imm > 0 & LDP/STP for case imm 0 & Arithmetic & Conditions >> \\
  match inst {
    "ldr" => handle_ldr_command(&vec![op1, op2, op3]),
    "str" => handle_str_command(&vec![op1, op2, op3]),
    "ldp" => handle_ldp_command(&vec![op1, op2, op3]),
    "stp" => handle_stp_command(&vec![op1, op2, op3]),
    "add" | "sub" | "mul" | "div" | "rem" => handle_arithmetic_command(inst, op1, op2, op3),
    "beq" | "bne" | "bgt" | "blt" | "bge" | "ble" => handle_condition_command(inst, op1, op2, op3),
    _ => Err(AssemblerError::TextParsingError(format!("Unknown or unimplemented instruction: '{}'.", inst))),
  }
}


pub fn handle_five_length_command(inst: &str, op1: &str, op2: &str, op3: &str, op4: &str) -> Result<ISACommand, AssemblerError> {
  // << handles Arithmetic, Conditions & LDP/STP for case imm > 0 >> \\
  match inst {
    "ldp" => handle_ldp_command(&vec![op1, op2, op3, op4]),
    "stp" => handle_stp_command(&vec![op1, op2, op3, op4]),
    _ => Err(AssemblerError::TextParsingError(format!("Unknown or unimplemented instruction: '{}'.", inst))),
  }
}

pub fn get_isa_from_tokens(tokens: &Vec<&str>) -> Result<ISACommand, AssemblerError> {
  let inst = tokens[0];
  // -- match length-by-length first -- \\
  match tokens.len() {
    1 => handle_one_length_command(inst),
    2 => handle_two_length_command(inst, tokens[1]),
    3 => handle_three_length_command(inst, tokens[1], tokens[2]),
    4 => handle_four_length_command(inst, tokens[1], tokens[2], tokens[3]),
    _ => Err(AssemblerError::TextParsingError(format!("Unexpected or invalid token stream: {:?}.", tokens))),
  }
}

///// ===================== \\\\\
///// +++++ Tokenizer +++++ \\\\\
///// ===================== \\\\\

pub fn tokenize_raw_text(program_text: &str) -> Vec<Vec<String>> {
  // << first trim out comments >> \\
  program_text.lines()
    .map(|l| {
      if let Some(index) = l.find("//") {
        &l[..index]
      } else {
        l
      }
      .trim()
    })
  // << filter out empty lines >> \\
    .filter(|l| !l.is_empty() )
  // << replace symbols, split & collect >> \\
    .map(|l| {
      l.replace(',', "")
        .replace('[', "")
        .replace(']', "")
        .replace('+', "")
        .split_whitespace()
        .map(|s| s.to_string() )
        .collect::<Vec<String>>()
    })
    .collect()
}

///// ============= \\\\\
/// +++ Assembler +++ \\\
///// ============= \\\\\

pub fn assemble_program(program_text: &str) -> Result<Program, AssemblerError> {
  // 1. Tokenize into owned Vec<Vec<String>>
  let token_stream = tokenize_raw_text(program_text);

  let mut symbol_table: HashMap<String, usize> = HashMap::new();
  let mut instructions_only = Vec::new();

  // Pass 1: Build symbol table and filter out labels
  let mut instruction_index: usize = 0;
  for tokens in &token_stream {
    if tokens.len() == 1 && tokens[0].ends_with(':') {
      let label_token = tokens[0].trim_end_matches(':').to_string();
      symbol_table.insert(label_token, instruction_index);
    } else {
      instructions_only.push(tokens);
      instruction_index += 1;
    }
  }

  // Pass 2: Resolve labels and generate ISA commands
  let mut program = Vec::new();
  for tokens in instructions_only {
    let mut cloned_tokens = tokens.clone();

    // Check if it's a branch instruction that might use a label
    if cloned_tokens.len() >= 2 && cloned_tokens[0].starts_with('b') {
      let target_string = &cloned_tokens[1];
            
      // If it's not a raw immediate number, look it up in the symbol table
      if parse_imm(target_string).is_err() {
        if let Some(&target_index) = symbol_table.get(target_string) {
          cloned_tokens[1] = format!("#{}", target_index); // format as immediate for your parser
        } else {
          return Err(AssemblerError::LabelResolutionError(format!("Undefined label: '{}'. Known labels: {:#?}", target_string, symbol_table)));
        }
      }
    }

    // Convert the resolved tokens into an ISACommand using your existing parser!
    let isa_cmd = get_isa_from_tokens(&cloned_tokens.iter().map(|s| s.as_str()).collect())?;
    program.push(isa_cmd);
  }

  Ok(program)
}
