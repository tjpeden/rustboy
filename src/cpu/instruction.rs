use std::fmt;
use super::opcode::Opcode;

use num::FromPrimitive;

pub struct Instruction(pub u8);

impl Instruction {
  pub fn opcode(&self) -> Opcode {
    Opcode::from_u8(self.0).unwrap_or_else(|| panic!("Unrecognized instruction: {:#06x}", self.0))
  }
}

impl fmt::Debug for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:#06x}", self.0)
  }
}
