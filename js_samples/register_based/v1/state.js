// ।। ॐ नमः शिवाय ।। \\
// @date_start 31st October, 2026

export class Registers {
  constructor() {
    this.x = new BigUint64Array(31);
  }
  read(index) {
    if (index == 31) {
      return BigInt(0);
    }
    return this.x[index];
  }

  write(index, newValue) {
    if (index == 31) {
      return;
    }
    this.x[index] = newValue;
  }
}

export class Flags {
  constructor() {
    this.N = false;
    this.Z = false;
    this.C = false;
    this.V = false;
  }
}

export class State {
  constructor() {
    this.isRunning = false;
  }
}