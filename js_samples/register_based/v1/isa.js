// ।। ॐ नमः शिवाय ।। \\
// @date_start 31st October, 2026

export class Exit {
  constructor() {
    
  }
  
  execute(cpu) {
    cpu.state.isRunning = false;
  }
  
  static run(cpu) {
    cpu.state.isRunning = false;
  }

  static hex() {
    return 0x00;
  }
}

export class Movi {
  constructor(rd, imm) {
    this.rd = rd;
    this.imm = BigInt(imm);
  }

  execute(cpu) {
    cpu.registers.write(this.rd, this.imm);
  }

  static run(cpu, rd, imm) {
    cpu.registers.write(rd, BigInt(imm));
  }

  static hex() {
    return 0x01;
  }
}

export class Movr {
  constructor(rd, rn) {
    this.rd = rd;
    this.rn = rn;
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn));
  }

  static run(cpu, rd, rn) {
    cpu.registers.write(rd, cpu.registers.read(rn));
  }

  static hex() {
    return 0x02;
  }
}

export class Addi {
  constructor(rd, rn, imm) {
    this.rd = rd;
    this.rn = rn;
    this.imm = BigInt(imm);
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn) + this.imm);
  }

  static run(cpu, rd, rn, imm) {
    cpu.registers.write(rd, cpu.registers.read(rn) + BigInt(imm));
  }

  static hex() {
    return 0x03;
  }
}

export class Addr {
  constructor(rd, rn, rm) {
    this.rd = rd;
    this.rn = rn;
    this.rm = rm;
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn) + cpu.registers.read(this.rm));
  }

  static run(cpu, rd, rn, rm) {
    cpu.registers.write(rd, cpu.registers.read(rn) + cpu.registers.read(rm));
  }

  static hex() {
    return 0x04;
  }
}

export class Subi {
  constructor(rd, rn, imm) {
    this.rd = rd;
    this.rn = rn;
    this.imm = BigInt(imm);
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn) - this.imm);
  }

  static run(cpu, rd, rn, imm) {
    cpu.registers.x[rd] = cpu.registers.x[rn] - imm;
  }

  static hex() {
    return 0x05;
  }
}

export class Subr {
  constructor(rd, rn, rm) {
    this.rd = rd;
    this.rn = rn;
    this.rm = rm;
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn) - cpu.registers.read(this.rm));
  }

  static run(cpu, rd, rn, rm) {
    cpu.registers.write(rd, cpu.registers.read(rn) - cpu.registers.read(rm));
  }

  static hex() {
    return 0x06;
  }
}

export class Muli {
  constructor(rd, rn, imm) {
    this.rd = rd;
    this.rn = rn;
    this.imm = BigInt(imm);
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn) * this.imm);
  }

  static run(cpu, rd, rn, imm) {
    cpu.registers.write(rd, cpu.registers.read(rn) * imm);
  }

  static hex() {
    return 0x07;
  }
}

export class Mulr {
  constructor(rd, rn, rm) {
    this.rd = rd;
    this.rn = rn;
    this.rm = rm;
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(rn) * cpu.registers.read(this.rm));
  }

  static run(cpu, rd, rn, rm) {
    cpu.registers.write(rd, cpu.registers.read(rn) * cpu.registers.read(rm));
  }

  static hex() {
    return 0x08;
  }
}

export class Divi {
  constructor(rd, rn, imm) {
    this.rd = rd;
    this.rn = rn;
    this.imm = BigInt(imm);
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn) / this.imm);
  }

  static run(cpu, rd, rn, imm) {
    cpu.registers.write(rd, cpu.registers.read(rn) / imm);
  }

  static hex() {
    return 0x09;
  }
}

export class Divr {
  constructor(rd, rn, rm) {
    this.rd = rd;
    this.rn = rn;
    this.rm = rm;
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn) / cpu.registers.read(this.rm));
  }

  static run(cpu, rd, rn, rm) {
    cpu.registers.write(rd, cpu.registers.read(rn) / cpu.registers.read(rm));
  }

  static hex() {
    return 0x0A;
  }
}

export class Remi {
  constructor(rd, rn, imm) {
    this.rd = rd;
    this.rn = rn;
    this.imm = BigInt(imm);
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn) % this.imm);
  }

  static run(cpu, rd, rn, imm) {
    cpu.registers.write(rd, cpu.registers.read(rn) % imm);
  }

  static hex() {
    return 0x0B;
  }
}

export class Remr {
  constructor(rd, rn, rm) {
    this.rd = rd;
    this.rn = rn;
    this.rm = rm;
  }

  execute(cpu) {
    cpu.registers.write(this.rd, cpu.registers.read(this.rn) % cpu.registers.read(this.rm));
  }

  static run(cpu, rd, rn, rm) {
    cpu.registers.write(rd, cpu.registers.read(rn) % cpu.registers.read(rm));
  }

  static hex() {
    return 0x0C;
  }
}

const MASK_64 = 0xFFFFFFFFFFFFFFFFn;
const SIGN_BIT = 0x8000000000000000n;

export class Cmpi {
  constructor(rn, imm) {
    this.rn = rn;
    this.imm = BigInt(imm);
  }

  execute(cpu) {
    const val1 = cpu.registers.read(this.rn);
    const rawDiff = val1 - this.imm;
    const diff = rawDiff & MASK_64;

    // << cmp is essentially a sub that discards the result but updates flags >> \\
    cpu.flags.Z = diff === 0n;
    cpu.flags.N = (diff & SIGN_BIT) !== 0n;
    
    // << for comparison/subtraction, carry acts as 'not-borrow' (val1 >= imm) >> \\
    cpu.flags.C = val1 >= this.imm;

    // << signed overflow for subtraction >> \\
    const v1Signed = (val1 & SIGN_BIT) !== 0n;
    const immSigned = (this.imm & SIGN_BIT) !== 0n;
    const diffSigned = (diff & SIGN_BIT) !== 0n;
    cpu.flags.V = (v1Signed !== immSigned) && (v1Signed !== diffSigned);
  }
  
  static run(cpu, rn, imm) {
    const val1 = cpu.registers.read(rn);
    const rawDiff = val1 - imm;
    const diff = rawDiff & MASK_64;

    // << cmp is essentially a sub that discards the result but updates flags >> \\
    cpu.flags.Z = diff === 0n;
    cpu.flags.N = (diff & SIGN_BIT) !== 0n;
    
    // << for comparison/subtraction, carry acts as 'not-borrow' (val1 >= imm) >> \\
    cpu.flags.C = val1 >= imm;

    // << signed overflow for subtraction >> \\
    const v1Signed = (val1 & SIGN_BIT) !== 0n;
    const immSigned = (imm & SIGN_BIT) !== 0n;
    const diffSigned = (diff & SIGN_BIT) !== 0n;
    cpu.flags.V = (v1Signed !== immSigned) && (v1Signed !== diffSigned);
  }

  static hex() {
    return 0x0D;
  }
}

export class Cmpr {
  constructor(rn, rm) {
    this.rn = rn;
    this.rm = rm;
  }

  execute(cpu) {
    const val1 = cpu.registers.read(this.rn);
    const val2 = cpu.registers.read(this.rm);
    const rawDiff = val1 - val2;
    const diff = rawDiff & MASK_64;

    // << cmp is essentially a sub that discards the result but updates flags >> \\
    cpu.flags.Z = diff === 0n;
    cpu.flags.N = (diff & SIGN_BIT) !== 0n;
    
    // << for comparison/subtraction, carry acts as 'not-borrow' (val1 >= imm) >> \\
    cpu.flags.C = val1 >= val2;

    // << signed overflow for subtraction >> \\
    const v1Signed = (val1 & SIGN_BIT) !== 0n;
    const immSigned = (val2 & SIGN_BIT) !== 0n;
    const diffSigned = (diff & SIGN_BIT) !== 0n;
    cpu.flags.V = (v1Signed !== immSigned) && (v1Signed !== diffSigned);
  }
  
  static run(cpu, rn, rm) {
    const val1 = cpu.registers.read(rn);
    const val2 = cpu.registers.read(rm);
    const rawDiff = val1 - val2;
    const diff = rawDiff & MASK_64;

    // << cmp is essentially a sub that discards the result but updates flags >> \\
    cpu.flags.Z = diff === 0n;
    cpu.flags.N = (diff & SIGN_BIT) !== 0n;
    
    // << for comparison/subtraction, carry acts as 'not-borrow' (val1 >= imm) >> \\
    cpu.flags.C = val1 >= val2;

    // << signed overflow for subtraction >> \\
    const v1Signed = (val1 & SIGN_BIT) !== 0n;
    const immSigned = (val2 & SIGN_BIT) !== 0n;
    const diffSigned = (diff & SIGN_BIT) !== 0n;
    cpu.flags.V = (v1Signed !== immSigned) && (v1Signed !== diffSigned);
  }

  static hex() {
    return 0x0E;
  }
}

export class B {
  constructor(targetAddress) {
    this.targetAddress = targetAddress
  }

  execute(cpu) {
    cpu.pc = this.targetAddress;
  }
  
  static run(cpu, targetAddress) {
    cpu.pc = targetAddress;
  }

  static hex() {
    return 0x0F;
  }
}

export class Beq {
  constructor(targetAddress) {
    this.targetAddress = targetAddress
  }

  execute(cpu) {
    if (cpu.flags.Z) {
      cpu.pc = this.targetAddress;
    }
  }
  
  static run(cpu, targetAddress) {
    if (cpu.flags.Z) {
      cpu.pc = targetAddress;
    }
  }

  static hex() {
    return 0x10;
  }
}

export class Bne {
  constructor(targetAddress) {
    this.targetAddress = targetAddress
  }

  execute(cpu) {
    if (!cpu.flags.Z) {
      cpu.pc = this.targetAddress;
    }
  }
  
  static run(cpu, targetAddress) {
    if (!cpu.flags.Z) {
      cpu.pc = targetAddress;
    }
  }

  static hex() {
    return 0x11;
  }
}

export class Blt {
  constructor(targetAddress) {
    this.targetAddress = targetAddress
  }

  execute(cpu) {
    if (cpu.flags.N !== cpu.flags.V) {
      cpu.pc = this.targetAddress;
    }
  }
  
  static run(cpu, targetAddress) {
    if (cpu.flags.N !== cpu.flags.V) {
      cpu.pc = targetAddress;
    }
  }

  static hex() {
    return 0x12;
  }
}

export class Ble {
  constructor(targetAddress) {
    this.targetAddress = targetAddress
  }

  execute(cpu) {
    if (cpu.flags.Z || cpu.flags.N !== cpu.flags.V) {
      cpu.pc = this.targetAddress;
    }
  }
  
  static run(cpu, targetAddress) {
    if (cpu.flags.Z || cpu.flags.N !== cpu.flags.V) {
      cpu.pc = targetAddress;
    }
  }

  static hex() {
    return 0x13;
  }
}

export class Bgt {
  constructor(targetAddress) {
    this.targetAddress = targetAddress
  }

  execute(cpu) {
    if (!cpu.flags.Z && cpu.flags.N === cpu.flags.V) {
      cpu.pc = this.targetAddress;
    }
  }
  
  static run(cpu, targetAddress) {
    if (!cpu.flags.Z && cpu.flags.N === cpu.flags.V) {
      cpu.pc = targetAddress;
    }
  }

  static hex() {
    return 0x14;
  }
}

export class Bge {
  constructor(targetAddress) {
    this.targetAddress = targetAddress
  }

  execute(cpu) {
    if (cpu.flags.N === cpu.flags.V) {
      cpu.pc = this.targetAddress;
    }
  }
  
  static run(cpu, targetAddress) {
    if (cpu.flags.N === cpu.flags.V) {
      cpu.pc = targetAddress;
    }
  }

  static hex() {
    return 0x15;
  }
}

export class Bl {
  constructor(targetAddress) {
    this.targetAddress = targetAddress;
  }

  execute(cpu) {
    cpu.pc = this.targetAddress;
    cpu.sp = cpu.pc + 1;
  }
  
  static run(cpu, targetAddress) {
    cpu.pc = targetAddress;
    cpu.sp = cpu.pc + 1;
  }
  static hex() {
    return 0x016;
  }
}

export class Ret {
  constructor() {
  }

  execute(cpu) {
    cpu.pc = cpu.sp;
  }
  
  static run(cpu) {
    cpu.pc = cpu.sp;
  }
  
  static hex() {
    return 0x017;
  }
}