use super::Memory;

use std::ops::{Index, IndexMut};

pub struct RandomAccessMemory {
  data: Vec<u8>,
}

impl RandomAccessMemory {
  pub fn new() -> Self {
    RandomAccessMemory {
      data: Vec::new(),
    }
  }
}

impl Memory for RandomAccessMemory {
  type B = u16;
  type W = u16;

  fn read_byte(&mut self, address: u16) -> u8 {
    self.data[address as usize]
  }

  fn write_byte(&mut self, address: u16, value: u8) {
    self.data[address as usize] = value;
  }
}
