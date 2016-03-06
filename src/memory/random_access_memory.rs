use super::Memory;
use super::memory_map::RAM_SIZE;

use std::ops::{Deref, DerefMut};

pub struct RandomAccessMemory {
  data: [u8; RAM_SIZE as usize],
}

impl RandomAccessMemory {
  pub fn new() -> RandomAccessMemory {
    RandomAccessMemory {
      data: [0; RAM_SIZE as usize],
    }
  }
}

impl Deref for RandomAccessMemory {
  type Target = [u8; RAM_SIZE as usize];

  fn deref(&self) -> &[u8; RAM_SIZE as usize] {
    &self.data
  }
}

impl DerefMut for RandomAccessMemory {
  fn deref_mut(&mut self) -> &mut [u8; RAM_SIZE as usize] {
    &mut self.data
  }
}

impl Memory for RandomAccessMemory {
  type B = u16;
  type W = u16;

  fn read_byte(&mut self, address: u16) -> u8 {
    self[address as usize]
  }

  fn write_byte(&mut self, address: u16, value: u8) {
    self[address as usize] = value;
  }
}
