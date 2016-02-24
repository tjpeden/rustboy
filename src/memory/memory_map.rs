use super::Memory;
use super::ram::Ram;

const BOOTROM_START: u16 = 0x0000;
const BOOTROM_SIZE: u16 = 0xFF;
const BOOTROM_END: u16 = BOOTROM_START + BOOTROM_SIZE - 1;

const RAM_START: u16 = 0xC000;
pub const RAM_SIZE: u16 = 0x2000;
const RAM_END: u16 = RAM_START + RAM_SIZE - 1;

enum Address {
  Bootrom(u16),
  Ram(u16),
}

pub struct MemoryMap {
  pub bootrom: Box<[u8]>,
  pub ram: Ram,
}

impl MemoryMap {
  pub fn new(bootrom: Box<[u8]>) -> MemoryMap {
    MemoryMap {
      bootrom: bootrom,
      ram: Ram::new(),
    }
  }

  fn map_address(&self, address: u16) -> Address {
    match address {
      BOOTROM_START ... BOOTROM_END => {
        Address::Bootrom(address - BOOTROM_START)
      },
      RAM_START ... RAM_END => {
        Address::Ram(address - RAM_START)
      },
      _ => {
        panic!("Unrecognized address: {:#010x}", address);
      }
    }
  }
}

impl Memory for MemoryMap {
  fn read_byte(&mut self, address: u16) -> u8 {
    match self.map_address(address) {
      Address::Bootrom(offset) => self.bootrom[offset as usize],
      Address::Ram(offset) => self.ram[offset as usize],
    }
  }

  fn write_byte(&mut self, address: u16, value: u8) {
    match self.map_address(address) {
      Address::Bootrom(offset) => self.bootrom[offset as usize] = value,
      Address::Ram(offset) => self.ram[offset as usize] = value,
    }
  }
}
