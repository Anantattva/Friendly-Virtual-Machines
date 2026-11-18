// ।। ॐ नमः शिवाय ।। \\
// @date_start 10th November, 2026

module isa;

import cpu;

enum Operation { Add, Sub, Mul, Div, Rem }
enum Condition { Eq, Ne, Gt, Lt, Ge, Le }

enum OpType {
  Movi,
  Movr,
  Calci,
  Calcr,
  B, Bl, Ret,
  Bi, Br,
  Addsp, Subsp,
  Ldr, Str,
  Ldp, Stp,
  Exit
}

interface ISACommand {
  void execute(CPU cpu);
}

class Movi : ISACommand {
  size_t rd;
  ulong imm;

  this(size_t rd, ulong imm) {
    this.rd = rd;
    this.imm = imm;
  }

  void execute(CPU cpu) {
    cpu.registers.write(rd, imm);
  }
}

class Movr : ISACommand {
  size_t rd;
  size_t rn;

  this(size_t rd, size_t rn) {
    this.rd = rd;
    this.rn = rn;
  }

  void execute(CPU cpu) {
    cpu.registers.write(rd, cpu.registers.read(rn));
  }
}

class Calci : ISACommand {
  size_t rd;
  size_t rn;
  ulong imm;
  Operation op;

  this(size_t rd, size_t rn, ulong imm, Operation op) {
    this.rd = rd;
    this.rn = rn;
    this.imm = imm;
    this.op = op;
  }

  void execute(CPU cpu) {
    ulong val1 = cpu.registers.read(rn);
    ulong finalVal;
    final switch (op) {
      case Operation.Add: finalVal = val1 + imm; break;
      case Operation.Sub: finalVal = val1 - imm; break;
      case Operation.Mul: finalVal = val1 * imm; break;
      case Operation.Div: finalVal = val1 / imm; break;
      case Operation.Rem: finalVal = val1 % imm; break;
    }
    cpu.registers.write(rd, finalVal);
  }
}

class Calcr : ISACommand {
  size_t rd;
  size_t rn;
  size_t rm;
  Operation op;

  this(size_t rd, size_t rn, size_t rm, Operation op) {
    this.rd = rd;
    this.rn = rn;
    this.rm = rm;
    this.op = op;
  }

  void execute(CPU cpu) {
    ulong val1 = cpu.registers.read(rn);
    ulong val2 = cpu.registers.read(rm);
    ulong finalVal;
    final switch (op) {
      case Operation.Add: finalVal = val1 + val2; break;
      case Operation.Sub: finalVal = val1 - val2; break;
      case Operation.Mul: finalVal = val1 * val2; break;
      case Operation.Div: finalVal = val1 / val2; break;
      case Operation.Rem: finalVal = val1 % val2; break;
    }
    cpu.registers.write(rd, finalVal);
  }
}

class B : ISACommand {
  size_t ta;

  this(size_t ta) {
    this.ta = ta;
  }

  void execute(CPU cpu) {
    cpu.pc = ta;
  }
}

class Bl : ISACommand {
  size_t ta;

  this(size_t ta) {
    this.ta = ta;
  }

  void execute(CPU cpu) {
    cpu.registers.write(30, cast(size_t)(cpu.pc + 1));
    cpu.pc = ta;
  }
}

class Ret : ISACommand {
  void execute(CPU cpu) {
    cpu.pc = cast(size_t)cpu.registers.read(30);
  }
}

class Bi : ISACommand {
  size_t ta;
  size_t rn;
  ulong imm;
  Condition cd;

  this(size_t ta, size_t rn, ulong imm, Condition cd) {
    this.ta = ta;
    this.rn = rn;
    this.imm = imm;
    this.cd = cd;
  }

  void execute(CPU cpu) {
    ulong val1 = cpu.registers.read(rn);
    bool isTrue;
    final switch (cd) {
      case Condition.Eq: isTrue = val1 == imm; break;
      case Condition.Ne: isTrue = val1 != imm; break;
      case Condition.Gt: isTrue = val1 > imm; break;
      case Condition.Lt: isTrue = val1 < imm; break;
      case Condition.Ge: isTrue = val1 >= imm; break;
      case Condition.Le: isTrue = val1 <= imm; break;
    }
    if (isTrue) cpu.pc = ta;
  }
}

class Br : ISACommand {
  size_t ta;
  size_t rn;
  size_t rm;
  Condition cd;

  this(size_t ta, size_t rn, size_t rm, Condition cd) {
    this.ta = ta;
    this.rn = rn;
    this.rm = rm;
    this.cd = cd;
  }

  void execute(CPU cpu) {
    ulong val1 = cpu.registers.read(rn);
    ulong val2 = cpu.registers.read(rm);
    bool isTrue;
    final switch (cd) {
      case Condition.Eq: isTrue = val1 == val2; break;
      case Condition.Ne: isTrue = val1 != val2; break;
      case Condition.Gt: isTrue = val1 > val2; break;
      case Condition.Lt: isTrue = val1 < val2; break;
      case Condition.Ge: isTrue = val1 >= val2; break;
      case Condition.Le: isTrue = val1 <= val2; break;
    }
    if (isTrue) cpu.pc = ta;
  }
}

class Addsp : ISACommand {
  size_t imm;
  this(size_t imm) { this.imm = imm; }
  void execute(CPU cpu) { cpu.registers.sp += imm; }
}

class Subsp : ISACommand {
  size_t imm;
  this(size_t imm) { this.imm = imm; }
  void execute(CPU cpu) { cpu.registers.sp -= imm; }
}

class Str : ISACommand {
  size_t rt;
  size_t imm;
  this(size_t rt, size_t imm) { this.rt = rt; this.imm = imm; }
  void execute(CPU cpu) {
    ulong val = cpu.registers.read(rt);
    cpu.storeStackU64(cpu.registers.sp + imm, val);
  }
}

class Ldr : ISACommand {
  size_t rd;
  size_t imm;
  this(size_t rd, size_t imm) { this.rd = rd; this.imm = imm; }
  void execute(CPU cpu) {
    ulong val = cpu.loadStackU64(cpu.registers.sp + imm);
    cpu.registers.write(rd, val);
  }
}

class Exit : ISACommand {
  void execute(CPU cpu) {
    cpu.state.isRunning = false;
  }
}