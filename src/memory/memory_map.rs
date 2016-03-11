use super::Memory;
use super::random_access_memory::RandomAccessMemory;

// use super::super::io_ports::IOPorts;

const BOOTROM_START: u16 = 0x0000;
const BOOTROM_SIZE: u16 = 0xFF;
const BOOTROM_END: u16 = BOOTROM_START + BOOTROM_SIZE - 1;

const GAMEROM_START: u16 = 0x0000;
const GAMEROM_SIZE: u16 = 0x4000;
const GAMEROM_END: u16 = GAMEROM_START + GAMEROM_SIZE - 1;

const VRAM_START: u16 = 0x8000;
const VRAM_SIZE: u16 = 0x2000;
const VRAM_END: u16 = VRAM_START + VRAM_SIZE - 1;

const RAM_START: u16 = 0xC000;
pub const RAM_SIZE: u16 = 0x2000;
const RAM_END: u16 = RAM_START + RAM_SIZE - 1;

const IO_REG_START: u16 = 0xFF00;
const IO_REG_SIZE: u16 = 0x4c;
const IO_REG_END: u16 = IO_REG_START + IO_REG_SIZE - 1;

const ZERO_PAGE_START: u16 = 0xFF80;
const ZERO_PAGE_SIZE: u16 = 0x7F;
const ZERO_PAGE_END: u16 = ZERO_PAGE_START + ZERO_PAGE_SIZE - 1;

pub const IO_BASE_REG: u16 = 0xFF00;
/* Save for later
const IO_NR_11_REG: u16 = 0xFF11;
const IO_NR_52_REG: u16 = 0xFF26;
*/

enum AddressType {
  Bootrom(u16),
  Gamerom(u16),
  ZeroPage(u16),
  Ram(u16),
  Vram(u16),
  IoReg(u16),
  // IoNr52Reg,
  // IoNr11Reg,
}

pub struct MemoryMap {
  bootrom: Box<[u8]>,
  gamerom: Box<[u8]>,
  zero_page: RandomAccessMemory,
  ram: RandomAccessMemory,
  vram: RandomAccessMemory,
  io: RandomAccessMemory,
  // io: IOPorts,
}

impl MemoryMap {
  pub fn new(bootrom: Box<[u8]>, gamerom: Box<[u8]>) -> MemoryMap {
    MemoryMap {
      bootrom: bootrom,
      gamerom: gamerom,
      zero_page: RandomAccessMemory::new(ZERO_PAGE_SIZE as usize),
      ram: RandomAccessMemory::new(RAM_SIZE as usize),
      vram: RandomAccessMemory::new(VRAM_SIZE as usize),
      io: RandomAccessMemory::new(IO_REG_SIZE as usize),
      // io: IOPorts::default(),
    }
  }

  fn map_address(&self, address: u16) -> AddressType {
    match address {
      BOOTROM_START ... BOOTROM_END => {
        AddressType::Bootrom(address - BOOTROM_START)
      }

      GAMEROM_START ... GAMEROM_END => {
        AddressType::Gamerom(address - GAMEROM_START)
      }

      RAM_START ... RAM_END => {
        AddressType::Ram(address - RAM_START)
      }

      VRAM_START ... VRAM_END => {
        AddressType::Vram(address - VRAM_START)
      }

      ZERO_PAGE_START ... ZERO_PAGE_END => {
        AddressType::ZeroPage(address - ZERO_PAGE_START)
      }

      IO_REG_START ... IO_REG_END => {
        AddressType::IoReg(address - IO_REG_START)
      }

      // IO_NR_11_REG => {
      //   AddressType::IoNr11Reg
      // }
      //
      // IO_NR_52_REG => {
      //   AddressType::IoNr52Reg
      // }

      _ => {
        panic!("Unrecognized address: {:#06x}", address);
      }
    }
  }
}

impl Memory for MemoryMap {
  type B = u16;
  type W = u16;

  fn read_byte(&self, address: u16) -> u8 {
    match self.map_address(address) {
      AddressType::Bootrom(offset) => self.bootrom[offset as usize],
      AddressType::Gamerom(offset) => self.gamerom[offset as usize],
      AddressType::ZeroPage(offset) => self.zero_page.read_byte(offset),
      AddressType::Ram(offset) => self.ram.read_byte(offset),
      AddressType::Vram(offset) => self.vram.read_byte(offset),
      AddressType::IoReg(offset) => self.io.read_byte(offset),
      // AddressType::IoNr11Reg => self.io.read_nr_11(),
      // AddressType::IoNr52Reg => self.io.read_nr_52(),
    }
  }

  fn write_byte(&mut self, address: u16, value: u8) {
    match self.map_address(address) {
      AddressType::Bootrom(_) => panic!("Cannot write to read-only memory."),
      AddressType::Gamerom(_) => panic!("Cannot write to read-only memory."),
      AddressType::ZeroPage(offset) => self.zero_page.write_byte(offset, value),
      AddressType::Ram(offset) => self.ram.write_byte(offset, value),
      AddressType::Vram(offset) => self.vram.write_byte(offset, value),
      AddressType::IoReg(offset) => self.io.write_byte(offset, value),
      // AddressType::IoNr11Reg => self.io.write_nr_11(value),
      // AddressType::IoNr52Reg => self.io.write_nr_52(value),
    }
  }
}
