mod opcode;
mod registers;
mod instruction;

use super::memory::Memory;

use self::opcode::*;
use self::registers::*;
use self::instruction::*;

const FLAG_Z: u8 = 0x80;
const FLAG_N: u8 = 0x40;
const FLAG_H: u8 = 0x20;
const FLAG_C: u8 = 0x10;

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
        let f = self.registers.read_byte(REG_F);
        let offset = self.read_immediate_byte();

        if (f & FLAG_Z) != FLAG_Z {
          self.registers.increment_pc((offset as i8) as i16);
        }
      }

      Opcode::LoadC => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_C, value);
      }

      Opcode::LoadA => {
        let value = self.read_immediate_byte();

        self.registers.write_byte(REG_A, value);
      }

      Opcode::LoadDecHlA => {
        let hl = self.registers.read_word(REG_HL);
        let address = M::B::from(hl);
        let value = self.memory.read_byte(address);

        self.registers.write_byte(REG_A, value);
        self.registers.write_word(REG_HL, hl - 1);
      }

      Opcode::LoadHl => {
        let immediate = self.read_immediate_word();

        self.registers.write_word(REG_HL, immediate);
      }

      Opcode::LoadSp =>
      {
        // Ignore SP commands
        self.registers.increment_pc(2);
      }

      Opcode::XorA => {
        let a = self.registers.read_byte(REG_A);
        self.registers.write_byte(REG_A, a ^ a);
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
        let f = self.registers.read_byte(REG_F);
        let h = self.registers.read_byte(REG_H);
        let c = f & FLAG_C;
        let z = FLAG_Z - (h & FLAG_Z);

        self.registers.write_byte(REG_F, z + FLAG_H + c); // => 0bz01c0000
      }
    }
  }
}
