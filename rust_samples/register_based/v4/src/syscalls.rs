// ।। ॐ नमः शिवाय ।। \\
// @date_start 5th November, 2026

use crate::cpu::CPU;

#[derive(Debug)]
pub enum SyscallsID {
  Write = 0     // write
}

impl CPU {
  pub fn call_system(&self, scid: &SyscallsID) {
    match scid {
      SyscallsID::Write => {
        let ptr = self.registers.read(1) as usize;
        let len = self.registers.read(2) as usize;

        if ptr + len < self.memory.len() {
          let string_bytes = self.memory[ptr..ptr + len];
          let output_string = String::from_utf8_lossy(&string_bytes);
          print!("{}", output_string);
        } else {
          panic!("OutOfMemoryError: Memory limit exceeded error caught at Syscall Write.\nAvailable memory size: {}. Max requested pointer: {}.", self.memory.len(), ptr + len);
        }
      }
    }
  }
}
