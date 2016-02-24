use super::Memory;
use super::memory_map::RAM_SIZE;

use std::ops::{Deref, DerefMut};

pub struct Ram {
  data: [u8; RAM_SIZE as usize],
}

impl Ram {
  pub fn new() -> Ram {
    Ram {
      data: [0u8; RAM_SIZE as usize],
    }
  }
}

impl Deref for Ram {
  type Target = [u8; RAM_SIZE as usize];

  fn deref(&self) -> &[u8; RAM_SIZE as usize] {
    &self.data
  }
}

impl DerefMut for Ram {
  fn deref_mut(&mut self) -> &mut [u8; RAM_SIZE as usize] {
    &mut self.data
  }
}

impl Memory for Ram {
  fn read_byte(&mut self, address: u16) -> u8 {
    self[address as usize]
  }

  fn write_byte(&mut self, address: u16, value: u8) {
    self[address as usize] = value;
  }
}
