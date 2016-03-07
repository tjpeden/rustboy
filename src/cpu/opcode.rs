enum_from_primitive! {
  #[derive(Debug)]
  pub enum Opcode {
    // Jumps
    JumpNZ = 0x20,

    // 8-bit load
    LoadC = 0x0E,
    LoadA = 0x3E,
    LoadDecHlA = 0x32,

    // 16-bit load
    LoadHl = 0x21,
    LoadSp = 0x31,

    // 8-bit math
    XorA = 0xAF,

    Special = 0xCB,
  }
}

enum_from_primitive! {
  #[derive(Debug)]
  pub enum SpecialOpcode {
    Bit7H = 0x7C,
  }
}
