// ।। ॐ नमः शिवाय ।। \\
// @date_start 3rd November, 2026

#![allow(warnings)]

use std::collections::HashMap;
use crate::isa::ISACommands;

pub fn get_isa_command_from_tokens(tokens: &Vec<&str>) -> ISACommands {
  // << setup >> \\
  let len = tokens.len();
  let opcode = tokens[0];
  
  // << match length 1 first: Exit & Ret get eliminated here >> \\  
  if len == 1 {
    match opcode {
      "exit" => return ISACommands::Exit,
      "ret" => return ISACommands::Ret,
      _ => panic!("AssemblerError::TextParsingError: Unknown or unimplemented instruction or opcode: {}.\nCaught at ISA construction stage!", opcode),
    }
  }

  // << match length 2: Branches & Label get eliminated here >> \\
  if len == 2 {
    match opcode {
      "b" => {
        return ISACommands::B { target_address: tokens[1].parse::<usize>().unwrap() };
      },
      "bl" => {
        return ISACommands::Bl { target_address: tokens[1].parse::<usize>().unwrap() };
      },
      "beq" => {
        return ISACommands::Beq { target_address: tokens[1].parse::<usize>().unwrap() };
      },
      "bne" => {
        return ISACommands::Bne { target_address: tokens[1].parse::<usize>().unwrap() };
      },
      "bgt" => {
        return ISACommands::Bgt { target_address: tokens[1].parse::<usize>().unwrap() };
      },
      "blt" => {
        return ISACommands::Blt { target_address: tokens[1].parse::<usize>().unwrap() };
      },
      "bge" => {
        return ISACommands::Bge { target_address: tokens[1].parse::<usize>().unwrap() };
      },
      "ble" => {
        return ISACommands::Ble { target_address: tokens[1].parse::<usize>().unwrap() };
      },
      _ => panic!("AssemblerError::TextParsingError: Unknown or unimplemented instruction or opcode: {}.\nCaught at ISA construction stage!", opcode),
    }
  }

  // << match length 3: Mov & Cmp get eliminated here >> \\
  if len == 3 {
    let first_operand_parsed = tokens[1].replace('x', "").parse::<usize>().unwrap();
    let second_operand = tokens[2];
    match opcode {
      // << handles both Movi & Movr >> \\
      "mov" => {
        if second_operand.contains('#') {
          return ISACommands::Movi { rd: first_operand_parsed, imm: second_operand.replace('#', "").parse::<u16>().unwrap() };
        } else if second_operand.contains('x') {
          return ISACommands::Movr { rd: first_operand_parsed, rn: second_operand.replace('x', "").parse::<usize>().unwrap() };        
        } else {
          panic!("AssemblerError::InvalidOperandsError: Invalid second operand for `mov`: {}.\nCaught at ISA construction stage!", second_operand);
        }
      },

      // << handles both Cmpi & Cmpr >> \\
      "cmp" => {
        if second_operand.contains('#') {
          return ISACommands::Cmpi { rn: first_operand_parsed, imm: second_operand.replace('#', "").parse::<u16>().unwrap() };
        } else if second_operand.contains('x') {
          return ISACommands::Cmpr { rn: first_operand_parsed, rm: second_operand.replace('x', "").parse::<usize>().unwrap() };        
        } else {
          panic!("AssemblerError:InvalidOperandsError: Invalid second operand for `cmp`: {}.\nCaught at ISA construction stage!", second_operand);
        }
      },
      _ => panic!("AssemblerError::TextParsingError: Unknown or unimplemented instruction or opcode: {}.\nCaught at ISA construction stage!", opcode),
    }
  }

  // << match length 4: Rest of the ISA gets covered here >> \\
  if len == 4 {
    let first_operand_parsed = tokens[1].replace('x', "").parse::<usize>().unwrap();
    let second_operand_parsed = tokens[2].replace('x', "").parse::<usize>().unwrap();
    let third_operand = tokens[3];
    match opcode {     
      // << handles both Addi & Addr >> \\
      "add" => {
        if third_operand.contains('#') {
          return ISACommands::Addi { rd: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
        } else if third_operand.contains('x') {
          return ISACommands::Addr { rd: first_operand_parsed, rn: second_operand_parsed, rm: third_operand.replace('x', "").parse::<usize>().unwrap() };
        } else {
          panic!("AssemblerError::InvalidOperandsError: Invalid third operand for `add`: {}.\nCaught at ISA construction stage!", third_operand);
        }
      },

      // << handles both Addis & Addrs >> \\
      "adds" => {
        if third_operand.contains('#') {
          return ISACommands::Addis { rd: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
        } else if third_operand.contains('x') {
          return ISACommands::Addrs { rd: first_operand_parsed, rn: second_operand_parsed, rm: third_operand.replace('x', "").parse::<usize>().unwrap() };
        } else {
          panic!("AssemblerError::InvalidOperandsError: Invalid third operand for `adds`: {}.\nCaught at ISA construction stage!", third_operand);
        }
      },

      // << handles both Subi & Subr >> \\
      "sub" => {
        if third_operand.contains('#') {
          return ISACommands::Subi { rd: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
        } else if third_operand.contains('x') {
          return ISACommands::Subr { rd: first_operand_parsed, rn: second_operand_parsed, rm: third_operand.replace('x', "").parse::<usize>().unwrap() };
        } else {
          panic!("AssemblerError::InvalidOperandsError: Invalid third operand for `sub`: {}.\nCaught at ISA construction stage!", third_operand);
        }
      },

      // << handles both Subis & Subrs >> \\
      "subs" => {
        if third_operand.contains('#') {
          return ISACommands::Subis { rd: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
        } else if third_operand.contains('x') {
          return ISACommands::Subrs { rd: first_operand_parsed, rn: second_operand_parsed, rm: third_operand.replace('x', "").parse::<usize>().unwrap() };
        } else {
          panic!("AssemblerError::InvalidOperandsError: Invalid third operand for `subs`: {}.\nCaught at ISA construction stage!", third_operand);
        }
      },

      // << handles both Muli & Mulr >> \\
      "mul" => {
        if third_operand.contains('#') {
          return ISACommands::Muli { rd: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
        } else if third_operand.contains('x') {
          return ISACommands::Mulr { rd: first_operand_parsed, rn: second_operand_parsed, rm: third_operand.replace('x', "").parse::<usize>().unwrap() };
        } else {
          panic!("AssemblerError::InvalidOperandsError: Invalid third operand for `mul`: {}.\nCaught at ISA construction stage!", third_operand);
        }
      },

      // << handles both Divi & Divr >> \\
      "div" => {
        if third_operand.contains('#') {
          return ISACommands::Divi { rd: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
        } else if third_operand.contains('x') {
          return ISACommands::Divr { rd: first_operand_parsed, rn: second_operand_parsed, rm: third_operand.replace('x', "").parse::<usize>().unwrap() };
        } else {
          panic!("AssemblerError::InvalidOperandsError: Invalid third operand for `div`: {}.\nCaught at ISA construction stage!", third_operand);
        }
      },

      // << handles both Remi & Remr >> \\
      "rem" => {
        if third_operand.contains('#') {
          return ISACommands::Remi { rd: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
        } else if third_operand.contains('x') {
          return ISACommands::Remr { rd: first_operand_parsed, rn: second_operand_parsed, rm: third_operand.replace('x', "").parse::<usize>().unwrap() };
        } else {
          panic!("AssemblerError::InvalidOperandsError: Invalid third operand for `rem`: {}.\nCaught at ISA construction stage!", third_operand);
        }
      },

      // << handle load/store >> \\
      "ldr" => {
        return ISACommands::Ldr { rd: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
      },
      "str" => {
        return ISACommands::Str { rt: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
      },
      "ldrb" => {
        return ISACommands::Ldrb { rd: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
      },
      "strb" => {
        return ISACommands::Strb { rt: first_operand_parsed, rn: second_operand_parsed, imm: third_operand.replace('#', "").parse::<u8>().unwrap() };
      },

      _ => panic!("AssemblerError::TextParsingError: Unknown or unimplemented instruction or opcode: {}.\nCaught at ISA construction stage!", opcode),
    }
  } else {
    panic!("AssemblerError::TextParsingError: Unknown or unimplemented instruction: {:#?}.", tokens);
  }
}

pub fn stream_out_tokens_from_raw_text(program_text: &str) -> Vec<Vec<String>> {
  program_text.lines()
    // 1. Remove comments and trim whitespace
    .map(|line| {
      if let Some(idx) = line.find("//") {
        &line[..idx]
      } else {
        line
      }
      .trim()
    })
    // 2. Filter out empty lines
    .filter(|line| !line.is_empty())
    // 3. Clean commas and split into tokens
    .map(|line| {
      line.replace(',', "")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    })
    .collect()
}

pub fn assemble_program(program_text: &str) -> Vec<ISACommands> {
  let raw_tokens = stream_out_tokens_from_raw_text(program_text);
  
  let mut symbol_table: HashMap<String, usize> = HashMap::new();
  let mut instructions_tokens = Vec::new();

  // --- PASS 1: Build Symbol Table & Filter out pure label lines --- \\
  let mut instruction_index = 0;
  for tokens in raw_tokens {
    if tokens.len() == 1 && tokens[0].ends_with(':') {
      // << it's a label line like "loop:" >> \\
      let label_name = tokens[0].trim_end_matches(':').to_string();
      symbol_table.insert(label_name, instruction_index);
    } else {
      // << it's a real instruction, so it will occupy an index in the final program >> \\
      instructions_tokens.push(tokens);
      instruction_index += 1;
    }
  }

  // --- PASS 2: Resolve Labels and Generate ISACommands --- \\
  let mut program = Vec::new();
  for mut tokens in instructions_tokens {
    // << check if it's a branch instruction with a string label target (e.g., bne loop) >> \\
    if tokens.len() == 2 && (tokens[0].starts_with('b') || tokens[0] == "bl") {
      let target_str = &tokens[1];
      // << if the target is not a raw number, look it up in the symbol table >> \\
      if let Ok(_num) = target_str.parse::<usize>() {
        // << it's already a number, do nothing >> \\
      } else if let Some(&target_idx) = symbol_table.get(target_str) {
        // << replace string label with resolved numeric index string >> \\
        tokens[1] = target_idx.to_string();
      } else {
        panic!("AssemblerError::LabelResolutionError: Undefined label error: '{}'", target_str);
      }
    }

    // << convert string tokens into your ISACommands enum using your parser >> \\
    let slice_tokens: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let command = get_isa_command_from_tokens(&slice_tokens);
    program.push(command);
  }

  program
}

// Caveats:
// Multi-line comments are not supported
// The final assembler assumes the program is syntactically-perfect
// Also assumes instructions are separated by lines.
// Example:
// one cant write `mov x0, #1 mov x1 #10`.
// one should strictly write:
// mov x0, #1
// mov x1, #10