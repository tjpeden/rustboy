use std::ops;
use std::fmt;

use super::super::memory::Memory;

const NUM_GPR: usize = 8;

pub struct ByteRegister(usize);

impl From<u16> for ByteRegister {
  fn from(scalar: u16) -> Self {
    ByteRegister(scalar as usize)
  }
}

impl From<WordRegister> for ByteRegister {
  fn from(register: WordRegister) -> Self {
    let WordRegister(index) = register;

    ByteRegister(index)
  }
}

#[derive(Copy, Clone)]
pub struct WordRegister(usize);

impl ops::Add for WordRegister {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    let WordRegister(index) = self;
    let WordRegister(scalar) = rhs;

    WordRegister(index + scalar)
  }
}

impl From<u8> for WordRegister {
  fn from(scalar: u8) -> Self {
    WordRegister(scalar as usize)
  }
}

impl From<u16> for WordRegister {
  fn from(scalar: u16) -> Self {
    WordRegister(scalar as usize)
  }
}

impl From<ByteRegister> for WordRegister {
  fn from(register: ByteRegister) -> Self {
    let ByteRegister(index) = register;

    WordRegister(index)
  }
}

const REG_A_INDEX: usize = 0;
const REG_B_INDEX: usize = 2;
const REG_D_INDEX: usize = 4;
const REG_H_INDEX: usize = 6;

pub const REG_A: ByteRegister = ByteRegister(REG_A_INDEX);
pub const REG_F: ByteRegister = ByteRegister(REG_A_INDEX + 1);
pub const REG_B: ByteRegister = ByteRegister(REG_B_INDEX);
pub const REG_C: ByteRegister = ByteRegister(REG_B_INDEX + 1);
pub const REG_D: ByteRegister = ByteRegister(REG_D_INDEX);
pub const REG_E: ByteRegister = ByteRegister(REG_D_INDEX + 1);
pub const REG_H: ByteRegister = ByteRegister(REG_H_INDEX);
pub const REG_L: ByteRegister = ByteRegister(REG_H_INDEX + 1);

#[allow(dead_code)]
pub const REG_AF: WordRegister = WordRegister(REG_A_INDEX);
pub const REG_BC: WordRegister = WordRegister(REG_B_INDEX);
pub const REG_DE: WordRegister = WordRegister(REG_D_INDEX);
pub const REG_HL: WordRegister = WordRegister(REG_H_INDEX);

pub const ZERO_FLAG: u8 = 0x80;
pub const SUBTRACT_FLAG: u8 = 0x40;
pub const HALF_CARRY_FLAG: u8 = 0x20;
pub const CARRY_FLAG: u8 = 0x10;

pub struct Registers {
  value: [u8; NUM_GPR],
  sp: u16,
  pc: u16,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      value: [0; NUM_GPR],
      sp: 0,
      pc: 0,
    }
  }

  pub fn get_stack_pointer(&self) -> u16 {
    self.sp
  }

  pub fn set_stack_pointer(&mut self, value : u16) {
    self.sp = value;
  }

  pub fn increment_stack_pointer(&mut self, value: i16) {
    self.sp = (self.sp as i16 + value) as u16;
  }

  pub fn decrement_stack_pointer(&mut self, value: i16) {
    self.sp = (self.sp as i16 - value) as u16;
  }

  pub fn get_program_counter(&self) -> u16 {
    self.pc
  }

  pub fn set_program_counter(&mut self, value: u16) {
    self.pc = value;
  }

  pub fn increment_program_counter(&mut self, value: i16) {
    self.pc = (self.pc as i16 + value) as u16;
  }

  pub fn get_flag(&mut self, flag: u8) -> bool {
    let f = self.read_byte(REG_F);

    (f & flag) != 0
  }

  pub fn set_flag(&mut self, flag: u8, on: bool) {
    let mut f = self.read_byte(REG_F);

    if on {
      f |= flag;
    } else {
      f &= !flag;
    }

    self.write_byte(REG_F, f);
  }

  pub fn increment_byte(&mut self, register: ByteRegister) {
    let ByteRegister(index) = register;
    let previous = self.value[index];

    self.set_flag(ZERO_FLAG, previous + 1 == 0);
    self.set_flag(SUBTRACT_FLAG, false);
    self.set_flag(HALF_CARRY_FLAG, ((previous & 0xF) + 1) & 0x10 != 0);

    self.value[index] += 1;
  }

  pub fn decrement_byte(&mut self, register: ByteRegister) {
    let ByteRegister(index) = register;
    let value = self.value[index];

    self.set_flag(ZERO_FLAG, (value.wrapping_sub(1)) == 0);
    self.set_flag(SUBTRACT_FLAG, true);
    self.set_flag(HALF_CARRY_FLAG, ((value & 0xF).wrapping_sub(1)) & 0x10 != 0);

    self.write_byte(register, value.wrapping_sub(1));
  }

  pub fn increment_word(&mut self, register: WordRegister) {
    let value = self.read_word(register);
    self.write_word(register, value + 1);
  }

  pub fn decrement_word(&mut self, register: WordRegister) {
    let value = self.read_word(register);
    self.write_word(register, value - 1);
  }
}

impl Memory for Registers {
  type B = ByteRegister;
  type W = WordRegister;

  fn read_byte(&self, register: ByteRegister) -> u8 {
    let ByteRegister(index) = register;

    self.value[index]
  }

  fn read_word(&self, address: WordRegister) -> u16 {
    (self.read_byte(Self::B::from(address)) as u16) << 8 |
    self.read_byte(Self::B::from(address + Self::W::from(1u16))) as u16
  }

  fn write_byte(&mut self, register: ByteRegister, value: u8) {
    let ByteRegister(index) = register;

    self.value[index] = value;
  }

  fn write_word(&mut self, address: Self::W, value: u16) {
    self.write_byte(Self::B::from(address), ((value >> 8) & 0xFF) as u8);
    self.write_byte(Self::B::from(address + Self::W::from(1u16)), (value & 0xFF) as u8);
  }
}

/* Reference
impl fmt::Debug for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CPU {{ registers: {} }}", self.registers)
  }
}
*/

impl fmt::Debug for Registers {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // const BYTE_REGISTERS: [&'static str; NUM_GPR] = [
    //   "A", "F", "B", "C", "D", "E", "H", "L"
    // ];
    const WORD_REGISTERS: [&'static str; NUM_GPR/2] = [
      "AF", "BC", "DE", "HL"
    ];

    // for index in 0..NUM_GPR {
    //   let value = self.read_byte(ByteRegister(index));
    //   try!(
    //     writeln!(f, "  {name}: {value:#04x}", name = BYTE_REGISTERS[index], value = value)
    //   );
    // }
    //
    // try!(writeln!(f, ""));

    for index in 0..NUM_GPR/2 {
      let value = self.read_word(WordRegister(index * 2));
      try!(
        writeln!(f, "  {name}: {value:#06x}", name = WORD_REGISTERS[index], value = value)
      )
    }

    try!(writeln!(f, "  SP: {:#06x}", self.sp));
    try!(writeln!(f, "  PC: {:#06x}", self.pc));

    writeln!(f, "")
  }
}
