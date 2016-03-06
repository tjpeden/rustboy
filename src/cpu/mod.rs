mod opcode;
mod registers;
mod instruction;

use super::memory::Memory;

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

    let instruction = Instruction(self.memory.read_byte(M::B::from(pc)));

    println!("PC: {:#06x}: {:?}", pc, instruction);

    self.registers.increment_pc_byte();

    instruction
  }

  fn read_special_instruction(&mut self) -> SpecialInstruction {
    let pc = self.registers.read_pc();

    let special_instruction = SpecialInstruction(self.memory.read_byte(M::B::from(pc)));

    println!("PC: {:#06x}: {:?}", pc, special_instruction);

    self.registers.increment_pc_byte();

    special_instruction
  }

  fn read_immediate_byte(&mut self) -> u8 {
    let pc = self.registers.read_pc();
    let immediate = self.memory.read_byte(M::B::from(pc));

    self.registers.increment_pc_byte();

    immediate
  }

  fn read_immediate_word(&mut self) -> u16 {
    let pc = self.registers.read_pc();
    let immediate = self.memory.read_word(M::W::from(pc));

    self.registers.increment_pc_word();

    immediate
  }

  fn execute_instruction(&mut self, instruction: Instruction) {
    match instruction.opcode() {
      Opcode::LoadHL => {
        let immediate = self.read_immediate_word();

        self.registers.write_word(REG_HL, immediate);
      }

      Opcode::LoadSP =>
      {
        // Ignore SP commands
        let _ = self.read_immediate_word();
      }

      Opcode::XorA => {
        let a = self.registers.read_byte(REG_A);
        self.registers.write_byte(REG_A, a ^ a);
      }

      Opcode::LoadHLDecA => {
        let address = self.registers.read_word(REG_HL);
        let value = self.memory.read_byte(M::B::from(address));

        self.registers.write_byte(REG_A, value);
        self.registers.write_word(REG_HL, address - 1);
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
        
      }
    }
  }
}
