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

const IO_REG_START: u16 = 0xFF00;
const IO_REG_SIZE: u16 = 0x4C;
const IO_REG_END: u16 = IO_REG_START + IO_REG_SIZE - 1;

enum AddressType {
  Bootrom(u16),
  Ram(u16),
  Vram(u16),
  IoReg(u16),
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

  fn map_address(&self, address: u16) -> AddressType {
    match address {
      BOOTROM_START ... BOOTROM_END => {
        AddressType::Bootrom(address - BOOTROM_START)
      }

      RAM_START ... RAM_END => {
        AddressType::Ram(address - RAM_START)
      }

      VRAM_START ... VRAM_END => {
        AddressType::Vram(address - VRAM_START)
      }

      IO_REG_START ... IO_REG_END => {
        AddressType::IoReg(address - IO_REG_START)
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
      AddressType::Bootrom(offset) => self.bootrom[offset as usize],
      AddressType::Ram(offset) => self.ram.read_byte(offset),
      AddressType::Vram(offset) => self.vram.read_byte(offset),
      AddressType::IoReg(offset) => panic!("IO Register: {:#04x}", offset),
    }
  }

  fn write_byte(&mut self, address: u16, value: u8) {
    match self.map_address(address) {
      AddressType::Bootrom(_) => panic!("Cannot write to read-only memory."),
      AddressType::Ram(offset) => self.ram.write_byte(offset, value),
      AddressType::Vram(offset) => self.vram.write_byte(offset, value),
      AddressType::IoReg(offset) => panic!("IO Register: {:#04x} value: {}", offset, value),
    }
  }
}
