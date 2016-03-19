mod opcode;
mod registers;
mod instruction;

use std::fmt;

use super::memory::{Memory, IO_BASE_REG};

use self::opcode::*;
use self::registers::*;
use self::instruction::*;

pub struct Processor<M: Memory> {
  registers: Registers, // General Purpose Registers

  memory: M,
}

impl<M: Memory> Processor<M> {
  pub fn new(memory: M) -> Self {
    Processor {
      registers: Registers::new(),

      memory: memory,
    }
  }

  pub fn step(&mut self) {
    let instruction = self.read_instruction();

    self.execute_instruction(instruction);
  }

  fn read_instruction(&mut self) -> Instruction {
    let pc = self.registers.get_program_counter();
    let address = M::B::from(pc);

    let instruction = Instruction(self.memory.read_byte(address));

    println!("PC: {:#06x}: {:?}", pc, instruction);

    self.registers.increment_program_counter(1);

    instruction
  }

  fn read_special_instruction(&mut self) -> SpecialInstruction {
    let pc = self.registers.get_program_counter();
    let address = M::B::from(pc);

    let special_instruction = SpecialInstruction(self.memory.read_byte(address));

    println!("PC: {:#06x}: {:?}", pc, special_instruction);

    self.registers.increment_program_counter(1);

    special_instruction
  }

  fn read_immediate_byte(&mut self) -> u8 {
    let pc = self.registers.get_program_counter();
    let address = M::B::from(pc);
    let immediate = self.memory.read_byte(address);

    self.registers.increment_program_counter(1);

    immediate
  }

  fn read_immediate_word(&mut self) -> u16 {
    let pc = self.registers.get_program_counter();
    let address = M::W::from(pc);
    let immediate = self.memory.read_word(address);

    self.registers.increment_program_counter(2);

    immediate
  }

  fn stack_push(&mut self, value: u16) {
    let sp = self.registers.get_stack_pointer();
    let address = M::W::from(sp - 1);

    self.registers.decrement_stack_pointer(2);
    self.memory.write_word(address, value);
  }

  fn stack_pop(&mut self) -> u16 {
    let sp = self.registers.get_stack_pointer();
    let address = M::W::from(sp + 1);

    self.registers.increment_stack_pointer(2);
    self.memory.read_word(address)
  }

  fn execute_instruction(&mut self, instruction: Instruction) {
    match instruction.opcode() {
      /***** Control *****/
      Opcode::NoOp /* 0x00 */ => {
        // No operation
      }

      Opcode::Special /* 0xCB */ => {
        let special_instruction = self.read_special_instruction();

        self.execute_special_instruction(special_instruction);
      }

      /***** Flow Control *****/
      Opcode::JumpRelative /* 0x18 */ => {
        let offset = self.read_immediate_byte();

        self.registers.increment_program_counter((offset as i8) as i16);
      }

      Opcode::JumpRelativeNonZero /* 0x20 */ => {
        let offset = self.read_immediate_byte();

        if !self.registers.get_flag(ZERO_FLAG) {
          self.registers.increment_program_counter((offset as i8) as i16);
        }
      }

      Opcode::JumpRelativeZero /* 0x28 */ => {
        let offset = self.read_immediate_byte();

        if self.registers.get_flag(ZERO_FLAG) {
          self.registers.increment_program_counter((offset as i8) as i16);
        }
      }

      Opcode::Return /* 0xC9 */ => {
        let value = self.stack_pop();

        self.registers.set_program_counter(value);
      }

      Opcode::CallAddrImm /* 0xCD */ => {
        let value = self.read_immediate_word();
        let pc = self.registers.get_program_counter();

        self.stack_push(pc);
        self.registers.set_program_counter(value);
      }

      /***** 8-bit Load *****/
      Opcode::LoadAIntoC /* 0x4F */ => {
        let a = self.registers.read_byte(REG_A);

        self.registers.write_byte(REG_C, a);
      }

      Opcode::LoadAIntoD /* 0x57 */ => {
        let a = self.registers.read_byte(REG_A);

        self.registers.write_byte(REG_D, a);
      }

      Opcode::LoadAIntoH /* 0x67 */ => {
        let a = self.registers.read_byte(REG_A);

        self.registers.write_byte(REG_H, a);
      }

      Opcode::LoadEIntoA /* 0x7B */ => {
        let e = self.registers.read_byte(REG_E);

        self.registers.write_byte(REG_A, e);
      }

      Opcode::LoadImmIntoB /* 0x06 */ => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_B, value);
      }

      Opcode::LoadImmIntoC /* 0x0E */ => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_C, value);
      }

      Opcode::LoadImmIntoE /* 0x1E */ => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_E, value);
      }

      Opcode::LoadImmIntoL /* 0x2E */ => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_L, value);
      }

      Opcode::LoadImmIntoA /* 0x3E */ => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_A, value);
      }

      Opcode::LoadAIntoAddrC /* 0xE2 */ => {
        let a = self.registers.read_byte(REG_A);
        let c = self.registers.read_byte(REG_C);
        let address = M::B::from(IO_BASE_REG + c as u16);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAIntoAddrBc /* 0x02 */ => {
        let a = self.registers.read_byte(REG_A);
        let bc = self.registers.read_word(REG_BC);
        let address = M::B::from(bc);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAIntoAddrHl /* 0x77 */ => {
        let a = self.registers.read_byte(REG_A);
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAddrBcIntoA /* 0x0A */ => {
        let bc = self.registers.read_word(REG_BC);
        let address = M::B::from(bc);
        let value = self.memory.read_byte(address);

        self.registers.write_byte(REG_A, value);
      }

      Opcode::LoadAddrDeIntoA /* 0x1A */ => {
        let de = self.registers.read_word(REG_DE);
        let address = M::B::from(de);
        let value = self.memory.read_byte(address);

        self.registers.write_byte(REG_A, value);
      }

      Opcode::LoadAIntoAddrImm /* 0xEA */ => {
        let a = self.registers.read_byte(REG_A);
        let value = self.read_immediate_word();
        let address = M::B::from(value);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAintoAddrHlAndInc /* 0x22 */ => {
        let a = self.registers.read_byte(REG_A);
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);

        self.memory.write_byte(address, a);
        self.registers.increment_word(REG_HL);
      }

      Opcode::LoadAIntoAddrHlAndDec /* 0x32 */ => {
        let a = self.registers.read_byte(REG_A);
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);

        self.memory.write_byte(address, a);
        self.registers.decrement_word(REG_HL);
      }

      Opcode::LoadAIntoAddrImmIO /* 0xE0 */ => {
        let a = self.registers.read_byte(REG_A);
        let value = self.read_immediate_byte();
        let address = M::B::from(IO_BASE_REG + value as u16);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAddrImmIOIntoA /* 0xF0 */ => {
        let value = self.read_immediate_byte();
        let address = M::B::from(IO_BASE_REG + value as u16);
        let value = self.memory.read_byte(address);

        self.registers.write_byte(REG_A, value);
      }

      /***** 16-bit Load *****/
      Opcode::LoadImmIntoDe /* 0x11 */ => {
        let immediate = self.read_immediate_word();

        self.registers.write_word(REG_DE, immediate);
      }

      Opcode::LoadImmIntoHl /* 0x21 */ => {
        let immediate = self.read_immediate_word();

        self.registers.write_word(REG_HL, immediate);
      }

      Opcode::LoadImmIntoSp /* 0x31 */ => {
        let address = self.read_immediate_word();

        self.registers.set_stack_pointer(address);
      }

      Opcode::PopBc /* 0xC1 */ => {
        let value = self.stack_pop();

        self.registers.write_word(REG_BC, value);
      }

      Opcode::PushBc /* 0xC5 */ => {
        let bc = self.registers.read_word(REG_BC);

        self.stack_push(bc);
      }

      Opcode::IncrementB /* 0x04 */ => {
        self.registers.increment_byte(REG_B);
      }

      Opcode::IncrementC /* 0x0C */ => {
        self.registers.increment_byte(REG_C);
      }

      Opcode::DecrementB /* 0x05 */ => {
        self.registers.decrement_byte(REG_B);
      }

      Opcode::DecrementC /* 0x0D */ => {
        self.registers.decrement_byte(REG_C);
      }

      Opcode::DecrementA /* 0x3D */ => {
        self.registers.decrement_byte(REG_A);
      }

      Opcode::SubtractL /* 0x95 */ => {
        let a = self.registers.read_byte(REG_A);
        let l = self.registers.read_byte(REG_L);
        let result = self.subtract(a, l);

        self.registers.write_byte(REG_A, result);
      }

      Opcode::XorA /* 0xAF */ => {
        let a = self.registers.read_byte(REG_A);
        let result = a ^ a;

        self.registers.set_flag(ZERO_FLAG, result == 0);
        self.registers.write_byte(REG_A, result);
      }

      Opcode::CompareImm /* 0xFE */ => {
        let a = self.registers.read_byte(REG_A);
        let value = self.read_immediate_byte();
        let _ = self.subtract(a, value);
      }

      /***** 16-bit Math *****/
      Opcode::IncrementDe /* 0x13 */ => {
        self.registers.increment_word(REG_DE);
      }

      Opcode::IncrementHl /* 0x23 */ => {
        self.registers.increment_word(REG_HL);
      }

      /***** 8-bit Shift/Rotate *****/
      Opcode::RotateLeftA /* 0x17 */ => {
        let a = self.registers.read_byte(REG_A);
        let carry = self.registers.get_flag(CARRY_FLAG);
        let value = self.shift_left(a, carry);

        self.registers.write_byte(REG_A, value);
      }
    }
  }

  fn execute_special_instruction(&mut self, special_instruction: SpecialInstruction) {
    match special_instruction.opcode() {
      SpecialOpcode::RotateLeftC /* 0x11 */ => {
        let c = self.registers.read_byte(REG_C);
        let carry = self.registers.get_flag(CARRY_FLAG);
        let value = self.shift_left(c, carry);

        self.registers.write_byte(REG_C, value);
      }

      SpecialOpcode::Bit7H /* 0x7C */ => {
        let h = self.registers.read_byte(REG_H);

        self.registers.set_flag(ZERO_FLAG, (h & 0x80) == 0);
        self.registers.set_flag(SUBTRACT_FLAG, false);
        self.registers.set_flag(HALF_CARRY_FLAG, true);
      }
    }
  }

  fn shift_left(&mut self, value: u8, lsb: bool) -> u8 {
    let carry = (value & 0x80) != 0;
    let mut result = value << 1;

    if lsb { result |= 1; }

    self.registers.set_flag(ZERO_FLAG, result == 0);
    self.registers.set_flag(CARRY_FLAG, carry);

    result
  }
}

impl<M: Memory> fmt::Debug for Processor<M> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CPU Registers\n{:?}", self.registers)
  }
}
