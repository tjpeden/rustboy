use super::Memory;
use super::random_access_memory::RandomAccessMemory;

const BOOTROM_START: u16 = 0x0000;
const BOOTROM_SIZE: u16 = 0xFF;
const BOOTROM_END: u16 = BOOTROM_START + BOOTROM_SIZE - 1;

const RAM_START: u16 = 0xC000;
pub const RAM_SIZE: u16 = 0x2000;
const RAM_END: u16 = RAM_START + RAM_SIZE - 1;

const VRAM_START: u16 = 0x8000;
const VRAM_SIZE: u16 = 0x2000;
const VRAM_END: u16 = VRAM_START + VRAM_SIZE - 1;

enum Address {
  Bootrom(u16),
  Ram(u16),
  Vram(u16),
}

pub struct MemoryMap {
  bootrom: Box<[u8]>,
  ram: RandomAccessMemory,
  vram: RandomAccessMemory,
}

impl MemoryMap {
  pub fn new(bootrom: Box<[u8]>) -> MemoryMap {
    MemoryMap {
      bootrom: bootrom,
      ram: RandomAccessMemory::new(),
      vram: RandomAccessMemory::new(), // this is a little bit of a hack, just because I can't use VRAM_SIZE
    }
  }

  fn map_address(&self, address: u16) -> Address {
    match address {
      BOOTROM_START ... BOOTROM_END => {
        Address::Bootrom(address - BOOTROM_START)
      }

      RAM_START ... RAM_END => {
        Address::Ram(address - RAM_START)
      }

      VRAM_START ... VRAM_END => {
        Address::Vram(address - VRAM_START)
      }

      _ => {
        panic!("Unrecognized address: {:#06x}", address);
      }
    }
  }
}

impl Memory for MemoryMap {
  type B = u16;
  type W = u16;

  fn read_byte(&mut self, address: u16) -> u8 {
    match self.map_address(address) {
      Address::Bootrom(offset) => self.bootrom[offset as usize],
      Address::Ram(offset) => self.ram[offset as usize],
      Address::Vram(offset) => self.vram[offset as usize],
    }
  }

  fn write_byte(&mut self, address: u16, value: u8) {
    match self.map_address(address) {
      Address::Bootrom(_) => panic!("Cannot write to read-only memory."),
      Address::Ram(offset) => self.ram[offset as usize] = value,
      Address::Vram(offset) => self.vram[offset as usize] = value,
    }
  }
}
