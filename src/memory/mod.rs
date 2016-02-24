mod ram;
mod memory_map;

pub use self::ram::Ram;
pub use self::memory_map::MemoryMap;

pub trait Memory {
  fn read_byte(&mut self, address: u16) -> u8;
  fn write_byte(&mut self, address: u16, value: u8);

  fn read_word(&mut self, address: u16) -> u16 {
    self.read_byte(address) as u16 | (self.read_byte(address + 1) as u16) << 8
  }

  fn write_word(&mut self, address: u16, value: u16) {
    self.write_byte(address, (value & 0xFF) as u8);
    self.write_byte(address + 1, ((value >> 8) & 0xFF) as u8);
  }
}
