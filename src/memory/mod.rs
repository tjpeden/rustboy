mod random_access_memory;
mod memory_map;

use std::ops;

pub use self::random_access_memory::RandomAccessMemory;
pub use self::memory_map::MemoryMap;
pub use self::memory_map::IO_BASE_REG;

pub trait Memory {
  type B: From<Self::W> + From<u16>;
  type W: ops::Add<Output=Self::W> + From<u16> + Copy;

  fn read_byte(&self, address: Self::B) -> u8;
  fn read_word(&self, address: Self::W) -> u16 {
    self.read_byte(Self::B::from(address)) as u16 |
    (self.read_byte(Self::B::from(address + Self::W::from(1))) as u16) << 8
  }

  fn write_byte(&mut self, address: Self::B, value: u8);
  fn write_word(&mut self, address: Self::W, value: u16) {
    self.write_byte(Self::B::from(address), (value & 0xFF) as u8);
    self.write_byte(Self::B::from(address + Self::W::from(1)), ((value >> 8) & 0xFF) as u8);
  }
}
