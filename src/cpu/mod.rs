mod opcode;
mod instruction;

use super::memory::Memory;

use self::opcode::*;
use self::instruction::Instruction;

const NUM_GPR: usize = 8;

// struct Registers<M: Memory> {
//   value: [u8; NUM_GPR],
// }
//
// impl<M: Memory> Registers<M> {
//   pub fn new() -> Registers<M> {
//     Registers {
//       value: [0; NUM_GPR],
//     }
//   }
// }

pub struct Cpu<M: Memory> {
  // registers: Registers, // General Purpose Registers
  pc: u16, // Program Counter

  memory: M,
}

impl<M: Memory> Cpu<M> {
  pub fn new(memory: M) -> Cpu<M> {
    Cpu {
      // registers: [0; NUM_GPR],
      pc: 0,

      memory: memory,
    }
  }

  pub fn run(&mut self) {
    loop {
      self.execute_instruction()
    }
  }

  fn execute_instruction(&mut self) {
    let pc = self.pc;
    let instruction = self.read_instruction(pc);

    println!("PC: {:#06x}: {:?}", self.pc, instruction);

    self.pc += 1;
    self.interpret_instruction(instruction);
  }

  fn read_instruction(&mut self, address: u16) -> Instruction {
    Instruction(self.memory.read_byte(address))
  }

  fn interpret_instruction(&mut self, instruction: Instruction) {
    match instruction.opcode() {
      Opcode::LoadSPI16 => {
        // Ignore SP commands
        self.pc += 2;
      },
      Opcode::XorA => {
        // let a = self.registers[Register::A];
        // self.registers[Register::A] = a ^ a;
      }
    }
  }
}
