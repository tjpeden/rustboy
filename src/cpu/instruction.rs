use std::fmt;
use super::opcode::*;

use num::FromPrimitive;

#[derive(Clone, Copy)]
pub struct Instruction(pub u8);

impl Instruction {
  pub fn opcode(&self) -> Opcode {
    Opcode::from_u8(self.0).unwrap_or_else(|| panic!("Unrecognized instruction: {:#06x}", self.0))
  }
}

impl fmt::Debug for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:#06x} {:?}", self.0, self.opcode())
  }
}

#[derive(Clone, Copy)]
pub struct SpecialInstruction(pub u8);

impl SpecialInstruction {
  pub fn opcode(&self) -> SpecialOpcode {
    SpecialOpcode::from_u8(self.0).unwrap_or_else(|| panic!("Unrecognized special instruction: {:#06x}", self.0))
  }
}

impl fmt::Debug for SpecialInstruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:#06x} {:?}", self.0, self.opcode())
  }
}
