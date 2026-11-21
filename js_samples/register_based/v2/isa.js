// ॐ श्री गणेशाय नम \\
// ।। ॐ नमः शिवाय ।। \\
// @date_start 19th November, 2026

/**
 * inst 0x00
 * ++ expected structure:
 *  - exit            [ pc]
 */
export function exit(vm, program) {
  vm.state.isRunning = false;
  vm.pc++;
}

/**
 * inst 0x01
 * ++ expected structure:
 *  - movi rd, imm    [ pc, pc + 1, pc + 2 ]
 */
export function movi(vm, program) {
  const rd = program[vm.pc + 1];
  const imm = program[vm.pc + 2];
  vm.registers.write(rd, imm);
  vm.pc += 3;
}

/**
 * inst 0x02
 * ++ expected structure:
 *   - movr rd, rm
 */
export function movr(vm, program) {
  const rd = program[vm.pc + 1];
  const rm = program[vm.pc + 2];
  vm.registers.write(rd, vm.registers.read(rm));
  vm.pc += 3;
}

/**
 * ++ Operation codes:
 *   - Add: 1
 *   - Sub: 2
 *   - Mul: 3
 *   - Div: 4
 *   - Rem: 5
 */
export const op = {
  add: 0x01,
  sub: 0x02,
  mul: 0x03,
  div: 0x04,
  rem: 0x05
}

/**
 * inst 0x03
 * ++ expected structure:
 *   - cali rd, rm, imm, op      [ pc, pc + 1, pc + 2, pc + 3, pc + 4 ]
 */
export function cali(vm, program) {
  const rd = program[vm.pc + 1];
  const valueRM = vm.registers.read(program[vm.pc + 2]);
  const imm = program[vm.pc + 3];
  const op = program[vm.pc + 4];
  let value = 0;
  switch (op) {
    case 1: value = valueRM + imm; break;
    case 2: value = valueRM - imm; break;
    case 3: value = valueRM * imm; break;
    case 4: value = valueRM / imm; break;
    case 5: value = valueRM % imm; break;
    default: console.error(`VMError::OperationCodeError: Invalid token at cali: ${op}.`);
  }
  vm.registers.write(rd, value);
  vm.pc += 5;
}

/**
 * inst 0x04
 * ++ expected structure:
 *   - calr rd, rm, rn, op      [ pc, pc + 1, pc + 2, pc + 3, pc + 4 ]
 */
export function calr(vm, program) {
  const rd = program[vm.pc + 1];
  const valueRM = vm.registers.read(program[vm.pc + 2]);
  const valueRN = vm.registers.read(program[vm.pc + 3]);
  const op = program[vm.pc + 4];
  let value = 0;
  switch (op) {
    case 1: value = valueRM + valueRN; break;
    case 2: value = valueRM - valueRN; break;
    case 3: value = valueRM * valueRN; break;
    case 4: value = valueRM / valueRN; break;
    case 5: value = valueRM % valueRN; break;
    default: console.error(`VMError::OperationCodeError: Invalid token at calr: ${op}.`);
  }
  vm.registers.write(rd, value);
  vm.pc += 5;
}

/**
 * ++ Branching Condition Opcodes:
 *  - Eq: 1
 *  - Ne: 2
 *  - Gt: 3
 *  - Lt: 4
 *  - Ge: 5
 *  - Le: 6
 */

export const cd = {
  eq: 0x01,
  ne: 0x02,
  gt: 0x03,
  lt: 0x04,
  ge: 0x05,
  le: 0x06
}

/**
 * inst 0x05
 * ++ expected structure:
 *   - b ta     [ pc, pc + 1 ]
 */
export function b(vm, program) {
  const ta = program[vm.pc + 1];
  vm.pc = ta;
}

/**
 * inst 0x06
 * ++ expected structure:
 *   - bcri ta, rm, imm, cd   [ pc, pc + 1, pc + 2, pc + 3, pc + 4 ]
 */
export function bcri(vm, program) {
  const ta = program[vm.pc + 1];
  const valueRM = vm.registers.read(program[vm.pc + 2]);
  const imm = program[vm.pc + 3];
  const cd = program[vm.pc + 4];
  let isTrue = false;
  switch (cd) {
    case 1: isTrue = valueRM === imm; break;
    case 2: isTrue = valueRM !== imm; break;
    case 3: isTrue = valueRM > imm;   break;
    case 4: isTrue = valueRM < imm;   break;
    case 5: isTrue = valueRM >= imm;  break;
    case 6: isTrue = valueRM <= imm;  break;
    default: console.error(`VMError::ConditionCodeError: invalid condition code found at 'bcri': ${cd}!`);
  }
  if (isTrue) {
    vm.pc = ta;
  } else {
    vm.pc += 5;
  }
}

/**
 * inst 0x07
 * ++ expected structure:
 *   - bcrr ta, rm, rn, cd    [ pc, pc + 1, pc + 2, pc + 3, pc + 4 ]
 */
export function bcrr(vm, program) {
  const ta = program[vm.pc + 1];
  const valueRM = vm.registers.read(program[vm.pc + 2]);
  const valueRN = vm.registers.read(program[vm.pc + 3]);
  const cd = program[vm.pc + 4];
  let isTrue = false;
  switch (cd) {
    case 1: isTrue = valueRM === valueRN; break;
    case 2: isTrue = valueRM !== valueRN; break;
    case 3: isTrue = valueRM > valueRN; break;
    case 4: isTrue = valueRM < valueRN; break;
    case 5: isTrue = valueRM >= valueRN; break;
    case 6: isTrue = valueRM <= valueRN; break;
    default: console.error(`VMError::ConditionCodeError: invalid condition code found at 'bcrr': ${cd}!`);
  }
  if (isTrue) {
    vm.pc = ta;
  } else {
    vm.pc += 5;
  }
}

export const inst = {
  exit:  0x00,
  movi:  0x01,
  movr:  0x02,
  cali: 0x03,
  calr: 0x04,
  b:     0x05,
  bcri:  0x06,
  bcrr:  0x07
}

export const dispatchTable = [
  exit,
  movi,
  movr,
  cali,
  calr,
  b,
  bcri,
  bcrr
];

