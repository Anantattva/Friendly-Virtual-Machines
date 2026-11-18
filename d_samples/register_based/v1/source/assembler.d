// ।। ॐ नमः शिवाय ।। \\
// @date_start 10th November, 2026

module assembler;

import std.stdio;
import std.string;
import std.array;
import std.algorithm;
import std.conv;
import std.ascii;

import isa;

alias Program = ISACommand[];

size_t parseReg(string token) {
  return token.replace("x", "").replace("#", "").strip.to!size_t;
}

size_t parseImm(string token) {
  return token.replace("#", "").strip.to!size_t;
}

ISACommand parseInstruction(string[] tokens, size_t[string] symbolTable) {
  string inst = tokens[0];

  switch (inst) {
    case "mov":
      if (tokens[2].startsWith("#")) {
        return new Movi(parseReg(tokens[1]), parseImm(tokens[2]));
      } else {
        return new Movr(parseReg(tokens[1]), parseReg(tokens[2]));
      }
    case "add":
    case "sub":
    case "mul":
    case "div":
    case "rem":
      Operation op;
      switch(inst) {
        case "add": op = Operation.Add; break;
        case "sub": op = Operation.Sub; break;
        case "mul": op = Operation.Mul; break;
        case "div": op = Operation.Div; break;
        case "rem": op = Operation.Rem; break;
        default: assert(0);
      }
      if (tokens[3].startsWith("#")) {
        return new Calci(parseReg(tokens[1]), parseReg(tokens[2]), parseImm(tokens[3]), op);
      } else {
        return new Calcr(parseReg(tokens[1]), parseReg(tokens[2]), parseReg(tokens[3]), op);
      }
    case "b":
      string target = tokens[1];
      size_t ta = target.startsWith("#") ? parseImm(target) : symbolTable[target];
      return new B(ta);
    case "bl":
      string target = tokens[1];
      size_t ta = target.startsWith("#") ? parseImm(target) : symbolTable[target];
      return new Bl(ta);
    case "ret":
      return new Ret();
    case "addsp":
      return new Addsp(parseImm(tokens[1]));
    case "subsp":
      return new Subsp(parseImm(tokens[1]));
    case "str":
      return new Str(parseReg(tokens[1]), tokens.length > 2 ? parseImm(tokens[2]) : 0);
    case "ldr":
      return new Ldr(parseReg(tokens[1]), tokens.length > 2 ? parseImm(tokens[2]) : 0);
    case "exit":
      return new Exit();
    default:
      // Handle conditional branches (e.g. bne, bge)
      if (inst.startsWith("b")) {
        string condStr = inst[1..$];
        Condition cd;
        switch(condStr) {
          case "eq": cd = Condition.Eq; break;
          case "ne": cd = Condition.Ne; break;
          case "gt": cd = Condition.Gt; break;
          case "lt": cd = Condition.Lt; break;
          case "ge": cd = Condition.Ge; break;
          case "le": cd = Condition.Le; break;
          default: throw new Exception("Unknown condition branch: " ~ inst);
        }
        string target = tokens[1];
        size_t ta = target.startsWith("#") ? parseImm(target) : symbolTable[target];
        
        if (tokens[3].startsWith("#")) {
          return new Bi(ta, parseReg(tokens[2]), parseImm(tokens[3]), cd);
        } else {
          return new Br(ta, parseReg(tokens[2]), parseReg(tokens[3]), cd);
        }
      }
      throw new Exception("Unknown instruction: " ~ inst);
  }
}

Program assembleProgram(string programText) {
  string[][] tokenStream;
  
  // Pass 0: Clean comments and tokenize
  foreach (line; programText.lineSplitter) {
    auto commentIdx = line.indexOf("//");
    auto cleanLine = (commentIdx != -1 ? line[0..commentIdx] : line).strip;
    if (cleanLine.empty) continue;

    auto tokens = cleanLine
      .replace(",", "")
      .replace("[", "")
      .replace("]", "")
      .replace("+", "")
      .split();
    
    if (!tokens.empty) {
      tokenStream ~= tokens;
    }
  }

  // Pass 1: Build Symbol Table
  size_t[string] symbolTable;
  string[][] instructionsOnly;
  size_t instructionIndex = 0;

  foreach (tokens; tokenStream) {
    if (tokens.length == 1 && tokens[0].endsWith(":")) {
      string label = tokens[0][0..$-1];
      symbolTable[label] = instructionIndex;
    } else {
      instructionsOnly ~= tokens;
      instructionIndex++;
    }
  }

  // Pass 2: Generate ISACommands
  Program program;
  foreach (tokens; instructionsOnly) {
    program ~= parseInstruction(tokens, symbolTable);
  }

  return program;
}
