// ।। ॐ नमः शिवाय ।। \\
// @date_start 7th November, 2026

#![allow(warnings)]

// ==== Memory ==== \\
pub struct Memory {
  pub stack: Vec<u8>,
  pub heap: Vec<u8>,
}

impl Memory {
  pub fn new(stack_size: usize, heap_size: usize) -> Self {
    Self {
      stack: vec![0; stack_size],
      heap: vec![0; heap_size],
    }
  }
}
// ==== Registers ==== \\

pub struct Registers {
  pub x: RegistersX,   // u64 registers
  pub v: RegistersV,   // f64 registers
  pub sp: usize,       // stack pointer
  pub hp: usize,       // heap pointer
}

impl Registers {
  pub fn new() -> Self {
    Self {
      x: RegistersX::new(),
      v: RegistersV::new(),
      sp: 0,
      hp: 0,
    }
  }
}

pub struct RegistersX(pub [u64; 32]);

impl RegistersX {
  pub fn new() -> Self {
    Self([0; 32])
  }
  #[inline(always)]
  pub fn read(&self, index: usize) -> u64 {
    unsafe {
      if index == 31 {
        return 0; // xzr/x31 or zero register always reads zero
      } else if index < 31 {
        return *self.0.get_unchecked(index);
      } else {
        panic!("CPUError::RegistersError::RegistersXError:\nOut of bounds error. Tried to read register `{}`, which exceed maximum bound of 31.", index);
      }
    }
  }
  #[inline(always)]
  pub fn write(&mut self, index: usize, new_value: u64) {
    unsafe {
      if index == 31 {
        return; // writes to xzr/x31 or zero register are always discarded
      } else if index < 31 {
        *self.0.get_unchecked_mut(index) = new_value;
      } else {
        panic!("CPUError::RegistersError::RegistersXError:\nOut of bounds error. Tried to read register `{}`, which exceed maximum bound of 31.", index);
      }
    }
  }
}

pub struct RegistersV(pub [f64; 32]);

impl RegistersV {
  pub fn new() -> Self {
    Self([0.0; 32])
  }
  #[inline(always)]
  pub fn read(&self, index: usize) -> f64 {
    unsafe {
      if index < 32 {
        return *self.0.get_unchecked(index);
      } else {
        panic!("CPUError::RegistersError::RegistersVError:\nOut of bounds error. Tried to read register `{}`, which exceed maximum bound of 31.", index);
      }
    }
  }
  #[inline(always)]
  pub fn write(&mut self, index: usize, new_value: f64) {
     unsafe { 
      if index < 32 {
        *self.0.get_unchecked_mut(index) = new_value;
      } else {
        panic!("CPUError::RegistersError::RegistersVError:\nOut of bounds error. Tried to read register `{}`, which exceed maximum bound of 31.", index);
      }
    }
  }
}

// ==== State ==== \\

pub struct State {
  pub is_running: bool,
}

impl State {
  pub fn new() -> Self {
    Self {
      is_running: false,
    }
  }
}