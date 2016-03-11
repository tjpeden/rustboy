mod opcode;
mod registers;
mod instruction;

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
    let pc = self.registers.read_pc();
    let address = M::B::from(pc);

    let instruction = Instruction(self.memory.read_byte(address));

    println!("PC: {:#06x}: {:?}", pc, instruction);

    self.registers.increment_pc(1);

    instruction
  }

  fn read_special_instruction(&mut self) -> SpecialInstruction {
    let pc = self.registers.read_pc();
    let address = M::B::from(pc);

    let special_instruction = SpecialInstruction(self.memory.read_byte(address));

    println!("PC: {:#06x}: {:?}", pc, special_instruction);

    self.registers.increment_pc(1);

    special_instruction
  }

  fn read_immediate_byte(&mut self) -> u8 {
    let pc = self.registers.read_pc();
    let address = M::B::from(pc);
    let immediate = self.memory.read_byte(address);

    self.registers.increment_pc(1);

    immediate
  }

  fn read_immediate_word(&mut self) -> u16 {
    let pc = self.registers.read_pc();
    let address = M::W::from(pc);
    let immediate = self.memory.read_word(address);

    self.registers.increment_pc(2);

    immediate
  }

  fn execute_instruction(&mut self, instruction: Instruction) {
    match instruction.opcode() {
      Opcode::JumpNZ => {
        let offset = self.read_immediate_byte();

        if !self.registers.get_flag(ZERO) {
          self.registers.increment_pc((offset as i8) as i16);
        }
      }

      Opcode::LoadAddrDeIntoA => {
        let de = self.registers.read_word(REG_DE);
        let address = M::B::from(de);
        let value = self.memory.read_byte(address);

        self.registers.write_byte(REG_A, value);
      }

      Opcode::LoadImmIntoC => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_C, value);
      }

      Opcode::LoadImmIntoA => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_A, value);
      }

      Opcode::LoadAIntoHlAndDec => {
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);
        let value = self.memory.read_byte(address);

        self.registers.write_byte(REG_A, value);
        self.registers.write_word(REG_HL, hl - 1);
      }

      Opcode::LoadAIntoAddrC => {
        let a = self.registers.read_byte(REG_A);
        let c = self.registers.read_byte(REG_C);
        let address = M::B::from(IO_BASE_REG + c as u16);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAIntoAddrHl => {
        let a = self.registers.read_byte(REG_A);
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);

        self.memory.write_byte(address, a);
      }

      Opcode::LoadAIntoAddrImm => {
        let a = self.registers.read_byte(REG_A);
        let value = self.read_immediate_byte();
        let address = M::B::from(IO_BASE_REG + value as u16);

        self.memory.write_byte(address, a);
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
        // Ignore SP commands
        self.registers.increment_pc(2);
      }

      Opcode::IncrementC => {
        let c = self.registers.read_byte(REG_C);
        let value = c + 1;

        self.registers.set_flag(ZERO, value == 0);
        self.registers.set_flag(SUBTRACT, false);
        self.registers.set_flag(HALF_CARRY, ((c & 0xF) + 1) & 0x10 == 0x10);
        self.registers.write_byte(REG_C, value);
      }

      Opcode::XorA => {
        let a = self.registers.read_byte(REG_A);
        let value = a ^ a;

        self.registers.set_flag(ZERO, value == 0);
        self.registers.write_byte(REG_A, value);
      }

      Opcode::Special => {
        let special_instruction = self.read_special_instruction();

        self.execute_special_instruction(special_instruction);
      }
    }
  }

  fn execute_special_instruction(&mut self, special_instruction: SpecialInstruction) {
    match special_instruction.opcode() {
      SpecialOpcode::Bit7H => {
        let h = self.registers.read_byte(REG_H);

        self.registers.set_flag(ZERO, (h & 0x80) == 0);
        self.registers.set_flag(SUBTRACT, false);
        self.registers.set_flag(HALF_CARRY, true);
      }
    }
  }
}
