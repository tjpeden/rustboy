use std::ops;

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

pub const REG_AF: WordRegister = WordRegister(REG_A_INDEX);
pub const REG_BC: WordRegister = WordRegister(REG_B_INDEX);
pub const REG_DE: WordRegister = WordRegister(REG_D_INDEX);
pub const REG_HL: WordRegister = WordRegister(REG_H_INDEX);

pub struct Registers {
  value: [u8; NUM_GPR],
  pc: u16,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      value: [0; NUM_GPR],
      pc: 0,
    }
  }

  pub fn read_pc(&self) -> u16 {
    self.pc
  }

  pub fn increment_pc(&mut self, value: i16) {
    self.pc = (self.pc as i16 + value) as u16;
  }
}

impl Memory for Registers {
  type B = ByteRegister;
  type W = WordRegister;

  fn read_byte(&mut self, address: ByteRegister) -> u8 {
    let ByteRegister(index) = address;

    self.value[index]
  }

  fn write_byte(&mut self, address: ByteRegister, value: u8) {
    let ByteRegister(index) = address;

    self.value[index] = value;
  }
}
