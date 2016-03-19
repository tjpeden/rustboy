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

  fn execute_instruction(&mut self, instruction: Instruction) {
    match instruction.opcode() {
      /*
        ***********
        * Control *
        ***********
      */

      Opcode::NoOp /* 0x00 */ => {
        // No operation
      }

      Opcode::Special /* 0xCB */ => {
        let special_instruction = self.read_special_instruction();

        self.execute_special_instruction(special_instruction);
      }

      /*
        ****************
        * Flow Control *
        ****************
      */

      Opcode::Jump /* 0xC3 */ => {
        let value = self.read_immediate_word();

        self.registers.set_program_counter(value);
      }

      Opcode::JumpNonZero /* 0xC2 */ => { self.jump_conditionally(ZERO_FLAG, false); }

      Opcode::JumpZero /* 0xCA */ => { self.jump_conditionally(ZERO_FLAG, true); }

      Opcode::JumpNonCarry /* 0xD2 */ => { self.jump_conditionally(CARRY_FLAG, false); }

      Opcode::JumpCarry /* 0xDA */ => { self.jump_conditionally(CARRY_FLAG, true); }

      Opcode::JumpRelative /* 0x18 */ => {
        let offset = self.read_immediate_byte();

        self.registers.increment_program_counter((offset as i8) as i16);
      }

      Opcode::JumpRelativeNonZero /* 0x20 */ => { self.jump_relative_conditionally(ZERO_FLAG, false); }

      Opcode::JumpRelativeZero /* 0x28 */ => { self.jump_relative_conditionally(ZERO_FLAG, true); }

      Opcode::JumpRelativeNonCarry /* 0x30 */ => { self.jump_relative_conditionally(CARRY_FLAG, false); }

      Opcode::JumpRelativeCarry /* 0x38 */ => { self.jump_relative_conditionally(CARRY_FLAG, true); }

      Opcode::Return /* 0xC9 */ => {
        let value = self.stack_pop();

        self.registers.set_program_counter(value);
      }

      Opcode::CallImmAddr /* 0xCD */ => {
        let value = self.read_immediate_word();
        let pc = self.registers.get_program_counter();

        self.stack_push(pc);
        self.registers.set_program_counter(value);
      }

      /*
        **************
        * 8-bit Load *
        **************
      */

      Opcode::LoadAIntoC /* 0x4F */ => { self.registers.transfer_byte(REG_A, REG_C); }

      Opcode::LoadAIntoD /* 0x57 */ => { self.registers.transfer_byte(REG_A, REG_D); }

      Opcode::LoadAIntoH /* 0x67 */ => { self.registers.transfer_byte(REG_A, REG_H); }

      Opcode::LoadEIntoA /* 0x7B */ => { self.registers.transfer_byte(REG_E, REG_A); }

      Opcode::LoadImmIntoB /* 0x06 */ => { self.load_immediate_byte(REG_B); }

      Opcode::LoadImmIntoC /* 0x0E */ => { self.load_immediate_byte(REG_C); }

      Opcode::LoadImmIntoD /* 0x16 */ => { self.load_immediate_byte(REG_D); }

      Opcode::LoadImmIntoE /* 0x1E */ => { self.load_immediate_byte(REG_E); }

      Opcode::LoadImmIntoH /* 0x26 */ => { self.load_immediate_byte(REG_H); }

      Opcode::LoadImmIntoL /* 0x2E */ => { self.load_immediate_byte(REG_L); }

      Opcode::LoadImmIntoA /* 0x3E */ => { self.load_immediate_byte(REG_A); }

      Opcode::LoadImmIntoAddrHl /* 0x36 */ => {
        let value = self.read_immediate_byte();
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);

        self.memory.write_byte(address, value);
      }

      Opcode::LoadAIntoAddrC /* 0xE2 */ => {
        let a = self.registers.read_byte(REG_A);
        let c = self.registers.read_byte(REG_C);
        let address = M::B::from(IO_BASE_REG + c as u16);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAIntoAddrBc /* 0x02 */ => { self.transfer_to_address(REG_A, REG_BC); }

      Opcode::LoadAIntoAddrDe /* 0x12 */ => { self.transfer_to_address(REG_A, REG_DE); }

      Opcode::LoadAIntoAddrHl /* 0x77 */ => { self.transfer_to_address(REG_A, REG_HL); }

      Opcode::LoadAddrBcIntoA /* 0x0A */ => { self.transfer_from_address(REG_BC, REG_A); }

      Opcode::LoadAddrDeIntoA /* 0x1A */ => { self.transfer_from_address(REG_DE, REG_A); }

      Opcode::LoadAIntoImmAddr /* 0xEA */ => { self.transfer_to_immediate_address(REG_A); }

      Opcode::LoadAIntoAddrHlAndInc /* 0x22 */ => {
        self.transfer_to_address(REG_A, REG_HL);
        self.registers.increment_word(REG_HL);
      }

      Opcode::LoadAddrHLIntoAAndInc /* 0x2A */ => {
        self.transfer_from_address(REG_HL, REG_A);
        self.registers.increment_word(REG_HL);
      }

      Opcode::LoadAIntoAddrHlAndDec /* 0x32 */ => {
        self.transfer_to_address(REG_A, REG_HL);
        self.registers.decrement_word(REG_HL);
      }

      Opcode::LoadAddrHlIntoAAndDec /* 0x3A */ => {
        self.transfer_from_address(REG_HL, REG_A);
        self.registers.decrement_word(REG_HL);
      }

      Opcode::LoadAIntoImmAddrIO /* 0xE0 */ => {
        let a = self.registers.read_byte(REG_A);
        let value = self.read_immediate_byte();
        let address = M::B::from(IO_BASE_REG + value as u16);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadImmAddrIOIntoA /* 0xF0 */ => {
        let value = self.read_immediate_byte();
        let address = M::B::from(IO_BASE_REG + value as u16);
        let value = self.memory.read_byte(address);

        self.registers.write_byte(REG_A, value);
      }

      /*
        ***************
        * 16-bit Load *
        ***************
      */

      Opcode::LoadImmIntoBc /* 0x01 */ => { self.load_immediate_word(REG_BC); }

      Opcode::LoadImmIntoDe /* 0x11 */ => { self.load_immediate_word(REG_DE); }

      Opcode::LoadImmIntoHl /* 0x21 */ => { self.load_immediate_word(REG_HL); }

      Opcode::LoadImmIntoSp /* 0x31 */ => {
        let address = self.read_immediate_word();

        self.registers.set_stack_pointer(address);
      }

      Opcode::LoadSpIntoImmAddr /* 0x08 */ => {
        let value = self.read_immediate_word();
        let address = M::W::from(value);
        let value = self.registers.get_stack_pointer();

        self.memory.write_word(address, value);
      }

      Opcode::PopBc /* 0xC1 */ => {
        let value = self.stack_pop();

        self.registers.write_word(REG_BC, value);
      }

      Opcode::PushBc /* 0xC5 */ => {
        let bc = self.registers.read_word(REG_BC);

        self.stack_push(bc);
      }

      Opcode::IncrementB /* 0x04 */ => { self.registers.increment_byte(REG_B); }

      Opcode::IncrementC /* 0x0C */ => { self.registers.increment_byte(REG_C); }

      Opcode::DecrementB /* 0x05 */ => { self.registers.decrement_byte(REG_B); }

      Opcode::DecrementC /* 0x0D */ => { self.registers.decrement_byte(REG_C); }

      Opcode::DecrementA /* 0x3D */ => { self.registers.decrement_byte(REG_A); }

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

      /*
        ***************
        * 16-bit Math *
        ***************
      */

      Opcode::IncrementDe /* 0x13 */ => { self.registers.increment_word(REG_DE); }

      Opcode::IncrementHl /* 0x23 */ => { self.registers.increment_word(REG_HL); }

      /*
        **********************
        * 8-bit Shift/Rotate *
        **********************
      */

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
      /*
        **********************
        * 8-bit Shift/Rotate *
        **********************
      */

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

  fn shift_left(&mut self, value: u8, lsb: bool) -> u8 {
    let carry = (value & 0x80) != 0;
    let mut result = value << 1;

    if lsb { result |= 1; }

    self.registers.set_flag(ZERO_FLAG, result == 0);
    self.registers.set_flag(CARRY_FLAG, carry);

    result
  }

  fn subtract(&mut self, a: u8, b: u8) -> u8 {
    let result = a.wrapping_sub(b);

    self.registers.set_flag(SUBTRACT_FLAG, true);
    self.registers.set_flag(HALF_CARRY_FLAG, (a & 0xF) < (b & 0xF));
    self.registers.set_flag(CARRY_FLAG, (a & 0xFF) < (b & 0xff));
    self.registers.set_flag(ZERO_FLAG, result == 0);

    result
  }

  fn load_immediate_byte(&mut self, register: ByteRegister) {
    let value = self.read_immediate_byte();

    self.registers.write_byte(register, value);
  }

  fn load_immediate_word(&mut self, register: WordRegister) {
    let value = self.read_immediate_word();

    self.registers.write_word(register, value);
  }

  fn transfer_to_immediate_address(&mut self, from: ByteRegister) {
    let address = M::B::from(self.read_immediate_word());
    let value = self.registers.read_byte(from);

    self.memory.write_byte(address, value);
  }

  fn transfer_to_address(&mut self, from: ByteRegister, to: WordRegister) {
    let value = self.registers.read_byte(from);
    let address = M::B::from(self.registers.read_word(to));

    self.memory.write_byte(address, value);
  }

  fn transfer_from_address(&mut self, from: WordRegister, to: ByteRegister) {
    let address = M::B::from(self.registers.read_word(from));
    let value = self.memory.read_byte(address);

    self.registers.write_byte(to, value);
  }

  fn jump_conditionally(&mut self, flag: u8, condition: bool) {
    let value = self.read_immediate_word();

    if self.registers.get_flag(flag) == condition {
      self.registers.set_program_counter(value);
    }
  }

  fn jump_relative_conditionally(&mut self, flag: u8, condition: bool) {
    let offset = self.read_immediate_byte();

    if self.registers.get_flag(flag) == condition {
      self.registers.increment_program_counter((offset as i8) as i16);
    }
  }
}

impl<M: Memory> fmt::Debug for Processor<M> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CPU Registers\n{:?}", self.registers)
  }
}
