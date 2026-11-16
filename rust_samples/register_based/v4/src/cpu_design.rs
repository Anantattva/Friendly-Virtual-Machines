// ।। ॐ नमः शिवाय ।। 
// @date_start 5th November, 2026

///// ++++++++++++++++++++++++++++ \\\\\
///// ===== Registers struct ===== \\\\\
///// ++++++++++++++++++++++++++++ \\\\\

pub struct Registers {
  pub x: [u64; 31],
}

impl Registers {
  pub fn new() -> Self {
    Self {
      x: [0; 31],
    }
  }
}

impl Registers {
  pub fn read(&self, index: usize) -> u64 {
    // << register zero (xzr) always return zero >> \\
    if index == 31 {
      return 0;
    }
    // << @design: compiler frontend will ensure index always ranges from 0 to 31 >> \\
    // << @optimization: so we use `unsafe` safely to extract more performance >> \\
    unsafe {
      *self.x.get_unchecked(index)
    }
  }
  
  pub fn write(&mut self, index: usize, new_value: u64) {
    // << writes to register zero (xzr) are discarded >> \\
    if index == 31 {
      return;
    }
    // << @design: compiler frontend will ensure index always ranges from 0 to 31 >> \\
    // << @optimization: so we use `unsafe` safely to extract more performance >> \\
    unsafe {
      *self.x.get_unchecked_mut(index) = new_value;
    }
  }
}


///// ++++++++++++++++ \\\\\
// ++== Flags struct ==++ \\
///// ++++++++++++++++ \\\\\

pub struct Flags {
  pub zero_flag: bool,     // 'Z': result was zero
  pub negative_flag: bool, // 'N': result's top bit was 1 (signed negative)
  pub carry_flag: bool,    // 'C': for SUB, 1 = no borrow; for ADD, 1 = carry out
  pub overflow_flag: bool, // 'V': signed overflow (result sign != expected)
}

impl Flags {
  pub fn new() -> Self {
    Self {
      zero_flag: false,
      negative_flag: false,
      carry_flag: false,
      overflow_flag: false,
    }
  }
}

impl Flags {
  pub fn reset(&mut self) {
    self.zero_flag = false;
    self.negative_flag = false;
    self.carry_flag = false;
    self.overflow_flag = false;
  }
}


///// ++++++++++++++++++++++++++++ \\\\\
///// +++++++ State struct +++++++ \\\\\
///// ++++++++++++++++++++++++++++ \\\\\

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

impl State {
  pub fn reset(&mut self) {
    self.is_running = false;
  }
}