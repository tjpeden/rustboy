use super::Memory;

pub struct RandomAccessMemory {
  data: Vec<u8>,
}

impl RandomAccessMemory {
  pub fn new(size: usize) -> Self {
    RandomAccessMemory {
      data: vec![0; size],
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
    println!("Writing {:#04x} to {:#06x}", value, address);
    self.data[address as usize] = value;
  }
}
