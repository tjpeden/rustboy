mod opcode;
mod registers;
mod instruction;

use std::fmt;

use super::memory::{Memory, IO_BASE_REG};

use self::opcode::*;
use self::registers::*;
use self::instruction::*;

pub struct Cpu<M: Memory> {
  registers: Registers, // General Purpose Registers

  memory: M,
}

impl<M: Memory> Cpu<M> {
  pub fn new(memory: M) -> Cpu<M> {
    Cpu {
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
      Opcode::JumpRelative => {
        let offset = self.read_immediate_byte();

        self.registers.increment_program_counter((offset as i8) as i16);
      }

      Opcode::JumpRelativeNZ => {
        let offset = self.read_immediate_byte();

        if !self.registers.get_flag(ZERO_FLAG) {
          self.registers.increment_program_counter((offset as i8) as i16);
        }
      }

      Opcode::JumpRelativeZ => {
        let offset = self.read_immediate_byte();

        if self.registers.get_flag(ZERO_FLAG) {
          self.registers.increment_program_counter((offset as i8) as i16);
        }
      }

      Opcode::CallAddrImm => {
        let value = self.read_immediate_word();
        let pc = self.registers.get_program_counter();

        self.stack_push(pc);
        self.registers.set_program_counter(value);
      }

      Opcode::Return => {
        let value = self.stack_pop();

        self.registers.set_program_counter(value);
      }

      Opcode::LoadEIntoA => {
        let e = self.registers.read_byte(REG_E);

        self.registers.write_byte(REG_A, e);
      }

      Opcode::LoadAIntoC => {
        let a = self.registers.read_byte(REG_A);

        self.registers.write_byte(REG_C, a);
      }

      Opcode::LoadAIntoD => {
        let a = self.registers.read_byte(REG_A);

        self.registers.write_byte(REG_D, a);
      }

      Opcode::LoadAIntoH => {
        let a = self.registers.read_byte(REG_A);

        self.registers.write_byte(REG_H, a);
      }

      Opcode::LoadAddrDeIntoA => {
        let de = self.registers.read_word(REG_DE);
        let address = M::B::from(de);
        let value = self.memory.read_byte(address);

        self.registers.write_byte(REG_A, value);
      }

      Opcode::LoadImmIntoA => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_A, value);
      }

      Opcode::LoadImmIntoB => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_B, value);
      }

      Opcode::LoadImmIntoC => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_C, value);
      }

      Opcode::LoadImmIntoE => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_E, value);
      }

      Opcode::LoadImmIntoL => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_L, value);
      }

      Opcode::LoadAintoAddrHlAndInc => {
        let a = self.registers.read_byte(REG_A);
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);

        self.memory.write_byte(address, a);
        self.registers.increment_word(REG_HL);
      }

      Opcode::LoadAIntoAddrHlAndDec => {
        let a = self.registers.read_byte(REG_A);
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);

        self.memory.write_byte(address, a);
        self.registers.decrement_word(REG_HL);
      }

      Opcode::LoadAIntoAddrHl => {
        let a = self.registers.read_byte(REG_A);
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAIntoAddrC => {
        let a = self.registers.read_byte(REG_A);
        let c = self.registers.read_byte(REG_C);
        let address = M::B::from(IO_BASE_REG + c as u16);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAIntoAddrImm => {
        let a = self.registers.read_byte(REG_A);
        let value = self.read_immediate_word();
        let address = M::B::from(value);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAIntoAddrImmIO => {
        let a = self.registers.read_byte(REG_A);
        let value = self.read_immediate_byte();
        let address = M::B::from(IO_BASE_REG + value as u16);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAddrImmIOIntoA => {
        let value = self.read_immediate_byte();
        let address = M::B::from(IO_BASE_REG + value as u16);
        let value = self.memory.read_byte(address);

        self.registers.write_byte(REG_A, value);
      }

      Opcode::LoadImmIntoDe => {
        let immediate = self.read_immediate_word();

        self.registers.write_word(REG_DE, immediate);
      }

      Opcode::LoadImmIntoHl => {
        let immediate = self.read_immediate_word();

        self.registers.write_word(REG_HL, immediate);
      }

      Opcode::LoadImmIntoSp =>
      {
        let address = self.read_immediate_word();

        self.registers.set_stack_pointer(address);
      }

      Opcode::PushBc => {
        let bc = self.registers.read_word(REG_BC);

        self.stack_push(bc);
      }

      Opcode::PopBc => {
        let value = self.stack_pop();

        self.registers.write_word(REG_BC, value);
      }

      Opcode::DecrementA => {
        self.registers.decrement_byte(REG_A);
      }

      Opcode::DecrementB => {
        self.registers.decrement_byte(REG_B);
      }

      Opcode::DecrementC => {
        self.registers.decrement_byte(REG_C);
      }

      Opcode::IncrementB => {
        self.registers.increment_byte(REG_B);
      }

      Opcode::IncrementC => {
        self.registers.increment_byte(REG_C);
      }

      Opcode::SubtractL => {
        let a = self.registers.read_byte(REG_A);
        let l = self.registers.read_byte(REG_L);
        let result = self.subtract(a, l);

        self.registers.write_byte(REG_A, result);
      }

      Opcode::XorA => {
        let a = self.registers.read_byte(REG_A);
        let result = a ^ a;

        self.registers.set_flag(ZERO_FLAG, result == 0);
        self.registers.write_byte(REG_A, result);
      }

      Opcode::CompareImm => {
        let a = self.registers.read_byte(REG_A);
        let value = self.read_immediate_byte();
        let _ = self.subtract(a, value);
      }

      Opcode::IncrementDe => {
        self.registers.increment_word(REG_DE);
      }

      Opcode::IncrementHl => {
        self.registers.increment_word(REG_HL);
      }

      Opcode::Special => {
        let special_instruction = self.read_special_instruction();

        self.execute_special_instruction(special_instruction);
      }

      Opcode::RotateLeftA => {
        let a = self.registers.read_byte(REG_A);
        let carry = self.registers.get_flag(CARRY_FLAG);
        let value = self.shift_left(a, carry);

        self.registers.write_byte(REG_A, value);
      }
    }
  }

  fn execute_special_instruction(&mut self, special_instruction: SpecialInstruction) {
    match special_instruction.opcode() {
      SpecialOpcode::Bit7H => {
        let h = self.registers.read_byte(REG_H);

        self.registers.set_flag(ZERO_FLAG, (h & 0x80) == 0);
        self.registers.set_flag(SUBTRACT_FLAG, false);
        self.registers.set_flag(HALF_CARRY_FLAG, true);
      }

      SpecialOpcode::RotateLeftC => {
        let c = self.registers.read_byte(REG_C);
        let carry = self.registers.get_flag(CARRY_FLAG);
        let value = self.shift_left(c, carry);

        self.registers.write_byte(REG_C, value);
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

  fn subtract(&mut self, a: u8, b: u8) -> u8 {
    let result = a.wrapping_sub(b);

    self.registers.set_flag(SUBTRACT_FLAG, true);
    self.registers.set_flag(HALF_CARRY_FLAG, (a & 0xF) < (b & 0xF));
    self.registers.set_flag(CARRY_FLAG, (a & 0xFF) < (b & 0xff));
    self.registers.set_flag(ZERO_FLAG, result == 0);

    result
  }
}

impl<M: Memory> fmt::Debug for Cpu<M> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CPU Registers\n{:?}", self.registers)
  }
}
